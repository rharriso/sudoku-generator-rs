use std::collections::HashSet;

extern crate rand;
use self::rand::Rng;

// how big is the board
#[derive(Debug, Clone)]
struct BoardConfig {
    all_neighbors: bool,
    size: usize,
    size_square: usize,
    size_quad: usize,
}

impl BoardConfig {
    fn new(base_size: usize, all_neighbors: bool) -> BoardConfig {
        return BoardConfig{
            all_neighbors: all_neighbors,
            size: base_size,
            size_square: base_size.pow(2),
            size_quad: base_size.pow(4),
        };
    }
}

/// 2-d position
#[derive(Clone, Hash, Eq, PartialEq, PartialOrd)]
struct Coord {
    i: usize,
    j: usize
}

/// block in a sudokuboard
///
struct SudokuCell {
    /// what value is stored here
    value: u64,
    /// what coords are related to this cell
    neighbors: Vec<Coord>
}

impl SudokuCell {
    /// create a new cell,
    /// position - where on the board
    /// all_neighbors - if false optimizes for fill
    ///
    fn new(position: Coord, board_config: &BoardConfig) -> SudokuCell {
        let neighbors = if board_config.all_neighbors {
            SudokuCell::generate_all_neighbors(&position, board_config)
        }
            else {
                SudokuCell::generate_optimal_neighbors(&position, board_config)
            };

        SudokuCell {
            value: 0,
            neighbors: neighbors
        }
    }

    fn generate_all_neighbors(position: &Coord, board_config: &BoardConfig) -> Vec<Coord> {
        let mut neighbors = HashSet::new();
        let &Coord{ i: pos_i, j: pos_j } = position;

        for index in 0..board_config.size_square{
            neighbors.insert(Coord{i: index, j: pos_j});
            neighbors.insert(Coord{i: pos_i, j: index});
        }

        // find the top left block position
        let i_floor = (pos_i / board_config.size) * board_config.size;
        let j_floor = (pos_j / board_config.size) * board_config.size;

        for i in i_floor..(i_floor + board_config.size) {
            for j in j_floor..(j_floor + board_config.size) {
                neighbors.insert(Coord{i: i, j: j});
            }
        }

        return neighbors.into_iter().collect();
    }

    fn generate_optimal_neighbors(position: &Coord, board_config: &BoardConfig) -> Vec<Coord> {
        let mut neighbors = HashSet::new();

        for index in 0..position.i {
            neighbors.insert(Coord{i: index, j: position.j});
        }

        for index in 0..position.j {
            neighbors.insert(Coord{i: position.i, j: index});
        }

        // find the top left block position
        let i_floor = (position.i / board_config.size) * board_config.size;
        let j_floor = (position.j / board_config.size) * board_config.size;


        for i in i_floor..position.i {
            for j in j_floor..(j_floor + board_config.size) {
                if i < position.i || j < position.j {
                    neighbors.insert(Coord{i: i, j: j});
                }

            }
        }

        return neighbors.into_iter().collect();
    }
}

fn index_to_coord(index: usize, board_config: &BoardConfig) -> Coord {
    Coord{
        i: index / board_config.size_square,
        j: index % board_config.size_square
    }
}

#[test]
fn test_index_to_coord (){
    assert!(index_to_coord(0, &BoardConfig::new(3, false)) == Coord{i: 0, j: 0}, );
    assert!(index_to_coord(80, &BoardConfig::new(3, false)) == Coord{i: 8, j: 8});
}

fn coord_to_index(coord: &Coord, board_config: &BoardConfig) -> usize {
    return coord.i * board_config.size_square + coord.j;
}

#[test]
fn test_coord_to_index (){
    assert!(coord_to_index(&Coord{i: 0, j: 0}, &BoardConfig::new(3, false)) == 0);
    assert!(coord_to_index(&Coord{i: 8, j: 8}, &BoardConfig::new(3, false)) == 80);
}

pub struct SudokuBoard {
    cells: Vec<SudokuCell>,
    valid_values: HashSet<u64>,
    board_config: BoardConfig,
}

impl SudokuBoard {
    fn new(board_config: BoardConfig) -> SudokuBoard {
        let mut cells = Vec::with_capacity(81);

        for index in 0..board_config.size_quad {
            let coord = index_to_coord(index, &board_config);
            cells.push(SudokuCell::new(coord, &board_config));
        }

        let mut valid_values : HashSet<u64> = HashSet::new();

        for n in 1usize..(board_config.size_square + 1) {
            valid_values.insert(n as u64);
        }

        SudokuBoard{ cells: cells, valid_values: valid_values, board_config}
    }

    fn fill(&mut self) {
        self.do_fill(0);
    }

    fn do_fill(&mut self, index: usize) -> bool {
        let ref cell_pos = index_to_coord(index, &self.board_config);
        let neighbor_values = self.neighbor_values(cell_pos);

        // get remaining values and shuffle them
        let mut remaining_values: Vec<u64> = self.valid_values
            .difference(&neighbor_values).cloned().collect();
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut remaining_values);

        // try the remaining values
        for v in remaining_values {
            self.mark_cell(cell_pos, v);

            if index == self.board_config.size_quad - 1 || self.do_fill(index + 1) {
                return true;
            }
        }

        self.mark_cell(cell_pos, 0);
        return false;
    }

    pub fn serialize(&self) -> String {
        let mut result = "".to_string();

        for cell in &self.cells {
            result = result + &cell.value.to_string() + &"|";
        }

        return result;
    }

    fn mark_cell(&mut self, coord: &Coord, value: u64) {
        let index = coord_to_index(coord, &self.board_config);
        self.mark_cell_pos(index, value);
    }

    fn mark_cell_pos(&mut self, index: usize, value: u64) {
        let cell = self.cells.get_mut(index).unwrap();
        cell.value = value;
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let index = coord_to_index(coord, &self.board_config);
        let ref cell = self.cells.get(index).unwrap();
        cell.neighbors.clone()
    }

    fn neighbor_values(&self, coord: &Coord) -> HashSet<u64> {
        let mut values = HashSet::new();

        for ref coord in self.neighbors(coord) {
            let ref cell = self.cells.get(coord_to_index(coord, &self.board_config)).unwrap();
            values.insert(cell.value);
        }

        return values;
    }
}

pub fn generate_and_fill_boards(board_count: usize, board_size: usize, all_neighbors: bool) {
    for _ in 0..board_count {
        let board_config = BoardConfig::new(board_size, all_neighbors);
        let mut board = SudokuBoard::new(board_config);
        board.fill();
    }
}