use super::*;

pub const DEFAULTED_IDENTIFIER: &str = "defaults";

#[derive(Clone)]
pub struct DefaultedField {}
impl DefaultedField {
    pub fn parse_defaults(input: ParseStream) -> Result<Vec<Ident>> {
        let mut fields: Vec<Ident> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(DEFAULTED_IDENTIFIER) {
                fields.push(attribute.parse_args()?)
            }
        }
        
        Ok(fields)
    }
}