#!/bin/bash

REPO_ROOT=$(git rev-parse --show-toplevel)
FOLDER_NAME=${1:-$(read -p "Enter the name of the new day folder (e.g., day02): " && echo "$REPLY")}
FOLDER_SNAKE=${FOLDER_NAME//day/day_}
FOLDER_PATH=${REPO_ROOT}/src/${FOLDER_NAME}

cd "$REPO_ROOT" || exit 1

if [ -d "$FOLDER_PATH" ]; then 
	echo "$FOLDER_NAME already exists - Did you put the wrong day-tag?"
	echo "No change was done - in order to fully reset an existing day remove the folder and rerun the script..."
	exit 1
fi

mkdir -p "$FOLDER_PATH"
touch "$FOLDER_PATH/input.txt"

cat > "$FOLDER_PATH/part1.rs" <<EOF
#![allow(unused_variables, dead_code)]
pub fn solve_${FOLDER_SNAKE}_part_01(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_01() {
        let input = read_string("./src/${FOLDER_NAME}/input.txt").unwrap();

        let solution = solve_${FOLDER_SNAKE}_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_01_sample() {
        let input = "".trim();

        assert_eq!(0, solve_${FOLDER_SNAKE}_part_01(input));
    }
}
EOF

cat > "$FOLDER_PATH/part2.rs" <<EOF
#![allow(unused_variables, dead_code)]
pub fn solve_${FOLDER_SNAKE}_part_02(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_02() {
        let input = read_string("./src/${FOLDER_NAME}/input.txt").unwrap();

        let solution = solve_${FOLDER_SNAKE}_part_02(&input);

        println!("{solution}");
    }


    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_02_sample() {
        let input = "".trim();

        assert_eq!(0, solve_${FOLDER_SNAKE}_part_02(input));
    }
}
EOF

cat > "$FOLDER_PATH/mod.rs" <<EOF
pub mod part1;
pub mod part2;
EOF

echo "pub mod $FOLDER_NAME;" >> $REPO_ROOT/src/lib.rs

echo "Created new day directory: $FOLDER_PATH"
