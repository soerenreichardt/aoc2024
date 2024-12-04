use winnow::Parser;

pub fn part1() {
    let mut input = std::str::from_utf8(include_bytes!("../../res/day4/part1")).unwrap();
    println!("{}", word_search(&mut input));
}

pub fn part2() {
    let mut input = std::str::from_utf8(include_bytes!("../../res/day4/part1")).unwrap();
    println!("{}", x_search(&mut input));
}

struct Board<'a> {
    board: Vec<Vec<&'a str>>,
    width: usize,
    height: usize,
}

struct XmasKernel {
    size: usize,
    target: String
}

struct XKernel {
    size: usize,
}

impl XKernel {
    pub(crate) fn search(&self, board: &[Vec<&str>], x: usize, y: usize) -> bool {
        if board[y][x] != "A" {
            return false;
        }
        
        if ((board[y-1][x-1] == "M" && board[y+1][x+1] == "S") || (board[y-1][x-1] == "S" && board[y+1][x+1] == "M")) &&
            ((board[y+1][x-1] == "M" && board[y-1][x+1] == "S") || (board[y+1][x-1] == "S" && board[y-1][x+1] == "M")) {
            return true;
        }
        
        false
    }
}

impl XmasKernel {
    fn new (target: String) -> Self {
        XmasKernel {
            size: target.len(),
            target
        }
    }
}

fn word_search(input: &str) -> u32 {
    let board = Board::parse(input);
    board.accept(XmasKernel::new("XMAS".to_string()))
}

fn x_search(input: &str) -> u32 {
    let board = Board::parse(input);
    board.accept2(XKernel { size: 3 })
}

impl<'a> Board<'a> {
    fn parse(input: &'a str) -> Self {
        let board = input
            .lines()
            .map(|line| (0..line.len()).map(|index| &line[index..index+1]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = board[0].len();
        let height = board.len();

        Board { board, width, height }
    }

    fn accept2(&self, kernel: XKernel) -> u32 {
        let mut counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if x > 0 && x < self.width - 1 && y > 0 && y < self.height -1 {
                    if kernel.search(&self.board, x, y) {
                        counter += 1;
                    }
                }
            }
        }
        
        counter
    }

    fn accept(&self, kernel: XmasKernel) -> u32 {
        let mut counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let enable_row_check = x < self.width - kernel.size + 1;
                let enable_column_check = y < self.height - kernel.size + 1;
                if enable_row_check {
                    if kernel.search_row(&self.board, x, y) {
                        counter += 1;
                    }
                }
                if enable_column_check {
                    if kernel.search_column(&self.board, x, y) {
                        counter += 1;
                    }
                }
                if enable_row_check && enable_column_check {
                    if kernel.search_diagonal_right(&self.board, x, y) {
                        counter += 1;
                    }
                    if kernel.search_diagonal_left(&self.board, x + kernel.size - 1, y) {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }
}

impl XmasKernel {
    fn search_row(&self, board: &[Vec<&str>], x_offset: usize, y_offset: usize) -> bool {
        let row = self.row_slice(board, x_offset, y_offset);
        if self.check_slice(row) {
            return true;
        }
        false
    }

    fn row_slice<'a>(&self, board: &'a [Vec<&'a str>], x_offset: usize, y_offset: usize) -> &'a [&'a str] {
        &board[y_offset][x_offset..x_offset+self.size]
    }

    fn search_column(&self, board: &[Vec<&str>], x_offset: usize, y_offset: usize) -> bool {
        let column = self.column_slice(board, x_offset, y_offset);
        if self.check_slice(&column) {
            return true;
        }
        false
    }

    fn column_slice<'a>(&self, board: &'a [Vec<&'a str>], x_offset: usize, y_offset: usize) -> Vec<&'a str> {
        let mut slice = Vec::new();
        for row in y_offset..y_offset + self.size {
            slice.push(board[row][x_offset]);
        }
        slice
    }

    fn search_diagonal_right(&self, board: &[Vec<&str>], x_offset: usize, y_offset: usize) -> bool {
        let diagonal = self.diagonal_slice_right(board, x_offset, y_offset);
        if self.check_slice(&diagonal) {
            return true;
        }
        false
    }

    fn diagonal_slice_right<'a>(&self, board: &'a [Vec<&'a str>], x_offset: usize, y_offset: usize) -> Vec<&'a str> {
        let mut slice = Vec::new();
        for i in 0..self.size {
            slice.push(board[y_offset + i][x_offset + i]);
        }
        slice
    }

    fn search_diagonal_left(&self, board: &[Vec<&str>], x_offset: usize, y_offset: usize) -> bool {
        let diagonal = self.diagonal_slice_left(board, x_offset, y_offset);
        self.check_slice(&diagonal)
    }

    fn diagonal_slice_left<'a>(&self, board: &'a [Vec<&'a str>], x_offset: usize, y_offset: usize) -> Vec<&'a str> {
        let mut slice = Vec::new();
        for i in 0..self.size {
            slice.push(board[y_offset + i][x_offset - i]);
        }
        slice
    }

    fn check_slice(&self, slice: &[&str]) -> bool {
        match slice {
            ["X", "M", "A", "S"] => true,
            ["S", "A", "M", "X"] => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::{word_search, x_search};

    #[test]
    fn part1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
        assert_eq!(18, word_search(input));
    }

    #[test]
    fn part2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
        assert_eq!(9, x_search(input));
    }
}