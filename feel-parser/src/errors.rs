//! Parser errors

pub type Result<T, E = ParserError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ParserError(pub String);

pub fn err_unexpected_eof() -> ParserError {
    ParserError("unexpected end of file".to_string())
}

pub fn err_expected_hex_digit(ch: char) -> ParserError {
    ParserError(format!(
        "expected hex digit but encountered '{}'",
        ch.escape_debug()
    ))
}

pub fn err_unicode_value_out_of_range(value: u64) -> ParserError {
    ParserError(format!(
        "value is out of allowed Unicode range 0x0000..0x10FFFF: {value:X}"
    ))
}

pub fn err_unicode_surrogate_out_of_range(value: u64) -> ParserError {
    ParserError(format!(
        "surrogate value is out of allowed range 0xD800..0xDFFF: {value:X}"
    ))
}
