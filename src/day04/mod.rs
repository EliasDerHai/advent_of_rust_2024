pub fn solve_day_04_part_01(original: String) -> u32 {
    let mut xmas_count = 0;
    let search_word = "XMAS";
    let reverse_search_word: &str = &search_word.chars().rev().collect::<String>();

    let line_count = original.lines().count();
    let col_count = original.lines().next().unwrap().chars().filter(|c| c.is_alphabetic()).count();
    let char_lookup: Vec<char> = original.trim().chars().filter(|c| c.is_alphabetic()).collect();
    let max_lookup = char_lookup.len();

    println!("rows {line_count} cols {col_count} -> {}/{max_lookup}", line_count * col_count);
    println!("{:?}", char_lookup);

    {
        // find all occurrences of 'XMAS' from left to right
        {
            let mut slice = original.as_str();
            let mut next = slice.find(search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + search_word.len()..];
                next = slice.find(search_word);
            }
        }
        println!("{xmas_count}");

        // find all occurrences of 'XMAS' from right to left
        {
            let mut slice = original.as_str();
            let mut next = slice.find(reverse_search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + reverse_search_word.len()..];
                next = slice.find(reverse_search_word);
            }
        }
        println!("{xmas_count}");
    }

    {
        // rotate the matrix (rows <-> cols)
        let mut transformation_top_bottom = String::new();

        for col in 0..col_count {
            for row in 0..line_count {
                let char_index = col + (row * col_count);
                let char = get_char_from_original_input(&char_lookup, char_index, max_lookup);
                // println!("grabbing: {char_index} = {char}");
                transformation_top_bottom.push(char);
            }
            transformation_top_bottom.push('\n');
        }

        {
            let mut slice = transformation_top_bottom.as_str();
            let mut next = slice.find(search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + search_word.len()..];
                next = slice.find(search_word);
            }
        }
        println!("{xmas_count}");

        {
            let mut slice = transformation_top_bottom.as_str();
            let mut next = slice.find(reverse_search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + reverse_search_word.len()..];
                next = slice.find(reverse_search_word);
            }
        }
        println!("{xmas_count}");
    }

    {
        // rotate the matrix (diagonals)
        let mut transformation_diagonal_one = String::new();

        for offset in 0..col_count {
            for col in 0..col_count {
                let x_coord = col + offset;
                if x_coord >= col_count { break; } // ~ out of bounds
                let y_coord = col;
                if y_coord >= line_count { break; } // ~ out of bounds

                let char_index = x_coord + y_coord * col_count;
                println!("x: {x_coord} - y: {y_coord} -> {char_index}");
                let char = get_char_from_original_input(&char_lookup, char_index, max_lookup);
                transformation_diagonal_one.push(char);
            }

            transformation_diagonal_one.push('\n');
        }

        for offset in 1..line_count { // start at one to skip first diagonal (which is already transformed)
            for row in 0..line_count {
                let x_coord = row;
                if x_coord >= col_count { break; } // ~ out of bounds
                let y_coord = row + offset;
                if y_coord >= line_count { break; } // ~ out of bounds

                let char_index = x_coord + y_coord * col_count;
                println!("x: {x_coord} - y: {y_coord} -> {char_index}");
                let char = get_char_from_original_input(&char_lookup, char_index, max_lookup);
                transformation_diagonal_one.push(char);
            }

            transformation_diagonal_one.push('\n');
        }

        {
            let mut slice = transformation_diagonal_one.as_str();
            let mut next = slice.find(search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + search_word.len()..];
                next = slice.find(search_word);
            }
        }
        println!("{xmas_count}");

        {
            let mut slice = transformation_diagonal_one.as_str();
            let mut next = slice.find(reverse_search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + reverse_search_word.len()..];
                next = slice.find(reverse_search_word);
            }
        }
        println!("{xmas_count}");

        println!("---------------------------");
        // println!("{transformation_diagonal_one}");
    }


    {
        // rotate the matrix (diagonals)
        let mut transformation_diagonal_two = String::new();

        for offset in 0..col_count {
            for col in 0..col_count {
                let x_coord = col + offset;
                if x_coord >= col_count { break; } // ~ out of bounds
                let y_coord: i32 = col_count as i32 - col as i32 - 1;
                if y_coord >= line_count as i32 || y_coord < 0 { break; } // ~ out of bounds

                let char_index = x_coord + y_coord as usize * col_count;
                // println!("x: {x_coord} - y: {y_coord} -> {char_index}");
                let char = get_char_from_original_input(&char_lookup, char_index, max_lookup);
                transformation_diagonal_two.push(char);
            }

            transformation_diagonal_two.push('\n');
        }

        println!("---------------------------");
        for offset in 1..line_count { // start at one to skip first diagonal (which is already transformed)
            for row in 0..line_count {
                let x_coord = row;
                if x_coord >= col_count { break; } // ~ out of bounds
                let y_coord: i32 = col_count as i32 - row as i32 - 2 + offset as i32;
                if y_coord >= line_count as i32 || y_coord < 0 { break; } // ~ out of bounds

                let char_index = x_coord + y_coord as usize * col_count;
                // println!("x: {x_coord} - y: {y_coord} -> {char_index}");
                let char = get_char_from_original_input(&char_lookup, char_index, max_lookup);
                transformation_diagonal_two.push(char);
            }

            transformation_diagonal_two.push('\n');
        }

        println!("---------------------------");
        println!("{transformation_diagonal_two}");

        {
            let mut slice = transformation_diagonal_two.as_str();
            let mut next = slice.find(search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + search_word.len()..];
                next = slice.find(search_word);
            }
        }
        println!("{xmas_count}");

        {
            let mut slice = transformation_diagonal_two.as_str();
            let mut next = slice.find(reverse_search_word);
            while let Some(i) = next {
                xmas_count += 1;
                slice = &slice[i + reverse_search_word.len()..];
                next = slice.find(reverse_search_word);
            }
        }
        println!("{xmas_count}");
    }

    xmas_count
}

fn get_char_from_original_input(char_lookup: &Vec<char>, char_index: usize, max: usize) -> char {
    *char_lookup.get(char_index)
        .expect(&*format!("Index out of bounds - {char_index} >= {}", max))
}

pub fn solve_day_04_part_02(input: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day04::solve_day_04_part_01;
    use crate::util::read_string;

    #[test]
    fn should_solve_day_04_part_01_sample() {
        let input = "WWXWWW
WSAMXW
WAWWAW
XMASWS
WXWWWW".trim().to_string();

        let actual = solve_day_04_part_01(input);

        assert_eq!(4, actual);
    }

    #[test]
    fn should_solve_day_04_part_01_sample_2() {
        let input = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX".to_string();

        assert_eq!(18, solve_day_04_part_01(input));
    }

    #[test]
    fn should_solve_day_04_part_01_sample_3() {
        let input = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX".to_string();

        assert_eq!(2 * 18, solve_day_04_part_01(input));
    }

    #[test]
    fn should_solve_day_04_part_01() {
        let input = read_string("./src/day04/input.txt")
            .expect("Should have");


        let solution = solve_day_04_part_01(input);

        println!("Solution: {solution}");
    }

    #[test]
    fn should_solve_day_04_part_02() {
        todo!()
    }
}
