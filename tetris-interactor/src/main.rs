use std::io::BufRead;

use rand::{rngs::StdRng, seq::SliceRandom, RngCore, SeedableRng};
use tracing::{debug, info, trace};

const EXIT_CODE_OK: i32 = 0;
const EXIT_CODE_WA: i32 = 1;
const EXIT_CODE_PE: i32 = 2;

struct Tetris {
    rng: StdRng,
    board: TetrisBoard,
    score: u64,
    score_limit: u64,
}

#[derive(Debug)]
enum GameOver {
    Ok,
    WrongInput,
    Dead,
}

impl Tetris {
    fn new(rng: StdRng, score_limit: u64) -> Self {
        Self {
            rng,
            board: TetrisBoard::default(),
            score: 0,
            score_limit,
        }
    }

    fn play(&mut self) -> GameOver {
        let mut line = String::new();
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        while self.score < self.score_limit {
            let mut new_tetromino = Tetromino {
                kind: *TETROMINO_KINDS
                    .choose(&mut self.rng)
                    .unwrap_or(&TetrominoKind::O),
                position: 0,
                rotation: 0,
            };
            new_tetromino.position =
                self.rng.next_u32() as usize % (11 - new_tetromino.width()) + 1;

            #[cfg(not(feature = "level-9"))]
            println!("{}", new_tetromino.position);
            #[cfg(feature = "level-9")]
            println!("{} {}", new_tetromino.kind, new_tetromino.position);
            info!(target: "game_log", "{} {}", new_tetromino.kind, new_tetromino.position);

            line.clear();
            if let Err(error) = stdin.read_line(&mut line) {
                debug!("Reading a new line from a solution failed: {:?}", error);
                return GameOver::WrongInput;
            }
            info!(target: "game_log", "{}", line.trim());

            for action in line.split_ascii_whitespace() {
                match action {
                    "shift_left" => {
                        if new_tetromino.position > 1 {
                            new_tetromino.position -= 1;
                        }
                    }
                    "shift_right" => {
                        if new_tetromino.position + new_tetromino.width() <= self.board.width() {
                            new_tetromino.position += 1;
                        }
                    }
                    "rotate" => {
                        new_tetromino.rotation += 90;
                        // Allow rotating tetromino when it does not fit by just updating the position
                        let rightmost_allowed_position =
                            self.board.width() - new_tetromino.width() + 1;
                        if new_tetromino.position > rightmost_allowed_position {
                            new_tetromino.position = rightmost_allowed_position;
                        }
                    }
                    "" => {}
                    _ => return GameOver::WrongInput,
                };
            }
            debug_assert!(new_tetromino.position >= 1);
            debug_assert!(new_tetromino.position <= self.board.width() - new_tetromino.width() + 1);
            if let Err(_) = self.board.try_apply_tetromino(new_tetromino) {
                return GameOver::Dead;
            }

            self.score += self.board.clean_full_lines();
        }

        GameOver::Ok
    }
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
enum TetrominoKind {
    #[cfg(feature = "level-9")]
    I,
    O,
    #[cfg(feature = "level-9")]
    T,
    #[cfg(feature = "level-9")]
    S,
    #[cfg(feature = "level-9")]
    Z,
    #[cfg(feature = "level-9")]
    J,
    #[cfg(feature = "level-9")]
    L,
}

#[cfg(not(feature = "level-9"))]
const TETROMINO_KINDS: &[TetrominoKind] = &[TetrominoKind::O];

#[cfg(feature = "level-9")]
const TETROMINO_KINDS: &[TetrominoKind] = &[
    TetrominoKind::I,
    TetrominoKind::O,
    TetrominoKind::T,
    TetrominoKind::S,
    TetrominoKind::Z,
    TetrominoKind::J,
    TetrominoKind::L,
];

#[derive(Debug)]
struct Tetromino {
    kind: TetrominoKind,
    position: usize,
    rotation: usize,
}

impl Tetromino {
    pub fn blocks(&self) -> &'static [(usize, usize)] {
        match self.kind {
            #[cfg(feature = "level-9")]
            TetrominoKind::I => match self.rotation % 180 {
                0 => &[(0, 0), (0, 1), (0, 2), (0, 3)],
                _ => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            },
            TetrominoKind::O => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            #[cfg(feature = "level-9")]
            TetrominoKind::T => match self.rotation % 360 {
                0 => &[(0, 0), (1, 0), (2, 0), (1, 1)],
                90 => &[(1, 0), (1, 1), (1, 2), (0, 1)],
                180 => &[(0, 1), (1, 1), (2, 1), (1, 0)],
                _ => &[(0, 0), (0, 1), (0, 2), (1, 1)],
            },
            #[cfg(feature = "level-9")]
            TetrominoKind::S => match self.rotation % 180 {
                0 => &[(0, 1), (1, 1), (1, 0), (2, 0)],
                _ => &[(0, 0), (0, 1), (1, 1), (1, 2)],
            },
            #[cfg(feature = "level-9")]
            TetrominoKind::Z => match self.rotation % 180 {
                0 => &[(0, 0), (1, 0), (1, 1), (2, 1)],
                _ => &[(1, 0), (1, 1), (0, 1), (0, 2)],
            },
            #[cfg(feature = "level-9")]
            TetrominoKind::J => match self.rotation % 360 {
                0 => &[(1, 0), (1, 1), (1, 2), (0, 2)],
                90 => &[(0, 0), (0, 1), (1, 1), (2, 1)],
                180 => &[(0, 0), (0, 1), (0, 2), (1, 0)],
                _ => &[(0, 0), (1, 0), (2, 0), (2, 1)],
            },
            #[cfg(feature = "level-9")]
            TetrominoKind::L => match self.rotation % 360 {
                0 => &[(0, 0), (0, 1), (0, 2), (1, 2)],
                90 => &[(0, 0), (1, 0), (2, 0), (0, 1)],
                180 => &[(0, 0), (1, 0), (1, 1), (1, 2)],
                _ => &[(0, 1), (1, 1), (2, 1), (2, 0)],
            },
        }
    }

    fn width(&self) -> usize {
        self.blocks().iter().map(|(x, _)| x).max().unwrap_or(&0) + 1
    }
}

#[derive(Debug, Clone, Copy)]
enum TetrisCell {
    Empty,
    Occupied,
}

impl Default for TetrisCell {
    fn default() -> Self {
        Self::Empty
    }
}

type TetrisBoardLine = [TetrisCell; 10];

#[derive(Default)]
struct TetrisBoard([TetrisBoardLine; 20]);

impl std::fmt::Display for TetrisBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter().rev() {
            format!(
                "|{}|\n",
                line.iter()
                    .map(|cell| match cell {
                        TetrisCell::Empty => ' ',
                        TetrisCell::Occupied => '#',
                    })
                    .collect::<String>()
            )
            .fmt(f)?;
        }
        "_".repeat(self.width() + 2).fmt(f)
    }
}

impl TetrisBoard {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn fit_tetromino(
        board: &mut [TetrisBoardLine],
        tetromino: &Tetromino,
        should_save: bool,
    ) -> Result<(), ()> {
        if board.is_empty() {
            return Err(());
        }
        let board_top = board.len() - 1;
        let positions: Vec<(usize, usize)> = tetromino
            .blocks()
            .into_iter()
            .map(|(x, y)| (x + tetromino.position - 1, *y))
            .collect();

        for position in &positions {
            if position.1 >= board.len() {
                return Err(());
            }
            if let TetrisCell::Occupied = board[board_top - position.1][position.0] {
                return Err(());
            }
        }
        if should_save {
            for position in &positions {
                board[board_top - position.1][position.0] = TetrisCell::Occupied;
            }
        }
        Ok(())
    }

    pub fn try_apply_tetromino(&mut self, tetromino: Tetromino) -> Result<(), ()> {
        trace!("Trying to apply {:?} to the board\n{}", tetromino, self);
        let mut vertical_position = self.0.len();
        while Self::fit_tetromino(&mut self.0[..vertical_position], &tetromino, false).is_ok() {
            vertical_position -= 1;
        }
        if vertical_position == self.0.len() {
            debug!("Tetromino could not get placed on the board");
            Err(())
        } else {
            debug!(
                "Tetromino {} shifted by {} will get placed on the line {}",
                tetromino.kind, tetromino.position, vertical_position
            );
            Self::fit_tetromino(&mut self.0[..vertical_position + 1], &tetromino, true)
                .expect("unreachable");
            trace!("Applied {:?} to the board\n{}", tetromino, self);

            Ok(())
        }
    }

    fn clean_full_lines(&mut self) -> u64 {
        let mut cleaned_lines = 0;
        let mut line_index = 0;
        while line_index < self.0.len() {
            if self.0[line_index].iter().all(|cell| match cell {
                TetrisCell::Occupied => true,
                _ => false,
            }) {
                self.0.copy_within(line_index + 1.., line_index);
                cleaned_lines += 1;
            } else {
                line_index += 1;
            }
        }

        debug!("Cleaned {} lines.", cleaned_lines);
        trace!("The board is:\n{}", self);
        cleaned_lines
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Initializing Tetris interactor");

    let mut seed = [0; 32];
    let answer = std::fs::read_to_string("answer.txt").unwrap_or(String::new());
    let seed_len = seed.len().min(answer.len());
    seed[..seed_len].copy_from_slice(answer.as_bytes());
    let mut tetris = Tetris::new(
        rand::rngs::StdRng::from_seed(seed),
        answer
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
    );

    let game_status = tetris.play();
    info!("{:?}. Score: {}", game_status, tetris.score);

    // Signal game over
    println!("0");

    // Wait for the solution to react to catch WA
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();

    let exit_code = match game_status {
        GameOver::Ok => EXIT_CODE_OK,
        GameOver::WrongInput => EXIT_CODE_PE,
        GameOver::Dead => EXIT_CODE_WA,
    };
    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_tetromino_blocks() {
        for tetromino_kind in TETROMINO_KINDS {
            for &rotation in &[0, 90, 180, 270] {
                let mut board = TetrisBoard::default();
                board
                    .try_apply_tetromino(Tetromino {
                        kind: *tetromino_kind,
                        position: 1,
                        rotation,
                    })
                    .unwrap();
                assert_snapshot!(
                    format!("tetromino_blocks_{}_{}", tetromino_kind, rotation),
                    board.to_string()
                );
            }
        }
    }

    #[cfg(feature = "level-9")]
    #[test]
    fn test_tetris_board() {
        let mut board = TetrisBoard::default();

        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::I,
                position: 1,
                rotation: 90,
            })
            .unwrap();
        board.clean_full_lines();
        assert_snapshot!(board.to_string());

        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::I,
                position: 1,
                rotation: 270,
            })
            .unwrap();
        board.clean_full_lines();
        assert_snapshot!(board.to_string());

        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::I,
                position: 9,
                rotation: 180,
            })
            .unwrap();
        board.clean_full_lines();
        assert_snapshot!(board.to_string());

        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::I,
                position: 10,
                rotation: 360,
            })
            .unwrap();
        board.clean_full_lines();
        assert_snapshot!(board.to_string());
        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::O,
                position: 5,
                rotation: 90,
            })
            .unwrap();
        board.clean_full_lines();
        assert_snapshot!(board.to_string());

        board
            .try_apply_tetromino(Tetromino {
                kind: TetrominoKind::O,
                position: 7,
                rotation: 90,
            })
            .unwrap();
        assert_snapshot!(board.to_string());
        board.clean_full_lines();
        assert_snapshot!(board.to_string());
    }
}
