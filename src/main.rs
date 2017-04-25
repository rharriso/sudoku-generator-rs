use std::collections::HashSet;
use std::iter::Map;

#[derive(Hash, Eq, PartialEq, PartialOrd)]
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
    fn new(position: Coord) -> SudokuCell {
        let neighbors = SudokuCell::generateAllNeighbors(&position);

        SudokuCell {
            value: 0,
            neighbors: neighbors,
            position: position
        }
    }

    fn generateAllNeighbors(position: &Coord) -> Vec<Coord> {
        let mut neighbors = HashSet::new();

        for index in 0..SIZE {
            neighbors.insert(Coord{i: index, j: position.j});
            neighbors.insert(Coord{i: position.i, j: index});
        }

        let iFloor = (position.i / THIRD) * THIRD;
        let jFloor = (position.j / THIRD) * THIRD;

        for i in iFloor..(iFloor + THIRD) {
            for j in jFloor..(jFloor + THIRD) {
                neighbors.insert(Coord{i: i, j: j});
            }
        }

        return neighbors.into_iter().collect();
    }
}

const FULL_SIZE : usize = 81;
const SIZE : usize = 9;
const THIRD: usize = 3;

fn index_to_coord(index: usize) -> Coord {
    Coord{
        i: index / SIZE,
        j: index % SIZE
    }
}

#[test]
fn test_index_to_coord (){
   assert!(index_to_coord(0) == Coord{i: 0, j: 0});
   assert!(index_to_coord(80) == Coord{i: 8, j: 8});
}

fn coord_to_index(coord: Coord) -> usize {
    return coord.i * SIZE + coord.j;
}

fn coord_ref_to_index(coord: &Coord) -> usize {
    return coord.i * SIZE + coord.j;
}

#[test]
fn test_coord_to_index (){
    assert!(coord_to_index(Coord{i: 0, j: 0}) == 0);
    assert!(coord_to_index(Coord{i: 8, j: 8}) == 80);
}

struct SudokuBoard {
   cells: Vec<SudokuCell>
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        let mut cells = Vec::with_capacity(81);
        for index in 0..FULL_SIZE {
            cells.push(SudokuCell::new(index_to_coord(index)));
        }

        SudokuBoard{ cells: cells}
    }

    fn print(&self) {
        println!("----------------------------------");

        for cell in &self.cells {
            if cell.position.j == 0 {
                print ! ("| ");
            }
            print!("{}  ", cell.value);

            if (cell.position.j % THIRD) == (THIRD - 1) {
                print ! ("| ");

            }
            if cell.position.j == SIZE -1 {
                print!("\n");

                if cell.position.i % THIRD == THIRD - 1 {
                    println!("----------------------------------");
                }
            }
        }
    }

    fn markCell(&mut self) {
        let cells = self.cells.as_mut_slice();
        let (head, tails) = cells.split_at_mut(60);
        let mut cell = head.last_mut().unwrap();
        cell.value = 1;

        for n in cell.neighbors {
            let neighbor = cells.get_mut(coord_to_index(n));
            match neighbor {
                Some(n) => n.value = 7,
                None => println!("Cannot divide by 0")
            }
        }


    }
}



fn main() {
    let mut board = SudokuBoard::new();
    board.markCell();
    board.print();
}
