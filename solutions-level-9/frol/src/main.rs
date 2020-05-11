use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
struct Position {
    line: usize,
    column: usize,
}

impl From<(usize, usize)> for Position {
    fn from((column, line): (usize, usize)) -> Self {
        Self { column, line }
    }
}

#[derive(Debug, Clone, Copy)]
enum TetrominoKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl std::str::FromStr for TetrominoKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "I" => Self::I,
            "O" => Self::O,
            "T" => Self::T,
            "S" => Self::S,
            "Z" => Self::Z,
            "J" => Self::J,
            "L" => Self::L,
            _ => return Err(()),
        })
    }
}
#[derive(Debug, Clone, Copy)]
struct Tetromino {
    kind: TetrominoKind,
    position: usize,
    rotation: usize,
}

impl std::str::FromStr for Tetromino {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_line = s.trim().split_ascii_whitespace();
        let kind = parsed_line.next().ok_or(())?.parse()?;
        let position = parsed_line.next().ok_or(())?.parse().unwrap();
        Ok(Tetromino {
            kind,
            position,
            rotation: 0,
        })
    }
}

impl Tetromino {
    fn blocks(&self) -> [Position; 4] {
        match self.kind {
            TetrominoKind::I => match self.rotation % 180 {
                0 => [(0, 0).into(), (0, 1).into(), (0, 2).into(), (0, 3).into()],
                _ => [(0, 0).into(), (1, 0).into(), (2, 0).into(), (3, 0).into()],
            },
            TetrominoKind::O => [(0, 0).into(), (1, 0).into(), (0, 1).into(), (1, 1).into()],
            TetrominoKind::T => match self.rotation % 360 {
                0 => [(0, 0).into(), (1, 0).into(), (2, 0).into(), (1, 1).into()],
                90 => [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 1).into()],
                180 => [(0, 1).into(), (1, 1).into(), (2, 1).into(), (1, 0).into()],
                _ => [(0, 0).into(), (0, 1).into(), (0, 2).into(), (1, 1).into()],
            },
            TetrominoKind::S => match self.rotation % 180 {
                0 => [(0, 1).into(), (1, 1).into(), (1, 0).into(), (2, 0).into()],
                _ => [(0, 0).into(), (0, 1).into(), (1, 1).into(), (1, 2).into()],
            },
            TetrominoKind::Z => match self.rotation % 180 {
                0 => [(0, 0).into(), (1, 0).into(), (1, 1).into(), (2, 1).into()],
                _ => [(1, 0).into(), (1, 1).into(), (0, 1).into(), (0, 2).into()],
            },
            TetrominoKind::J => match self.rotation % 360 {
                0 => [(1, 0).into(), (1, 1).into(), (1, 2).into(), (0, 2).into()],
                90 => [(0, 0).into(), (0, 1).into(), (1, 1).into(), (2, 1).into()],
                180 => [(0, 0).into(), (0, 1).into(), (0, 2).into(), (1, 0).into()],
                _ => [(0, 0).into(), (1, 0).into(), (2, 0).into(), (2, 1).into()],
            },
            TetrominoKind::L => match self.rotation % 360 {
                0 => [(0, 0).into(), (0, 1).into(), (0, 2).into(), (1, 2).into()],
                90 => [(0, 0).into(), (1, 0).into(), (2, 0).into(), (0, 1).into()],
                180 => [(0, 0).into(), (1, 0).into(), (1, 1).into(), (1, 2).into()],
                _ => [(0, 1).into(), (1, 1).into(), (2, 1).into(), (2, 0).into()],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BoardCell {
    Empty,
    Occupied,
}

impl Default for BoardCell {
    fn default() -> Self {
        Self::Empty
    }
}

impl BoardCell {
    fn is_occupied(&self) -> bool {
        match self {
            BoardCell::Occupied => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Stats {
    peak: usize,
    holes: usize,
}

#[derive(Debug, Default, Clone, Copy)]
struct Board([[BoardCell; 10]; 20]);

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter().rev() {
            format!(
                "|{}|\n",
                line.iter()
                    .map(|cell| match cell {
                        BoardCell::Empty => ' ',
                        BoardCell::Occupied => '#',
                    })
                    .collect::<String>()
            )
            .fmt(f)?;
        }
        "_".repeat(self.0[0].len() + 2).fmt(f)
    }
}

impl Board {
    fn fit(&self, tetromino: &Tetromino) -> usize {
        let blocks = tetromino.blocks();
        for line_index in (0..20).rev() {
            for block in &blocks {
                //eprintln!("{:?} {:?} {:?}", line_index, block, tetromino);
                if block.line > line_index || block.column > 10 - tetromino.position {
                    return line_index + 1;
                }
                if let BoardCell::Occupied =
                    self.0[line_index - block.line][tetromino.position + block.column - 1]
                {
                    return line_index + 1;
                }
            }
        }
        0
    }

    fn apply(&mut self, tetromino: &Tetromino, line_index: usize) {
        for block in &tetromino.blocks() {
            self.0[line_index - block.line][tetromino.position + block.column - 1] =
                BoardCell::Occupied;
        }
        for line_index in 0..self.0.len() {
            while self.0[line_index].iter().all(|cell| cell.is_occupied()) {
                self.0.copy_within(line_index + 1.., line_index);
            }
        }
    }

    fn stats(&self) -> Stats {
        let mut peak = 0;
        for (line_index, line) in self.0.iter().enumerate().rev() {
            if line.iter().any(|cell| cell.is_occupied()) {
                peak = line_index;
                break;
            }
        }
        let mut holes = 0;
        for column in 0..10 {
            holes += self
                .0
                .iter()
                .rev()
                .map(|line| line[column])
                .skip_while(|cell| !cell.is_occupied())
                .filter(|cell| !cell.is_occupied())
                .count();
        }
        Stats { peak, holes }
    }
}

fn main() -> Result<(), ()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut board = Board::default();
    let mut line = String::new();
    let mut commands = String::new();
    loop {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let tetromino: Tetromino = line.parse()?;
        //eprintln!("{:?}", tetromino);
        let mut best_position = (
            Stats {
                peak: 21,
                holes: 201,
            },
            tetromino,
            0,
        );
        for position in 1..11 {
            for rotation in (0..360).step_by(90) {
                let attempt_tetromino = Tetromino {
                    kind: tetromino.kind,
                    position,
                    rotation,
                };
                let attempted_tetromino_fit = board.fit(&attempt_tetromino);
                if attempted_tetromino_fit >= 20 {
                    continue;
                }
                let mut board_clone = board.clone();
                board_clone.apply(&attempt_tetromino, attempted_tetromino_fit);
                let stats = board_clone.stats();
                //eprintln!("Board:\n{}", board_clone);
                //eprintln!("Stats {:?}: {:?}", attempt_tetromino, stats);
                if stats < best_position.0 {
                    best_position = (stats, attempt_tetromino, attempted_tetromino_fit);
                }
            }
        }
        //eprintln!("{:?}", best_position);
        let best_tetromino = best_position.1;
        commands.clear();
        commands += &"shift_left ".repeat(tetromino.position - 1);
        commands += &"rotate ".repeat(best_tetromino.rotation / 90);
        commands += &"shift_right ".repeat(best_tetromino.position - 1);
        println!("{}", commands);

        board.apply(&best_tetromino, best_position.2);
        //eprintln!("Board:\n{}", board);
    }
}
