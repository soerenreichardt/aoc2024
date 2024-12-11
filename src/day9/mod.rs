use std::ops::Range;

pub fn part1() {
    let input = include_str!("../../res/day9/part1");
    println!("{}", fragment_disk(input));
}

pub fn part2() {
    let input = include_str!("../../res/day9/part1");
    println!("{}", move_files(input));
}

struct Cursor<'a> {
    index: usize,
    inner: Range<u32>,
    label: u32,
    increment: i64,
    disk_map: &'a [u32]
}

fn fragment_disk(input: &str) -> u64 {
    let input = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

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

#[derive(Debug)]
enum State {
    File(usize, u32),
    Free(usize)
}

fn move_files(input: &str) -> usize {
    let mut input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, len)| if i % 2 == 0 {
            State::File(len as usize, (i / 2) as u32)
        } else {
            State::Free(len as usize)
        })
        .collect::<Vec<_>>();

    loop {
        let mut changed = false;
        for index in (0..input.len()).rev() {
            let free_slot_with_file = match input[index] {
                State::File(length, label) => {
                    let mut result = None;
                    for i in 0..input.len() {
                        match input[i] {
                            State::Free(len) if len >= length && i < index => {
                                result = Some((i, length, label, len - length));
                                break;
                            },
                            _ => ()
                        };
                    }
                    result
                }
                State::Free(_) => None
            };

            if let Some((free_slot, length, label, remainder)) = free_slot_with_file {
                input[index] = State::Free(length);
                input[free_slot] = State::File(length, label);
                if remainder > 0 {
                    input.insert(free_slot + 1, State::Free(remainder));
                }
                changed = true;
            }

            if changed {
                break;
            }
        }

        if !changed {
            break;
        }
    }

    let mut global_position = 0;
    let mut sum = 0usize;
    for state in input.iter() {
        match state {
            State::Free(length) => {
                global_position += length;
            }
            State::File(length, label) => {
                (0..*length).for_each(|_| {
                    sum += *label as usize * global_position;
                    global_position += 1;
                })
            }
        }
    }

    sum
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

    #[test]
    fn part2() {
        let input = "2333133121414131402";
        assert_eq!(2858, move_files(input));
    }
}

// 00992111777.44.333....5555.6666.....8888..
// [File(2, 0), File(2, 9), File(1, 2), File(3, 1), File(3, 7), Free(1), File(2, 4), Free(1), File(3, 3), Free(1), Free(2), Free(1), File(4, 5), Free(1), File(4, 6), Free(1), Free(3), Free(1), File(4, 8), Free(0), Free(2)]
