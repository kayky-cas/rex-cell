use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Pos(char, usize);

#[derive(Debug)]
pub struct Sheet {
    pub table: HashMap<Pos, String>,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Sheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let table: HashMap<Pos, String> = s
            .lines()
            .enumerate()
            .flat_map(|(row, row_content)| {
                row_content.split(";").enumerate().map(move |(col, cell)| {
                    if col > 25 {
                        return None;
                    }
                    let col = char::from_u32(col as u32 + b'A' as u32)?;
                    return Some((Pos(col, row), cell.trim().to_owned()));
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
    use crate::Sheet;

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
}
