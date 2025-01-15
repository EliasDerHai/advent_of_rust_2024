#!/bin/bash

SRC_DIR="../src"

FOLDER_NAME=${1:-$(read -p "Enter the name of the new day folder (e.g., day02): " && echo "$REPLY")}

FOLDER_SNAKE=${FOLDER_NAME//day/day_}

mkdir -p "$SRC_DIR/$FOLDER_NAME"
touch "$SRC_DIR/$FOLDER_NAME/input.txt"

cat > "$SRC_DIR/$FOLDER_NAME/part1.rs" <<EOF
pub fn solve_${FOLDER_SNAKE}_part_01(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_01() {
        let input = read_string("./src/${FOLDER_NAME}/input.txt").unwrap();

        let solution = solve_${FOLDER_SNAKE}_part_01(input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_01_sample() {
        let input = "".trim().to_string();

        assert_eq!(0, solve_${FOLDER_SNAKE}_part_01(input));
    }
}
EOF

cat > "$SRC_DIR/$FOLDER_NAME/part2.rs" <<EOF
pub fn solve_${FOLDER_SNAKE}_part_02(input: String) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_02() {
        let input = read_string("./src/${FOLDER_NAME}/input.txt").unwrap();

        let solution = solve_${FOLDER_SNAKE}_part_02(input);

        println!("{solution}");
    }


    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_02_sample() {
        let input = "".trim().to_string();

        assert_eq!(0, solve_${FOLDER_SNAKE}_part_02(input));
    }
}
EOF

cat > "$SRC_DIR/$FOLDER_NAME/mod.rs" <<EOF
pub mod part1;
pub mod part2;
EOF

echo "Created new day directory: $SRC_DIR/$FOLDER_NAME"
read