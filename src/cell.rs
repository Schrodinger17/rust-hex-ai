use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Cell {
        Cell { x, y }
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, other: Cell) -> Cell {
        Cell {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Cell {
    pub fn is_on_board(&self, size: usize) -> bool {
        0 <= self.x && self.x < size as i32 && 0 <= self.y && self.y < size as i32
    }

    pub fn neighbors(&self, board_size: usize) -> Vec<Cell> {
        let steps = [
            Cell::new(-1, 0),
            Cell::new(-1, 1),
            Cell::new(0, -1),
            Cell::new(0, 1),
            Cell::new(1, -1),
            Cell::new(1, 0),
        ];

        steps
            .iter()
            .map(|step| *step + *self)
            .filter(|new_cell| new_cell.is_on_board(board_size))
            .collect()
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_is_on_board() {
        let cell = Cell::new(0, 0);
        assert!(cell.is_on_board(3));
        let cell = Cell::new(2, 2);
        assert!(cell.is_on_board(3));
        let cell = Cell::new(3, 3);
        assert!(!cell.is_on_board(3));
        let cell = Cell::new(-1, 0);
        assert!(!cell.is_on_board(3));
    }

    #[test]
    fn test_cell_add() {
        let cell1 = Cell::new(0, 0);
        let cell2 = Cell::new(1, 1);
        let cell3 = cell1 + cell2;
        assert_eq!(cell3, Cell::new(1, 1));
    }
}
