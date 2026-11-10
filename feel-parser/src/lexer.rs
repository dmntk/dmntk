use crate::errors::{
    Result, err_expected_hex_digit, err_unexpected_eof, err_unicode_surrogate_out_of_range,
    err_unicode_value_out_of_range,
};

/// Definition of a single space character.
const WS: char = ' ';

/// Buffer size for lexer input.
const BUF_SIZE: usize = 12;

pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    And,
    At,
    Between,
    BetweenAnd,
    Boolean(bool),
    BuiltInTypeName(Name),
    Colon,
    Comma,
    Context,
    Div,
    Dot,
    Ellipsis,
    Else,
    Eof,
    Eq,
    Every,
    Exp,
    External,
    For,
    Function,
    LeftBrace,
    LeftBracket,
    LeftParen,
    Le,
    Lt,
    Ge,
    Gt,
    If,
    In,
    Instance,
    List,
    Minus,
    Mul,
    Not,
    Name(Name),
    NameDateTime(Name),
    Ne,
    Null,
    Numeric(String, String, char, String),
    Of,
    Or,
    Plus,
    Range,
    Return,
    RightArrow,
    RightBrace,
    RightBracket,
    RightParen,
    Satisfies,
    Some,
    String(String),
    Then,
    Undefined,
}

/// Lexer for FEEL language.
pub struct Lexer {
    /// Input characters.
    input: Vec<char>,
    /// Current cursor position in input vector.
    position: usize,
    /// Next token.
    next_token: Option<Token>,
}

impl Lexer {
    /// Creates a new lexer for specified input.
    pub fn new(input: impl AsRef<str>) -> Self {
        Self {
            input: input.as_ref().chars().collect(),
            position: 0,
            next_token: None,
        }
    }

    /// Returns next token.
    ///
    /// # Errors
    ///
    /// //TODO Describe errors
    pub fn next_token(&mut self) -> Result<Token> {
        self.next_token
            .take()
            .map_or_else(|| self.read_next_token(), Ok)
    }

    /// Peeks the next token.
    ///
    /// # Errors
    ///
    /// //TODO Describe errors
    pub fn peek(&mut self) -> Result<Token> {
        if let Some(token) = &self.next_token {
            Ok(token.clone())
        } else {
            let token = self.read_next_token()?;
            self.next_token = Some(token.clone());
            Ok(token)
        }
    }

    /// Reads the next token starting from current position.
    #[allow(clippy::too_many_lines)]
    fn read_next_token(&mut self) -> Result<Token> {
        let chars = self.read_input();
        match chars {
            ['s', 'a', 't', 'i', 's', 'f', 'i', 'e', 's', WS, _, _] => {
                self.position += 9;
                Ok(Token::Satisfies)
            }
            ['e', 'x', 't', 'e', 'r', 'n', 'a', 'l', WS, _, _, _] => {
                self.position += 8;
                Ok(Token::External)
            }
            ['f', 'u', 'n', 'c', 't', 'i', 'o', 'n', _, _, _, _]
                if self.is_function_separator(8) =>
            {
                self.position += 8;
                Ok(Token::Function)
            }
            ['i', 'n', 's', 't', 'a', 'n', 'c', 'e', WS, _, _, _] => {
                self.position += 8;
                Ok(Token::Instance)
            }
            ['b', 'e', 't', 'w', 'e', 'e', 'n', WS, _, _, _, _] => {
                self.position += 7;
                Ok(Token::Between)
            }
            ['c', 'o', 'n', 't', 'e', 'x', 't', _, _, _, _, _] if self.is_context_separator(7) => {
                self.position += 7;
                Ok(Token::Context)
            }
            ['r', 'e', 't', 'u', 'r', 'n', WS, _, _, _, _, _] => {
                self.position += 6;
                Ok(Token::Return)
            }
            ['e', 'v', 'e', 'r', 'y', WS, _, _, _, _, _, _] => {
                self.position += 5;
                Ok(Token::Every)
            }
            ['f', 'a', 'l', 's', 'e', ch, _, _, _, _, _, _] if is_separator(ch) => {
                self.position += 5;
                Ok(Token::Boolean(false))
            }
            ['r', 'a', 'n', 'g', 'e', _, _, _, _, _, _, _] if self.is_range_separator(5) => {
                self.position += 5;
                Ok(Token::Range)
            }
            ['n', 'u', 'l', 'l', ch, _, _, _, _, _, _, _] if is_separator(ch) => {
                self.position += 4;
                Ok(Token::Null)
            }
            ['e', 'l', 's', 'e', WS, _, _, _, _, _, _, _] => {
                self.position += 4;
                Ok(Token::Else)
            }
            ['l', 'i', 's', 't', _, _, _, _, _, _, _, _] if self.is_list_separator(4) => {
                self.position += 4;
                Ok(Token::List)
            }
            ['s', 'o', 'm', 'e', WS, _, _, _, _, _, _, _] => {
                self.position += 4;
                Ok(Token::Some)
            }
            ['t', 'h', 'e', 'n', WS, _, _, _, _, _, _, _] => {
                self.position += 4;
                Ok(Token::Then)
            }
            ['t', 'r', 'u', 'e', ch, _, _, _, _, _, _, _] if is_separator(ch) => {
                self.position += 4;
                Ok(Token::Boolean(true))
            }
            ['a', 'n', 'd', WS, _, _, _, _, _, _, _, _] => {
                self.position += 3;
                Ok(Token::And)
            }
            ['f', 'o', 'r', WS, _, _, _, _, _, _, _, _] => {
                self.position += 3;
                Ok(Token::For)
            }
            ['n', 'o', 't', _, _, _, _, _, _, _, _, _] => {
                self.position += 3;
                Ok(Token::Not)
            }
            ['i', 'f', WS, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::If)
            }
            ['i', 'n', WS, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::In)
            }
            ['o', 'f', WS, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Of)
            }
            ['o', 'r', WS, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Or)
            }
            ['.', '.', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Ellipsis)
            }
            ['*', '*', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Exp)
            }
            ['!', '=', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Ne)
            }
            ['<', '=', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Le)
            }
            ['>', '=', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::Ge)
            }
            ['-', '>', _, _, _, _, _, _, _, _, _, _] => {
                self.position += 2;
                Ok(Token::RightArrow)
            }
            ['.', ch, _, _, _, _, _, _, _, _, _, _] if ch.is_ascii_digit() => {
                Ok(self.consume_number())
            }
            ['.', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Dot)
            }
            [',', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Comma)
            }
            [':', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Colon)
            }
            ['"', _, _, _, _, _, _, _, _, _, _, _] => self.consume_string(),
            ['+', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Plus)
            }
            ['-', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Minus)
            }
            ['*', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Mul)
            }
            ['/', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Div)
            }
            ['=', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Eq)
            }
            ['<', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Lt)
            }
            ['>', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::Gt)
            }
            ['(', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::LeftParen)
            }
            [')', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::RightParen)
            }
            ['[', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::LeftBracket)
            }
            [']', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::RightBracket)
            }
            ['{', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::LeftBrace)
            }
            ['}', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::RightBrace)
            }
            ['@', _, _, _, _, _, _, _, _, _, _, _] => {
                self.position += 1;
                Ok(Token::At)
            }
            [ch, _, _, _, _, _, _, _, _, _, _, _] if ch.is_ascii_digit() => {
                Ok(self.consume_number())
            }
            [ch, _, _, _, _, _, _, _, _, _, _, _] if is_name_start(ch) => Ok(self.consume_name()),
            [WS, WS, WS, WS, WS, WS, WS, WS, WS, WS, WS, WS] => Ok(Token::Eof),
            _ => Ok(Token::Undefined),
        }
    }

    /// Reads characters from input.
    fn read_input(&mut self) -> [char; BUF_SIZE] {
        self.consume_whitespace();
        loop {
            let consumed_comment = self.consume_comment();
            self.consume_whitespace();
            if !consumed_comment {
                break;
            }
        }
        let mut buffer = [WS; BUF_SIZE];
        for (offset, value) in buffer.iter_mut().enumerate() {
            if let Some(ch) = self.char_at(offset)
                && !is_whitespace(ch)
            {
                *value = ch;
            }
        }
        buffer
    }

    /// Consumes all whitespace characters starting from the current position.
    /// After consuming a whitespace character the current position is advanced.
    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.char_at(0) {
            if is_whitespace(ch) {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    /// Consumes single comment starting from the current position.
    /// After consuming a comment, the current position is advanced
    /// to the next character after comment.
    fn consume_comment(&mut self) -> bool {
        let pair = (self.char_at(0), self.char_at(1));
        match pair {
            (Some('/'), Some('/')) => {
                self.position += 2;
                while let Some(ch) = self.char_at(0) {
                    if ch == '\n' {
                        return true;
                    }
                    self.position += 1;
                }
            }
            (Some('/'), Some('*')) => {
                self.position += 2;
                while let Some(ch) = self.char_at(0) {
                    if ch == '*' && self.char_at(1) == Some('/') {
                        self.position += 2;
                        return true;
                    }
                    self.position += 1;
                }
            }
            _ => {}
        }
        false
    }

    /// Consumes the string literal.
    fn consume_string(&mut self) -> Result<Token> {
        let mut string = String::new();
        self.position += 1;
        loop {
            let first = self.char_at(0);
            let second = self.char_at(1);
            match (first, second) {
                (Some('\\'), Some('\'')) => {
                    string.push('\'');
                    self.position += 2;
                }
                (Some('\\'), Some('"')) => {
                    string.push('"');
                    self.position += 2;
                }
                (Some('\\'), Some('\\')) => {
                    string.push('\\');
                    self.position += 2;
                }
                (Some('\\'), Some('n')) => {
                    string.push('\n');
                    self.position += 2;
                }
                (Some('\\'), Some('r')) => {
                    string.push('\r');
                    self.position += 2;
                }
                (Some('\\'), Some('t')) => {
                    string.push('\t');
                    self.position += 2;
                }
                (Some('\\'), Some('u' | 'U')) => {
                    string.push(self.consume_unicode()?);
                }
                (Some('"'), _) => {
                    self.position += 1;
                    break;
                }
                (Some(ch), _) if is_vspace(ch) => {
                    return Ok(Token::Undefined);
                }
                (Some(ch), _) => {
                    string.push(ch);
                    self.position += 1;
                }
                _ => return Ok(Token::Eof),
            }
        }
        Ok(Token::String(string))
    }

    /// Consumes a number available on input starting from the current position.
    /// When the number is consumed, the current position is incremented accordingly.
    fn consume_number(&mut self) -> Token {
        // storage for digits before decimal separator
        let mut digits_before = String::new();
        // storage for digits after decimal separator
        let mut digits_after = String::new();
        // storage for the sign of the exponent ('+' is the default)
        let mut exponent_sign: char = '+';
        // storage for digits of the exponent
        let mut digits_exponent = String::new();
        //------------------------------------------------------------------------------
        // Parse number using a state machine.
        //------------------------------------------------------------------------------
        let mut state = 1;
        while let Some(ch) = self.char_at(0) {
            match state {
                1 => match ch {
                    '0'..='9' => {
                        // consume a digit before decimal separator
                        digits_before.push(ch);
                        self.position += 1;
                    }
                    '.' if self.is_digit_at(1) => {
                        // consume decimal separator
                        self.position += 1;
                        state = 2;
                    }
                    'e' | 'E' => {
                        // consume exponent letter
                        self.position += 1;
                        state = 3;
                    }
                    _ => break,
                },
                2 => match ch {
                    '0'..='9' => {
                        // consume a digit after decimal separator
                        digits_after.push(ch);
                        self.position += 1;
                    }
                    'e' | 'E' => {
                        // consume exponent letter
                        self.position += 1;
                        state = 3;
                    }
                    _ => break,
                },
                3 => match ch {
                    '+' => {
                        // consume plus - this is the default exponent sign
                        self.position += 1;
                        state = 4;
                    }
                    '-' => {
                        // consume minus - change the exponent sign
                        exponent_sign = '-';
                        self.position += 1;
                        state = 4;
                    }
                    '0'..='9' => {
                        // consume the first digit of the exponent
                        digits_exponent.push(ch);
                        self.position += 1;
                        state = 4;
                    }
                    _ => break,
                },
                4 => match ch {
                    '0'..='9' => {
                        // consume digits of the exponent
                        digits_exponent.push(ch);
                        self.position += 1;
                        state = 4;
                    }
                    _ => break,
                },
                _ => break,
            }
        }
        if digits_before.is_empty() {
            digits_before.push('0');
        }
        Token::Numeric(digits_before, digits_after, exponent_sign, digits_exponent)
    }

    /// Consumes a name.
    fn consume_name(&mut self) -> Token {
        // currently parsed name part
        let mut current_part = String::new();

        //------------------------------------------------------------------------------
        // The current character on input is already a `name start` character;
        // consume it and advance the input position.
        //------------------------------------------------------------------------------
        let mut ch = self.peek_character();
        current_part.push(ch);
        self.position += 1;

        //------------------------------------------------------------------------------
        // The first character of the name was just consumed.
        // Now start parsing the rest of the input using a state machine,
        // looking for characters belonging to this name.
        //------------------------------------------------------------------------------
        while self.is_next_name_part_char() {
            self.position += 1;
            ch = self.peek_character();
            current_part.push(ch);
        }

        //------------------------------------------------------------------------------
        // Return the name as it appears on input.
        //------------------------------------------------------------------------------
        Token::Name(current_part)
    }

    /// Peeks the current character from input.
    fn peek_character(&self) -> char {
        self.char_at(0).unwrap()
    }

    /// Consumes Unicode literal in one of the following forms:
    /// - \u0000 ('\' + 'u' + four hexadecimal characters), or
    /// - \U000000 ('\' + 'U' + six hexadecimal characters).
    fn consume_unicode_literal(&mut self) -> Result<u64> {
        let u = self.char_at(1).unwrap();
        self.position += 2;
        let mut value = 0_u64;
        if u == 'U' {
            value = 1_048_576 * self.consume_hex_digit()?;
            value += 65536 * self.consume_hex_digit()?;
        }
        value += 4096 * self.consume_hex_digit()?;
        value += 256 * self.consume_hex_digit()?;
        value += 16 * self.consume_hex_digit()?;
        value += self.consume_hex_digit()?;
        Ok(value)
    }

    /// Consumes the UTF-8 encoded character given in one of the following form:
    /// - \u0000 ('\' + 'u' + four hexadecimal characters), or
    /// - \U000000 ('\' + 'U' + six hexadecimal characters).
    fn consume_unicode(&mut self) -> Result<char> {
        let mut value = self.consume_unicode_literal()?;
        match value {
            // one byte UTF-8 value
            0x0000..=0x007F => {
                let b1 = (value & 0x7F) as u8;
                let s = String::from_utf8(vec![b1]).unwrap();
                Ok(s.chars().next().unwrap())
            }
            // two bytes UTF-8 value
            0x0080..=0x07FF => {
                let b2 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b1 = ((value & 0x1F) as u8) | 0xC0;
                let s = String::from_utf8(vec![b1, b2]).unwrap();
                Ok(s.chars().next().unwrap())
            }
            // three bytes UTF-8 value
            0x0800..=0xD7FF | 0xE000..=0xFFFF => {
                let b3 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b2 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b1 = ((value & 0xF) as u8) | 0xE0;
                let s = String::from_utf8(vec![b1, b2, b3]).unwrap();
                Ok(s.chars().next().unwrap())
            }
            // four bytes UTF-8 value
            0x10000..=0x10_FFFF => {
                let b4 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b3 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b2 = ((value & 0x3F) as u8) | 0x80;
                value >>= 6;
                let b1 = ((value & 0x7) as u8) | 0xF0;
                let s = String::from_utf8(vec![b1, b2, b3, b4]).unwrap();
                Ok(s.chars().next().unwrap())
            }
            // value is the high surrogate of UTF-16
            0xD800..=0xDBFF => {
                let low_surrogate = self.consume_unicode_literal()?;
                match low_surrogate {
                    0xDC00..=0xDFFF => {
                        let mut code_point =
                            0x10000 + ((value - 0xD800) * 0x400) + (low_surrogate - 0xDC00);
                        let b4 = ((code_point & 0xFF) as u8) | 0x80;
                        code_point >>= 6;
                        let b3 = ((code_point & 0x3F) as u8) | 0x80;
                        code_point >>= 6;
                        let b2 = ((code_point & 0x3F) as u8) | 0x80;
                        code_point >>= 6;
                        let b1 = ((code_point & 0x7) as u8) | 0xF0;
                        let s = String::from_utf8(vec![b1, b2, b3, b4]).unwrap();
                        Ok(s.chars().next().unwrap())
                    }
                    other => Err(err_unicode_surrogate_out_of_range(other)),
                }
            }
            other => Err(err_unicode_value_out_of_range(other)),
        }
    }

    /// Consumes a HEX digit from input or reports an error.
    fn consume_hex_digit(&mut self) -> Result<u64> {
        if let Some(ch) = self.char_at(0) {
            if ch.is_ascii_hexdigit() {
                self.position += 1;
                hex_to_decimal(ch)
            } else {
                Err(err_expected_hex_digit(ch))
            }
        } else {
            Err(err_unexpected_eof())
        }
    }

    /// Checks if the next character is the name part character.
    fn is_next_name_part_char(&self) -> bool {
        self.char_at(1).is_some_and(is_name_part)
    }

    /// Returns the character at the current position advanced with specified offset.
    fn char_at(&self, offset: usize) -> Option<char> {
        if self.position + offset < self.input.len() {
            Some(self.input[self.position + offset])
        } else {
            None
        }
    }

    /// Returns `true` when the character at the current
    /// position advanced by the offset is a digit.
    fn is_digit_at(&self, offset: usize) -> bool {
        self.char_at(offset).is_some_and(|ch| ch.is_ascii_digit())
    }

    /// Returns `true` when the specified character is a function keyword separator,
    /// that is a character that is allowed after keyword `function`.
    fn is_function_separator(&self, offset: usize) -> bool {
        self.is_next_character(&['(', '<'], offset)
    }

    /// Returns `true` when the specified character is a context keyword separator,
    /// that is a character that is allowed after keyword `context`.
    fn is_context_separator(&self, offset: usize) -> bool {
        self.is_next_character(&['<'], offset)
    }

    /// Returns `true` when the next character on input is a character
    /// from specified `chars` list.
    fn is_next_character(&self, chars: &[char], mut offset: usize) -> bool {
        while let Some(ch) = self.char_at(offset) {
            if chars.contains(&ch) {
                return true;
            } else if !is_whitespace(ch) {
                return false;
            }
            offset += 1;
        }
        false
    }

    /// Returns `true` when the specified character is a range keyword separator,
    /// that is a character that is allowed after keyword `range`.
    fn is_range_separator(&self, offset: usize) -> bool {
        self.is_next_character(&['<'], offset)
    }

    /// Returns `true` when the next character on input is a `list` keyword separator,
    /// that is, a character that is allowed after keyword `list`.
    /// Allowed characters after keyword `list` are whitespace and `<`.
    fn is_list_separator(&self, offset: usize) -> bool {
        self.is_next_character(&['<'], offset)
    }
}

/// Returns `true` when the specified character is a separator (white space equivalent).
#[rustfmt::skip]
const fn is_separator(ch: char) -> bool {
    matches!(ch,
      ' ' | '=' | '!' | '<' | '>' | '+' | '-' | '*' |
      '/' | '%' | '.' | ',' | ')' | '[' | ']' | '}' )
}

/// Returns `true` when the specified character is name start character.
#[rustfmt::skip]
const fn is_name_start(ch: char) -> bool {
    matches!(ch,
      '?' | 'A'..='Z' | '_' | 'a'..='z' |
      '\u{00C0}'..='\u{00D6}' | '\u{00D8}'..='\u{00F6}' | '\u{00F8}'..='\u{02FF}' |
      '\u{0370}'..='\u{037D}' | '\u{037F}'..='\u{1FFF}' | '\u{200C}'..='\u{200D}' |
      '\u{2070}'..='\u{218F}' | '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' |
      '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' | '\u{10000}'..='\u{EFFFF}')
}

/// Returns `true` when the specified character is name part character.
#[rustfmt::skip]
const fn is_name_part(ch: char) -> bool {
    is_name_start(ch) ||
    ch.is_ascii_digit() ||
    matches!(ch, '\u{00B7}' | '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}')
}

/// Returns `true` when the specified character is a whitespace character.
#[rustfmt::skip]
const fn is_whitespace(ch: char) -> bool {
    matches!(ch,
      '\u{0009}' | '\u{000A}'..='\u{000D}' | '\u{0020}' | '\u{0085}' | '\u{00A0}' |
      '\u{1680}' | '\u{180E}' | '\u{2000}'..='\u{200B}' | '\u{2028}' | '\u{2029}' |
      '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}'
    )
}

/// Returns `true` when the specified character is a vertical space character.
const fn is_vspace(ch: char) -> bool {
    matches!(ch, '\u{000A}'..='\u{000D}')
}

/// Return the decimal value that corresponds to hexadecimal digit, case-insensitive.
fn hex_to_decimal(ch: char) -> Result<u64> {
    match ch {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'A' | 'a' => Ok(10),
        'B' | 'b' => Ok(11),
        'C' | 'c' => Ok(12),
        'D' | 'd' => Ok(13),
        'E' | 'e' => Ok(14),
        'F' | 'f' => Ok(15),
        other => Err(err_expected_hex_digit(other)),
    }
}
