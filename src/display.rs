use core::fmt;

use crate::board::Board;

pub fn write_column_labels(
    f: &mut fmt::Formatter,
    board_size: usize,
    indent: usize,
) -> fmt::Result {
    write_indent(f, indent)?;
    write!(f, " ")?;

    for column in 0..board_size {
        write!(f, " {} ", column + 1)?;
    }

    writeln!(f)
}

pub fn write_row(f: &mut fmt::Formatter, board: &Board, row: usize) -> fmt::Result {
    write_indent(f, row)?;
    write!(f, "{:2}\\", row + 1)?;

    for column in 0..board.size() {
        if column > 0 {
            write!(f, "  ")?;
        }
        write!(f, "{}", board.get_board()[row][column].to_char())?;
    }

    writeln!(f, "\\{:2}", row + 1)
}

pub fn write_indent(f: &mut fmt::Formatter, length: usize) -> fmt::Result {
    write!(f, "{}", " ".repeat(length))
}
