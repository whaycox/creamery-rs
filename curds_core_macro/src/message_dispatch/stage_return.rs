use super::*;

#[derive(Clone)]
pub enum StageReturn {
    None,
    Message,
    Explicit(Type)
}

impl Parse for StageReturn {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            if input.parse::<Ident>()?.to_string().as_str() == "message" {
                Ok(Self::Message)
            }
            else { 
                panic!("Unrecognized request template keyword") 
            }
        }
        else {
            Ok(Self::Explicit(input.parse()?))
        }
    }
}