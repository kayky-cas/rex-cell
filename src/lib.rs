use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Pos(usize, usize);

#[derive(Debug)]
pub struct Sheet {
    pub table: HashMap<Pos, String>,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Sheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // BUG: not increase
        let mut width = 0;
        // BUG: not increase
        let mut height = 0;

        let table: HashMap<Pos, String> = s
            .split('\n')
            .enumerate()
            .flat_map(|(row, row_content)| {
                row_content
                    .split(";")
                    .enumerate()
                    .map(move |(col, content)| {
                        if col > width {
                            width = col;
                        }

                        if row > height {
                            height = col;
                        }

                        return (Pos(row, col), content.trim().to_owned());
                    })
            })
            .collect();

        Ok(Sheet {
            table,
            width,
            height,
        })
    }
}
