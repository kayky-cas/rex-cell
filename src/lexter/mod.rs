use anyhow::Result;

#[derive(Debug, PartialEq)]
enum ExpressionToken {
    Number(i64),
    Plus(char),
    Minus(char),
    Multiply(char),
    Divide(char),
    LeftParenthesis(char),
    RightParenthesis(char),
    Identifier(String),
}

struct Lexer {
    input: String,
    position: usize,
    paranthese_count: isize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            paranthese_count: 0,
        }
    }
}

impl Iterator for Lexer {
    type Item = Result<ExpressionToken>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
}
