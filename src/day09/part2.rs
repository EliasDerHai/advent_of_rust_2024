use std::fmt::{Display, Formatter};
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
struct FreeBlock {
    start: usize,
    length: usize,
}

impl FreeBlock {
    fn new(start: usize, length: usize) -> Self {
        FreeBlock {
            start,
            length,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct File {
    id: usize,
    start: usize,
    length: usize,
}

impl File {
    fn new(id: usize, start: usize, length: usize) -> Self {
        File {
            id,
            start,
            length,
        }
    }
}

#[derive(Debug, Clone)]
enum DiskItem {
    File(File),
    FreeBlock(FreeBlock),
}

impl DiskItem {
    fn get_index(&self) -> usize {
        match self {
            DiskItem::File(f) => f.start,
            DiskItem::FreeBlock(b) => b.start,
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Disk {
    files: Vec<File>,
    free_blocks: Vec<FreeBlock>,
    size: usize,
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut disk = Disk::default();
        disk.size = value.len();
        let mut index_counter = 0;

        value
            .chars()
            .enumerate()
            .for_each(|(idx, c)| {
                let length = c.to_digit(10).unwrap() as usize;
                let is_file = idx % 2 == 0;
                if is_file {
                    disk.files.push(File::new(idx / 2, index_counter, length));
                } else {
                    disk.free_blocks.push(FreeBlock::new(index_counter, length));
                }
                index_counter += length;
            });

        disk
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut items: Vec<DiskItem> = self.files
            .iter().cloned().map(DiskItem::File)
            .chain(self.free_blocks.iter().cloned().map(DiskItem::FreeBlock))
            .collect();

        items.sort_by_key(|item| item.get_index());


        let mut s = String::new();
        for it in items {
            let (char_to_add, length) = match it {
                DiskItem::File(file) => (file.id.to_string().chars().next().unwrap(), file.length),
                DiskItem::FreeBlock(block) => ('.', block.length)
            };
            let new: String = std::iter::repeat(char_to_add).take(length).collect();
            s.push_str(&new);
        }
        write!(f, "{}", s)
    }
}

impl Disk {
    fn defragment(&mut self) -> &mut Self {
        self.files.iter_mut().rev().for_each(|file| {
            let leftmost_free = self.free_blocks
                .iter_mut()
                .find(|free| free.length >= file.length);

            match leftmost_free {
                None => {}
                Some(free) => {
                    // essentially we swap the FreeBlock and File while respecting the case that the FreeBlock is bigger than the file and as such doesn't get fully occupied
                    // while it works for the requirement given, it is not entirely correct, because the previous file should be replaced with a FreeBlock as well
                    if free.start > file.start { return; }
                    let tmp = free.start;
                    free.length -= file.length;
                    free.start += file.length;
                    file.start = tmp;
                }
            }
        });
        self.files.sort_by(|a, b| a.start.cmp(&b.start));
        self.free_blocks = self.free_blocks.clone().into_iter().filter(|block| block.length > 0).collect();
        self.free_blocks.sort_by(|a, b| a.start.cmp(&b.start));
        self
    }

    fn checksum(&self) -> usize {
        self.files
            .iter()
            .map(|file| {
                let mut add = 0;
                for i in 0..file.length {
                    add += (file.start + i) * file.id
                }
                add
            })
            .sum()
    }
}

pub fn solve_day_09_part_02(input: String) -> usize {
    let ins = Instant::now();
    let mut disk = Disk::from(input.as_str());
    println!("{}ms - unzipped: {disk}", ins.elapsed().as_millis());
    let ins = Instant::now();
    disk.defragment();
    println!("{}ms - defragmented: {disk}", ins.elapsed().as_millis());
    let ins = Instant::now();
    let r = disk.checksum();
    println!("{}ms", ins.elapsed().as_millis());
    r
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;

    use super::*;

    #[test]
    fn should_solve_day_09_part_02() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_02(input);

        println!("{solution}");
        assert_eq!(6427437134372, solution);
    }


    #[test]
    fn should_solve_day_09_part_02_sample() {
        let input = "2333133121414131402".trim().to_string();

        assert_eq!(2858, solve_day_09_part_02(input));
    }
}
