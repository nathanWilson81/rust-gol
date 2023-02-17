use colored::*;
use core::fmt;
use std::thread;
use std::time;

enum Cell {
    Alive(usize, Vec<usize>),
    Dead(usize, Vec<usize>),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Alive(_, _) => write!(f, "{}", "O".green()),
            Cell::Dead(_, _) => write!(f, "{}", "O".red()),
        }
    }
}

impl Cell {
    fn get_neighbors<'a>(&'a self, game_state: &'a Vec<Cell>) -> Vec<&Cell> {
        let mut neighbors: Vec<&Cell> = Vec::new();
        let cell_neighbors = match self {
            Cell::Alive(_, neighbors) | Cell::Dead(_, neighbors) => neighbors,
        };

        cell_neighbors.iter().for_each(|neighbor| {
            if let Some(neighbor) = game_state.get(*neighbor) {
                neighbors.push(neighbor.clone())
            }
        });

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
                .filter(|neighbor| matches!(neighbor, Cell::Alive(_, _)))
                .collect();
            let alive_count = alive_neighbors.len();
            match cell {
                Cell::Alive(index, neighbors) => {
                    match alive_count {
                        2 | 3 => new_state.push(Cell::Alive(*index, neighbors.clone())),
                        _ => new_state.push(Cell::Dead(*index, neighbors.clone())),
                    };
                }
                Cell::Dead(index, neighbors) => {
                    match alive_count {
                        3 => new_state.push(Cell::Alive(*index, neighbors.clone())),
                        _ => new_state.push(Cell::Dead(*index, neighbors.clone())),
                    };
                }
            }
        }
        new_state
    }
}

fn main() {
    let size: i32 = 10;
    let chunk_rate: usize = 10;
    let mut initial_state = Vec::new();
    let indexes: Vec<i32> = vec![-1, -9, -10, -11, 1, 9, 10, 11];

    for i in 0..i32::pow(size, 2) {
        let index = i as usize;
        let neighbors = indexes
            .iter()
            .map(|index| i - index)
            .filter(|index| i32::is_positive(*index))
            .map(|index| index as usize)
            .collect();
        initial_state.push(if rand::random() {
            Cell::Dead(index, neighbors)
        } else {
            Cell::Alive(index, neighbors)
        })
    }

    let mut board = GameBoard {
        game_state: initial_state,
    };

    for row in board.game_state.chunks(chunk_rate) {
        println!("{row:?}")
    }
    loop {
        print!("{}[2J", 27 as char); // Clear terminal with magic
        let new_state = GameBoard::get_next_state(&board);
        board.game_state = new_state;
        for row in board.game_state.chunks(chunk_rate) {
            println!("{row:?}")
        }
        thread::sleep(time::Duration::from_millis(500))
    }
}
