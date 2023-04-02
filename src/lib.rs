use std::{cmp::max, collections::HashMap, fmt::Debug, str::FromStr};

#[derive(Debug)]
pub enum Cell {
    Text(String),
    Number(f64),
    Date,
    Expression,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<f64>() {
            Ok(Cell::Number(x))
        } else {
            Ok(Cell::Text(s.to_owned()))
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Pos(usize, usize);

pub struct Sheet {
    pub table: HashMap<Pos, Cell>,
    pub width: usize,
    pub height: usize,
}

impl Debug for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}", self.str_table())
    }
}

impl Sheet {
    // BUG:this isn't a bug but its really bad
    fn str_table(&self) -> String {
        let mut str_table = String::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.table.get(&Pos(col, row));
                str_table.push_str(" ");

                match cell {
                    Some(cell) => match cell {
                        Cell::Text(text) => str_table.push_str(text),
                        Cell::Number(number) => str_table.push_str(&format!("{number}")),
                        Cell::Expression => todo!(),
                        Cell::Date => todo!(),
                    },
                    None => {}
                }
            }
            str_table.push_str("\n");
        }

        return str_table;
    }
}

impl FromStr for Sheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut table = HashMap::new();

        let mut width = 0;
        let mut height = 0;

        for (row, row_content) in s.lines().enumerate() {
            for (col, cell) in row_content.split(';').enumerate() {
                width = max(width, col);
                height = max(height, row);

                let cell = cell.trim().parse()?;
                table.insert(Pos(col, row), cell);
            }
        }

        width += 1;
        height += 1;

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

        assert_eq!(sheet.width, 6)
    }

    #[test]
    fn input_with_6_height() {
        let input = ";\n;\n;\n;\n;\n;".to_owned();
        let sheet: Sheet = input.parse().expect("Not possible to parse!");

        assert_eq!(sheet.height, 6)
    }

    #[test]
    fn cell_number_from_str() {
        let cell: Cell = "10".parse().expect("Not possible to convert to a cell.");

        match cell {
            Cell::Text(_) => panic!("Not a text"),
            Cell::Number(x) => assert_eq!(x, 10.0),
            Cell::Expression => panic!("Not a Expression"),
            Cell::Date => unimplemented!(),
        }
    }

    #[test]
    fn cell_text_from_str() {
        let cell: Cell = "Suarez"
            .parse()
            .expect("Not possible to convert to a cell.");

        match cell {
            Cell::Text(s) => assert_eq!("Suarez", s),
            Cell::Number(_) => panic!("Not a text"),
            Cell::Expression => panic!("Not a Expression"),
            Cell::Date => unimplemented!(),
        }
    }
}
