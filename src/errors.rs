
#[derive(Debug, Clone)]
pub struct ParseSignatureError;

#[derive(Debug, Clone)]
pub struct ParseArgError;

#[derive(Debug, Clone, PartialEq)]
pub struct UnmatchedParenthesisError;


impl From<ParseArgError> for ParseSignatureError {
    fn from(_e: ParseArgError) -> ParseSignatureError {
        ParseSignatureError
    }
}

impl From<UnmatchedParenthesisError> for ParseArgError {
    fn from(_e: UnmatchedParenthesisError) -> ParseArgError {
        ParseArgError
    }
}
