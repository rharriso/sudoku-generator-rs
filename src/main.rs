extern crate rand;

use std::collections::HashSet;
use rand::{Rng};


const SIZE: usize = 3;
const SIZE_SQAURE: usize = SIZE * SIZE;
const SIZE_SQUARE_64: u64 = SIZE_SQAURE as u64;
const SIZE_QUAD: usize = SIZE_SQAURE * SIZE_SQAURE;


#[derive(Clone, Hash, Eq, PartialEq, PartialOrd)]
struct Coord {
    i: usize,
    j: usize
}

struct SudokuCell {
    position: Coord,
    value: u64,
    neighbors: Vec<Coord>
}

impl SudokuCell {
    fn new(position: Coord, all_neighbors: bool) -> SudokuCell {
        let neighbors = if all_neighbors {
            SudokuCell::generate_all_neighbors(&position)
        }
        else {
            SudokuCell::generate_optimal_neighbors(&position)
        };

        SudokuCell {
            value: 0,
            neighbors: neighbors,
            position: position
        }
    }

    fn generate_all_neighbors(position: &Coord) -> Vec<Coord> {
        let mut neighbors = HashSet::new();

        for index in 0..SIZE_SQAURE{
            neighbors.insert(Coord{i: index, j: position.j});
            neighbors.insert(Coord{i: position.i, j: index});
        }

        let i_floor = (position.i / SIZE) * SIZE;
        let j_floor = (position.j / SIZE) * SIZE;

        for i in i_floor..(i_floor + SIZE) {
            for j in j_floor..(j_floor + SIZE) {
                neighbors.insert(Coord{i: i, j: j});
            }
        }

        return neighbors.into_iter().collect();
    }

    fn generate_optimal_neighbors(position: &Coord) -> Vec<Coord> {
        let mut neighbors = HashSet::new();

        for index in 0..position.i {
            neighbors.insert(Coord{i: index, j: position.j});
        }

        for index in 0..position.j {
            neighbors.insert(Coord{i: position.i, j: index});
        }

        let i_floor = (position.i / SIZE) * SIZE;
        let j_floor = (position.j / SIZE) * SIZE;


        for i in i_floor..position.i {
            for j in j_floor..(j_floor + SIZE) {
                if i < position.i || j < position.j {
                    neighbors.insert(Coord{i: i, j: j});
                }

            }
        }
        
        return neighbors.into_iter().collect();
    }

}

fn index_to_coord(index: usize) -> Coord {
    Coord{
        i: index / SIZE_SQAURE,
        j: index % SIZE_SQAURE
    }
}

#[test]
fn test_index_to_coord (){
   assert!(index_to_coord(0) == Coord{i: 0, j: 0});
   assert!(index_to_coord(80) == Coord{i: 8, j: 8});
}

fn coord_to_index(coord: &Coord) -> usize {
    return coord.i * SIZE_SQAURE + coord.j;
}

#[test]
fn test_coord_to_index (){
    assert!(coord_to_index(Coord{i: 0, j: 0}) == 0);
    assert!(coord_to_index(Coord{i: 8, j: 8}) == 80);
}

struct SudokuBoard {
   cells: Vec<SudokuCell>,
   valid_values: HashSet<u64>
}

impl SudokuBoard {
    fn new(all_neighbors: bool) -> SudokuBoard {
        let mut cells = Vec::with_capacity(81);

        for index in 0..SIZE_QUAD {
            cells.push(SudokuCell::new(index_to_coord(index), all_neighbors));
        }

        let mut valid_values : HashSet<u64> = HashSet::new();

        for n in 1u64..(SIZE_SQUARE_64 + 1) {
            valid_values.insert(n);
        }

        SudokuBoard{ cells: cells, valid_values: valid_values}
    }

    fn fill(&mut self) {
        self.do_fill(0);
    }

    fn do_fill(&mut self, index: usize) -> bool {
        let ref cell_pos = index_to_coord(index);
        let neighbor_values = self.neighbor_values(cell_pos);

        // get remaining values and shuffle them
        let mut remaining_values: Vec<u64> = self.valid_values
                    .difference(&neighbor_values).cloned().collect();
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut remaining_values);

        // try the remaining values
        for v in remaining_values {
            self.mark_cell(cell_pos, v);

            if index == SIZE_QUAD - 1 || self.do_fill(index + 1) {
                return true;
            }
        }

        self.mark_cell(cell_pos, 0);
        return false;
    }

    fn print(&self) {
        println!("-------------------------------------------");

        for cell in &self.cells {
            if cell.position.j == 0 {
                print ! ("| ");
            }
            print!("{}  ", cell.value);

            if (cell.position.j % SIZE) == (SIZE - 1) {
                print ! ("| ");

            }
            if cell.position.j == SIZE_SQAURE -1 {
                print!("\n");

                if cell.position.i % SIZE == SIZE - 1 {
                    println!("-------------------------------------------");
                }
            }
        }
    }

    fn clear(&mut self) {
        for n in 0..SIZE_QUAD {
            self.mark_cell_pos(n, 0);
        }
    }

    fn serialize(&self) -> String {
        let mut result = "".to_string();

        for cell in &self.cells {
            result = result + &cell.value.to_string() + &"|";
        }

        return result;
    }

    fn mark_cell(&mut self, coord: &Coord, value: u64) {
        let index = coord_to_index(coord);
        self.mark_cell_pos(index, value);
    }
    
    fn mark_cell_pos(&mut self, index: usize, value: u64) {
        let cell = self.cells.get_mut(index).unwrap();
        cell.value = value;
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let index = coord_to_index(coord);
        let ref cell = self.cells.get(index).unwrap();
        cell.neighbors.clone()
    }

    fn neighbor_values(&self, coord: &Coord) -> HashSet<u64> {
        let mut values = HashSet::new();

        for ref coord in self.neighbors(coord) {
            let ref cell = self.cells.get(coord_to_index(coord)).unwrap();
            values.insert(cell.value);
        }

        return values;
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} board_count [--all-neighbors]", args[0]);
        return;
    }

    let board_count: usize = args[1].parse().unwrap();
    let mut all_neighbors = false;
    
    if args.len() > 2 {
        all_neighbors = args[2] == "--all-neighbors";
    }


    let mut board = SudokuBoard::new(all_neighbors);
    let mut result = "".to_string();

    if all_neighbors {
        for _ in 0..board_count {
            board.fill();
            result = result + &board.serialize() + "\n";
            board.clear();
        }
    } else {
        for _ in 0..board_count {
            board.fill();
            result = result + &board.serialize() + "\n";
        }
    }

    println!("{}", result);
}
