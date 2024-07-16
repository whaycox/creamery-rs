use super::*;

pub enum WheyMockedTrait {
    Transient(MockedTraitDefinition),
    Singleton(MockedTraitDefinition),
}

pub struct MockedTraitDefinition {
    pub mocked_trait: Path,
    pub whey_name: Path,
}

impl Parse for MockedTraitDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Option<Token![dyn]>>()?;
        let mocked_trait: Path = input.parse()?;
        let whey_name = Self::generate_testing_name(&mocked_trait);
        
        Ok(MockedTraitDefinition {
            mocked_trait,
            whey_name,
        })
    }
}

impl MockedTraitDefinition {
    pub fn generate_testing_name(mocked_trait: &Path) -> Path {
        let mut mocked_trait_segments = mocked_trait.segments.clone();
        let final_index = mocked_trait_segments.len() - 1;
        let type_segment = &mocked_trait_segments[final_index];
        let new_type_ident = WheyMock::testing_name(&type_segment.ident);
        mocked_trait_segments[final_index].ident = new_type_ident;
        Path {
            leading_colon: None,
            segments: mocked_trait_segments,
        }
    }
}
