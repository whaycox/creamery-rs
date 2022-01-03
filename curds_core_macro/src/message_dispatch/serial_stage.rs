use super::*;

#[derive(Clone, Debug)]
pub struct SerialStage {
    name: Ident,
    pub output: Option<Type>,
}
impl SerialStage {
    pub fn new(name: Ident, output: Option<Type>) -> Self {
        Self {
            name: name,
            output: output,
        }
    }

    pub fn output(&self) -> Option<Type> { self.output.clone() }

    pub fn trait_name(&self, base_name: &Ident) -> Ident {
        format_ident!("{}{}", base_name, self.name)
    }
}

impl Parse for SerialStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let output: Option<Type> = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Some(input.parse()?)
        }
        else {
            None
        };

        Ok(Self {
            name: name,
            output: output,
        })
    }
}