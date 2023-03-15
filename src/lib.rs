use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Cell {
    Text(String),
    Number(f64),
    Expression,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<f64>() {
            return Ok(Cell::Number(x));
        }

        return Ok(Cell::Text(s.to_owned()));
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Pos(char, usize);

#[derive(Debug)]
pub struct Sheet {
    pub table: HashMap<Pos, Cell>,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Sheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let table: HashMap<Pos, Cell> = s
            .lines()
            .enumerate()
            .flat_map(|(row, row_content)| {
                row_content.split(";").enumerate().map(move |(col, cell)| {
                    if col > 25 {
                        return None;
                    }
                    let col = char::from_u32(col as u32 + b'A' as u32)?;
                    return Some((Pos(col, row), cell.trim().to_owned().parse().unwrap()));
                })
            })
            .flatten()
            .collect();

        let width =
            (table.iter().map(|(Pos(c, _), _)| *c as u8).max().unwrap() - b'A') as usize + 1;
        let height = table.iter().map(|(Pos(_, r), _)| *r).max().unwrap() + 1;

        Ok(Sheet {
            table,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cell, Sheet};

    #[test]
    fn input_with_6_width() {
        let input = ";;;;;".to_owned();
        let sheet: Sheet = input.parse().expect("Not possible to parse!");

        assert!(sheet.width == 6)
    }

    #[test]
    fn input_with_6_height() {
        let input = ";\n;\n;\n;\n;\n;".to_owned();
        let sheet: Sheet = input.parse().expect("Not possible to parse!");

        assert!(sheet.height == 6)
    }

    #[test]
    fn cell_number_from_str() {
        let cell: Cell = "10".parse().expect("Not possible to convert to a cell.");

        match cell {
            Cell::Text(_) => panic!("Not a text"),
            Cell::Number(x) => assert_eq!(x, 10.0),
            Cell::Expression => panic!("Not a Expression"),
        }
    }

    #[test]
    fn cell_text_from_str() {
        let cell: Cell = "Carlos"
            .parse()
            .expect("Not possible to convert to a cell.");

        match cell {
            Cell::Text(s) => assert_eq!("Carlos", s),
            Cell::Number(_) => panic!("Not a text"),
            Cell::Expression => panic!("Not a Expression"),
        }
    }
}
