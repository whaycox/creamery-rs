use super::*;

const MESSAGE_KEYWORD: &str = "message";

#[derive(Clone)]
pub enum StageReturn {
    None,
    Explicit(Type),
    Message,
    Request,
    Response,
}

impl StageReturn {
    pub fn parse_message(keyword: Ident) -> Result<Self> {
        if keyword == Ident::new(MESSAGE_KEYWORD, Span::call_site()) {
            Ok(StageReturn::Message)
        }
        else {
            Err(Error::new(keyword.span(), format!("Unexpected keyword: {}", keyword)))
        }
    }
}