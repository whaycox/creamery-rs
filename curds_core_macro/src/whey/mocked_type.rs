use super::*;

pub struct WheyMockedType {
    pub mocked_trait: Path,
    pub whey_name: Path,
    pub core_name: Path,
}

impl Parse for WheyMockedType {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Option<Token![dyn]>>()?;
        let mocked_trait: Path = input.parse()?;
        let whey_name = Self::generate_whey_name(&mocked_trait);
        let core_name = Self::generate_core_name(&mocked_trait);
        
        Ok(WheyMockedType {
            mocked_trait,
            whey_name,
            core_name,
        })
    }
}

impl WheyMockedType {
    pub fn generate_whey_name(mocked_trait: &Path) -> Path {
        let mut mocked_trait_segments = mocked_trait.segments.clone();
        let final_index = mocked_trait_segments.len() - 1;
        let type_segment = &mocked_trait_segments[final_index];
        let new_type_ident = format_ident!("Whey{}", type_segment.ident);
        mocked_trait_segments[final_index].ident = new_type_ident;
        Path {
            leading_colon: None,
            segments: mocked_trait_segments,
        }
    }
    pub fn generate_core_name(mocked_trait: &Path) -> Path {
        let mut mocked_trait_segments = mocked_trait.segments.clone();
        let final_index = mocked_trait_segments.len() - 1;
        let type_segment = &mocked_trait_segments[final_index];
        let new_type_ident = format_ident!("WheyCore{}", type_segment.ident);
        mocked_trait_segments[final_index].ident = new_type_ident;
        Path {
            leading_colon: None,
            segments: mocked_trait_segments,
        }
    }
}
