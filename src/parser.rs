use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid utf8 character in beacon data")]
    Invalid,
    #[error("Unexpected top header (expected: HTTP/1.1 200 OK, found: {0})")]
    UnexpectedHeader(String),
    #[error("Unable to parse header, no key/value found")]
    UnexpectedEnd
}

pub fn parser(data: &[u8]) -> Result<HashMap<String,String>, ParserError>  {
    let data = std::str::from_utf8(data);
    let data = match data {
        Ok(data) => data,
        Err(_) => return Err(ParserError::Invalid),
    };

    let mut lines = data.lines();

    let protocol_header = lines.next();

    if  protocol_header != Some("HTTP/1.1 200 OK") && protocol_header != Some("NOTIFY * HTTP/1.1") {
        return Err(ParserError::UnexpectedHeader(protocol_header.unwrap_or("None").to_owned()))
    }

    let mut headers = HashMap::new();

    for line in lines {

        let mut l_iter = line.splitn(2, ": ");

        let key = l_iter.next().ok_or(ParserError::UnexpectedEnd)?.to_owned();
        let value = l_iter.next().ok_or(ParserError::UnexpectedEnd)?.to_owned();

        headers.insert(key, value);

    }

    Ok(headers)

}
