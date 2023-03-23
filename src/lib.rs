use std::{cmp::max, collections::HashMap, fmt::Debug, str::FromStr};

use anyhow::anyhow;

struct Lexer<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            bytes: value.as_bytes(),
        }
    }
}

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
            Ok(Cell::Number(x))
        } else if s.starts_with('=') {
            let buffer = s[1..].as_bytes();

            if buffer.is_empty() {
                return Err(anyhow!("Empty expression"));
            }

            let mut start = 0usize;
            let mut cursor = 0usize;

            let mut is_counting = false;

            let mut lexers = Vec::new();

            while cursor < buffer.len() {
                let cursor_buf = buffer[cursor];

                if cursor_buf.is_ascii_alphabetic() || cursor_buf.is_ascii_alphanumeric() {
                    if !is_counting {
                        is_counting = true;
                        start = cursor;
                    }
                } else {
                    let lexer = Lexer {
                        bytes: &buffer[start..cursor],
                    };
                    lexers.push(lexer);
                    is_counting = false;
                }

                cursor += 1;
            }

            Ok(Cell::Expression)
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
                        Cell::Expression => str_table.push_str("expression"),
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
        let table: HashMap<Pos, Cell> = s
            .lines()
            .enumerate()
            .flat_map(|(row, row_content)| {
                row_content.split(';').enumerate().map(move |(col, cell)| {
                    let cell = cell.trim().parse().ok()?;
                    Some((Pos(col, row), cell))
                })
            })
            .flatten()
            .collect();

        let mut width = 0usize;
        let mut height = 0usize;

        for Pos(col, row) in table.keys() {
            width = max(width, *col);
            height = max(height, *row);
        }

        Ok(Sheet {
            table,
            width: width + 1,
            height: height + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cell, Lexer, Sheet};

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
        }
    }

    #[test]
    fn lexers_from_buffer() {
        let buffer = "A1 + A3+135".as_bytes();

        let mut lexers: Vec<Lexer> = Vec::new();

        let mut cursor = 0usize;
        let mut start = 0usize;

        let mut is_counting = false;

        while cursor < buffer.len() {
            let byte = buffer[cursor];

            if byte.is_ascii_alphabetic() || byte.is_ascii_alphanumeric() {
                if !is_counting {
                    is_counting = true;
                    start = cursor;
                }

                if cursor == buffer.len() - 1 {
                    lexers.push(Lexer {
                        bytes: &buffer[start..],
                    });
                }
            } else {
                if is_counting {
                    lexers.push(Lexer {
                        bytes: &buffer[start..(cursor)],
                    });

                    is_counting = false;
                }

                if byte != b' ' {
                    lexers.push(Lexer {
                        bytes: &buffer[cursor..(cursor + 1)],
                    });
                    is_counting = false;
                }
            }

            cursor += 1;
        }

        println!("=====================");

        for l in lexers {
            println!("{}", std::str::from_utf8(l.bytes).unwrap());
        }

        assert!(false)
    }
}
