use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// copied from:
// https://github.com/jpverkamp/advent-of-code/blob/master/2024/src/day24.rs
// most aoc solutions seem to either A) visualize the tree with graphviz and manually try to spot
// swaps or build upon a deep understanding of ripple carry adder
// ... but I don't want to waste my time with either of this
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Invalid operator: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wire<'input> {
    Input(bool),
    Function(Operator, &'input str, &'input str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine<'input> {
    wires: HashMap<&'input str, Wire<'input>>,
}

impl<'input> From<&'input str> for Machine<'input> {
    fn from(input: &'input str) -> Self {
        let mut wires = HashMap::new();

        for line in input.lines() {
            if line.contains(':') {
                let (key, value) = line.split_once(": ").unwrap();
                let value = value == "1";

                wires.insert(key, Wire::Input(value));
            }

            if line.contains("->") {
                let mut parts = line.split_ascii_whitespace();
                let arg0 = parts.next().unwrap();
                let op = parts.next().unwrap().into();
                let arg1 = parts.next().unwrap();
                parts.next(); // Skip the ->
                let result = parts.next().unwrap();

                wires.insert(result, Wire::Function(op, arg0, arg1));
            }
        }

        Self { wires }
    }
}

impl<'input> Machine<'input> {
    fn wires(&'input self) -> impl Iterator<Item = &'input str> {
        self.wires.keys().copied()
    }

    pub fn wire(&self, key: &str) -> Option<Wire> {
        self.wires.iter().find(|(k, _)| **k == key).map(|(_, v)| *v)
    }

    pub fn wire_name(&self, key: &str) -> &'input str {
        self.wires
            .keys()
            .copied()
            .find(|&wire| wire == key)
            .unwrap()
    }

    pub fn function_for(&self, wire: &'input str) -> String {
        match self.wires[wire] {
            Wire::Input(value) => format!("{wire}={value}"),
            Wire::Function(op, arg0, arg1) => format!(
                "{wire}=({op:?} {} {})",
                self.function_for(arg0),
                self.function_for(arg1)
            ),
        }
    }

    pub fn value_of_prefixed(&'input self, prefix: char) -> usize {
        let mut binary = String::new();

        for wire in self.wires().sorted().rev() {
            if wire.starts_with(prefix) {
                binary.push(if self.value_of(wire) { '1' } else { '0' });
            }
        }

        usize::from_str_radix(&binary, 2).unwrap()
    }

    #[allow(clippy::result_unit_err)]
    pub fn value_of_prefixed_loopcheck(&'input self, prefix: char) -> Result<usize, ()> {
        let mut binary = String::new();

        for wire in self.wires().sorted().rev() {
            if wire.starts_with(prefix) {
                binary.push(if self.value_of_loopcheck(wire)? {
                    '1'
                } else {
                    '0'
                });
            }
        }

        usize::from_str_radix(&binary, 2).map_err(|_e| ())
    }

    pub fn value_of(&self, wire: &'input str) -> bool {
        match self.wires.get(wire).unwrap() {
            Wire::Input(value) => *value,
            Wire::Function(op, arg0, arg1) => {
                let arg0 = self.value_of(arg0);
                let arg1 = self.value_of(arg1);

                match op {
                    Operator::And => arg0 & arg1,
                    Operator::Or => arg0 | arg1,
                    Operator::Xor => arg0 ^ arg1,
                }
            }
        }
    }

    #[allow(clippy::result_unit_err)]
    pub fn value_of_loopcheck(&self, wire: &'input str) -> Result<bool, ()> {
        fn recur(me: &Machine, wire: &str, checked: Vec<&str>) -> Result<bool, ()> {
            if checked.contains(&wire) {
                return Err(());
            }
            let mut next_checked = checked.clone();
            next_checked.push(wire);

            match me.wires.get(wire).unwrap() {
                Wire::Input(value) => Ok(*value),
                Wire::Function(op, arg0, arg1) => {
                    let arg0 = recur(me, arg0, next_checked.clone())?;
                    let arg1 = recur(me, arg1, next_checked.clone())?;

                    Ok(match op {
                        Operator::And => arg0 & arg1,
                        Operator::Or => arg0 | arg1,
                        Operator::Xor => arg0 ^ arg1,
                    })
                }
            }
        }

        recur(self, wire, vec![])
    }

    pub fn swap(&mut self, a: &'input str, b: &'input str) {
        let old_b = self.wires[b];
        let old_a = self.wires[a];

        self.wires.insert(a, old_b);
        self.wires.insert(b, old_a);
    }

    pub fn depends_on(&self, wire: &'input str) -> Vec<&'input str> {
        let mut result = vec![wire];

        'searching: loop {
            for wire in result.clone().iter() {
                match self.wires[wire] {
                    Wire::Input(_) => continue,
                    Wire::Function(_, arg0, arg1) => {
                        let mut changed = false;

                        if !result.contains(&arg0) {
                            result.push(arg0);
                            changed = true;
                        }

                        if !result.contains(&arg1) {
                            result.push(arg1);
                            changed = true;
                        }

                        if changed {
                            continue 'searching;
                        }
                    }
                }
            }

            break;
        }

        result
    }

    fn wire_to_graphviz(&self, wire: &str) -> Vec<String> {
        match self.wires[wire] {
            Wire::Input(value) => vec![format!("    {wire} [label=\"{wire}={value}\"];")],
            Wire::Function(op, arg0, arg1) => vec![
                format!("    {wire} [label=\"{wire}={op:?}\"];"),
                format!("    {wire} -> {arg0};"),
                format!("    {wire} -> {arg1};"),
            ],
        }
    }

    pub fn to_graphviz(&self) -> String {
        self.to_graphviz_limited(45)
    }

    pub fn to_graphviz_limited(&self, limit: usize) -> String {
        let mut dot = String::new();

        dot.push_str("digraph {\n");
        dot.push_str("  compounded=true;\n");
        dot.push_str("  rankdir=LR;\n");

        let mut added = HashSet::new();
        let mut by_output: HashMap<&str, HashSet<&str>> = HashMap::new();

        for output in self
            .wires()
            .filter(|w| w.starts_with('z'))
            .sorted()
            .take(limit)
        {
            let deps = self.depends_on(output);

            for dep in deps {
                if !added.contains(&dep) {
                    added.insert(dep);
                    by_output.entry(output).or_default().insert(dep);
                }
            }
        }

        for (output, deps) in by_output.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            dot.push_str(&format!(
                "  subgraph cluster_{output} {{\n    label=\"{output}\";\n\n",
                output = output
            ));
            for line in self.wire_to_graphviz(output) {
                dot.push_str(&line);
                dot.push('\n');
            }

            for dep in deps.iter().sorted() {
                for line in self.wire_to_graphviz(dep) {
                    dot.push_str(&line);
                    dot.push('\n');
                }
                dot.push('\n');
            }
            dot.push_str("  }\n");
        }

        // External edges have any output in one and a wire in the other

        dot.push_str("}\n");

        dot
    }
}

pub fn solve_day_24_part_02(input: &str) -> String {
    let machine = Machine::from(input);
    let bits = machine.wires().filter(|w| w.starts_with('x')).count();

    fn find_op<'input>(
        machine: &'input Machine,
        op: Operator,
        input1: Option<&'input str>,
        input2: Option<&'input str>,
    ) -> Option<&'input str> {
        if input1.is_none() || input2.is_none() {
            return None;
        }

        for (&output, &wire) in machine.wires.iter() {
            if let Wire::Function(found_op, found_input1, found_input2) = wire {
                if found_op == op
                    && ((found_input1 == input1.unwrap() && found_input2 == input2.unwrap())
                        || (found_input1 == input2.unwrap() && found_input2 == input1.unwrap()))
                {
                    return Some(output);
                }
            }
        }

        None
    }

    let mut carry = None;
    let mut swaps = vec![];

    for bit in 0..bits {
        // New bits we're adding in
        let xin = Some(machine.wire_name(&format!("x{:02}", bit)));
        let yin = Some(machine.wire_name(&format!("y{:02}", bit)));

        // The combinations of just those bits
        let mut adder = find_op(&machine, Operator::Xor, xin, yin);
        let mut next = find_op(&machine, Operator::And, xin, yin);

        // Output should end up being zN and next_carry is the only value passed on
        let mut output = None;
        let mut next_carry = None;

        // Every bit after the first one :smile:
        if carry.is_some() {
            let mut result = find_op(&machine, Operator::And, adder, carry);
            if result.is_none() {
                swaps.push((adder, next));
                std::mem::swap(&mut adder, &mut next);

                result = find_op(&machine, Operator::And, adder, carry);
            }

            // This should be zN
            output = find_op(&machine, Operator::Xor, adder, carry);

            // Check if any of the wires are actually the z bit and swap them
            if adder.is_some_and(|a| a.starts_with('z')) {
                swaps.push((adder, output));
                std::mem::swap(&mut adder, &mut output);
            }

            if next.is_some_and(|a| a.starts_with('z')) {
                swaps.push((next, output));
                std::mem::swap(&mut next, &mut output);
            }

            if result.is_some_and(|a| a.starts_with('z')) {
                swaps.push((result, output));
                std::mem::swap(&mut result, &mut output);
            }

            // Calculate what our next carry will be
            next_carry = find_op(&machine, Operator::Or, next, result);
        }

        // As long as we're not the end, check if the output and carry are swapped
        if bit != (bits - 1) && next_carry.is_some_and(|a| a.starts_with('z')) {
            swaps.push((next_carry, output));
            std::mem::swap(&mut next_carry, &mut output);
        }

        // Pass along the carry to the next chunk of the adder
        carry = if carry.is_some() { next_carry } else { next };
    }

    swaps
        .iter()
        .flat_map(|(a, b)| vec![a.unwrap(), b.unwrap()])
        .sorted()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_24_part_02() {
        let input = read_string("./src/day24/input.txt").unwrap();

        let solution = solve_day_24_part_02(&input);

        assert_eq!("cqk,fph,gds,jrs,wrk,z15,z21,z34", &solution);
    }
}
