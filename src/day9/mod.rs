use std::ops::Range;

pub fn part1() {
    let input = include_str!("../../res/day9/part1");
    println!("{}", fragment_disk(input));
}

struct Cursor<'a> {
    index: usize,
    inner: Range<u32>,
    label: u32,
    increment: i64,
    disk_map: &'a [u32]
}

fn fragment_disk(input: &str) -> u64 {
    let mut input = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut forward_cursor = Cursor::forward(&input);
    let mut backward_cursor = Cursor::backward(&input);

    let mut checksum = 0u64;
    let mut global_position = 0;
    while forward_cursor.index() != backward_cursor.index() {
        while let Some(label) = forward_cursor.next() {
            checksum += (label * global_position) as u64;
            global_position += 1;
        }
        (0..input[forward_cursor.index() - 1]).map(|_| backward_cursor.next_safe()).for_each(|label| {
            checksum += (label * global_position) as u64;
            global_position += 1;
        })
    }

    while let Some(label) = backward_cursor.next() {
        checksum += (label * global_position) as u64;
        global_position += 1;
    }

    checksum
}

impl<'a> Cursor<'a> {
    fn forward(disk_map: &'a [u32]) -> Self {
        let inner = 0..disk_map[0];
        let label = 0;
        Cursor { index: 0 , inner, label, increment: 1, disk_map }
    }

    fn backward(disk_map: &'a [u32]) -> Self {
        let last_file_index = match disk_map.len() {
            len if len % 2 == 0 => len - 2,
            len if len % 2 == 1 => len - 1,
            _ => unreachable!(),
        };
        let inner = 0..disk_map[last_file_index];
        let label = (disk_map.len() / 2) as u32;
        Cursor { index: last_file_index, inner, label, increment: -1, disk_map }
    }

    fn next(&mut self) -> Option<u32> {
        match self.inner.next() {
            Some(_) => Some(self.label),
            None => {
                self.inner = 0..self.disk_map[self.advance()];
                self.label = (self.label as i64 + self.increment) as u32;
                None
            }
        }
    }

    fn next_safe(&mut self) -> u32 {
        match self.next() {
            Some(label) => label,
            None => self.next().unwrap()
        }
    }

    fn index(&self) -> usize {
        self.index
    }

    fn advance(&mut self) -> usize {
        self.index = (self.index as i64 + self.increment * 2) as usize;
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "2333133121414131402";
        assert_eq!(1928, fragment_disk(input));
    }
}
