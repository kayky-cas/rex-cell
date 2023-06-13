use anyhow::Result;

#[derive(Debug, PartialEq)]
enum ExpressionToken {
    Number(isize),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
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
        let text = &self.input[self.position..];

        match text.chars().next() {
            Some(c) if c.is_digit(10) => {
                let number = text
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap();

                self.position += number.to_string().len();

                return Some(Ok(ExpressionToken::Number(number)));
            }
            Some(c) if c.is_whitespace() => {
                self.position += 1;
                self.next()
            }
            Some(c) => {
                self.position += 1;

                return match c {
                    '+' => Some(Ok(ExpressionToken::Plus)),
                    '-' => Some(Ok(ExpressionToken::Minus)),
                    '*' => Some(Ok(ExpressionToken::Multiply)),
                    '/' => Some(Ok(ExpressionToken::Divide)),
                    '(' => {
                        self.paranthese_count += 1;
                        Some(Ok(ExpressionToken::LeftParenthesis))
                    }
                    ')' => {
                        self.paranthese_count -= 1;
                        if self.paranthese_count < 0 {
                            return Some(Err(anyhow::anyhow!("Unbalanced parenthesis")));
                        }
                        Some(Ok(ExpressionToken::RightParenthesis))
                    }
                    'a'..='z' | 'A'..='Z' => {
                        let identifier = text
                            .chars()
                            .take_while(|c| c.is_alphabetic())
                            .collect::<String>();

                        self.position += identifier.len() - 1;

                        Some(Ok(ExpressionToken::Identifier(identifier)))
                    }
                    _ => None,
                };
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lexer(input: String, expected: Vec<ExpressionToken>) {
        let mut lexer = Lexer::new(input);

        for token in expected {
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        }
    }

    #[test]
    fn basic_expression() {
        let input = "1 + 2 * 3";

        let expected = vec![
            ExpressionToken::Number(1),
            ExpressionToken::Plus,
            ExpressionToken::Number(2),
            ExpressionToken::Multiply,
            ExpressionToken::Number(3),
        ];

        test_lexer(input.to_string(), expected);
    }

    #[test]
    fn identifier() {
        let input = "a + b * c";

        let expected = vec![
            ExpressionToken::Identifier("a".to_string()),
            ExpressionToken::Plus,
            ExpressionToken::Identifier("b".to_string()),
            ExpressionToken::Multiply,
            ExpressionToken::Identifier("c".to_string()),
        ];

        test_lexer(input.to_string(), expected);
    }
}
