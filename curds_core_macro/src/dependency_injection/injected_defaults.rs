use super::*;

const DEFAULTED_IDENTIFIER: &str = "defaults";

pub struct DefaultedFields {
    pub ident: Ident,
}
impl DefaultedFields {
    pub fn parse_defaults(input: ParseStream) -> Result<Vec<DefaultedFields>> {
        let mut fields: Vec<DefaultedFields> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(DEFAULTED_IDENTIFIER) {
                fields.push(DefaultedFields {
                    ident: attribute.parse_args()?,
                })
            }
        }
        
        Ok(fields)
    }
}