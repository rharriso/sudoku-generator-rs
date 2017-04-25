use std::collections::HashSet;

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
    fn new(position: Coord) -> SudokuCell {
        let neighbors = SudokuCell::generate_all_neighbors(&position);

        SudokuCell {
            value: 0,
            neighbors: neighbors,
            position: position
        }
    }

    fn generate_all_neighbors(position: &Coord) -> Vec<Coord> {
        let mut neighbors = HashSet::new();

        for index in 0..SIZE {
            neighbors.insert(Coord{i: index, j: position.j});
            neighbors.insert(Coord{i: position.i, j: index});
        }

        let i_floor = (position.i / THIRD) * THIRD;
        let j_floor = (position.j / THIRD) * THIRD;

        for i in i_floor..(i_floor + THIRD) {
            for j in j_floor..(j_floor + THIRD) {
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

fn coord_to_index(coord: &Coord) -> usize {
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

    fn mark_cell(&mut self, coord: &Coord, value: u64) {
        let index = coord_to_index(coord);
        let cell = self.cells.get_mut(index).unwrap();
        cell.value = value;
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let index = coord_to_index(coord);
        let ref cell = self.cells.get(index).unwrap();
        cell.neighbors.clone()
    }
}



fn main() {
    let mut board = SudokuBoard::new();
    let ref coord = Coord{i: 6, j: 6};
    let neighbors = board.neighbors(coord);

    for ref n in neighbors {
        board.mark_cell(n, 1);
    }
    board.mark_cell(coord, 7);
    board.print();
}
