use colored::*;
use core::fmt;
use std::thread;
use std::time;

enum Cell {
    Alive(usize),
    Dead(usize),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Alive(_) => write!(f, "{}", "O".green()),
            Cell::Dead(_) => write!(f, "{}", "O".red()),
        }
    }
}

impl Cell {
    fn get_index(&self) -> &usize {
        match self {
            Cell::Alive(index) => index,
            Cell::Dead(index) => index,
        }
    }

    fn get_neighbors<'a>(&'a self, game_state: &'a [Cell]) -> Vec<&Cell> {
        let mut neighbors: Vec<&Cell> = Vec::new();
        let index = self.get_index();
        let indexes = vec![1, 9, 10, 11];

        // Negative indexes
        // There has to be a better way
        if index != &0 {
            if index < &10 {
                if let Some(val) = game_state.get(index - 1) {
                    neighbors.push(val)
                }
            } else if index % 10 == 0 {
                if let Some(val) = game_state.get(index - 10) {
                    neighbors.push(val)
                }
                if let Some(val) = game_state.get(index - 9) {
                    neighbors.push(val)
                }
            } else {
                for i in &indexes {
                    if let Some(val) = game_state.get(index - i) {
                        neighbors.push(val)
                    }
                }
            }
        }

        // Positive indexes
        for i in indexes {
            if let Some(val) = game_state.get(index + i) {
                neighbors.push(val)
            }
        }

        neighbors
    }
}

struct GameBoard {
    game_state: Vec<Cell>,
}

impl GameBoard {
    fn get_next_state(&self) -> Vec<Cell> {
        let mut new_state = Vec::new();
        for cell in &self.game_state {
            let neighbors = cell.get_neighbors(&self.game_state);
            let alive_neighbors: Vec<&Cell> = neighbors
                .into_iter()
                .filter(|neighbor| matches!(neighbor, Cell::Alive(_)))
                .collect();
            let alive_count = alive_neighbors.len();
            match cell {
                Cell::Alive(index) => {
                    match alive_count {
                        2 | 3 => new_state.push(Cell::Alive(*index)),
                        _ => new_state.push(Cell::Dead(*index)),
                    };
                }
                Cell::Dead(index) => {
                    match alive_count {
                        3 => new_state.push(Cell::Alive(*index)),
                        _ => new_state.push(Cell::Dead(*index)),
                    };
                }
            }
        }
        new_state
    }
}

fn main() {
    let size: usize = 10;
    let mut initial_state = Vec::new();

    for i in 0..usize::pow(size, 2) {
        initial_state.push(if rand::random() {
            Cell::Dead(i)
        } else {
            Cell::Alive(i)
        })
    }

    let mut board = GameBoard {
        game_state: initial_state,
    };

    for row in board.game_state.chunks(size) {
        println!("{row:?}")
    }
    loop {
        print!("{}[2J", 27 as char); // Clear terminal with magic
        let new_state = GameBoard::get_next_state(&board);
        board.game_state = new_state;
        for row in board.game_state.chunks(size) {
            println!("{row:?}")
        }
        thread::sleep(time::Duration::from_millis(500))
    }
}
