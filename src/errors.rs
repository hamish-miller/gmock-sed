
#[derive(Debug, Clone)]
pub struct ParseSignatureError;

#[derive(Debug, Clone)]
pub struct ParseArgError;


impl From<ParseArgError> for ParseSignatureError {
    fn from(_e: ParseArgError) -> ParseSignatureError {
        ParseSignatureError
    }
}
