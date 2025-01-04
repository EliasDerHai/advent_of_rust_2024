#!/bin/bash

SRC_DIR="../src"

FOLDER_NAME=${1:-$(read -p "Enter the name of the new day folder (e.g., day02): " && echo $REPLY)}

FOLDER_SNAKE=${FOLDER_NAME//day/day_}

mkdir -p "$SRC_DIR/$FOLDER_NAME"
touch "$SRC_DIR/$FOLDER_NAME/input.txt"

cat > "$SRC_DIR/$FOLDER_NAME/mod.rs" <<EOF
pub fn solve_${FOLDER_SNAKE}_part_01(input: Vec<String>) -> u32 {
    todo!()
}

pub fn solve_${FOLDER_SNAKE}_part_02(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::read_lines;
    use super::*;

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_01() {
        todo!()
    }

    #[test]
    fn should_solve_${FOLDER_SNAKE}_part_02() {
        todo!()
    }
}
EOF

echo "Created new day directory: $SRC_DIR/$FOLDER_NAME"
