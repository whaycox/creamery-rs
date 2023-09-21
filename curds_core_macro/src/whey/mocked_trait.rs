use super::*;

pub enum WheyMockedTrait {
    Transient(MockedTraitDefinition),
    Singleton(MockedTraitDefinition),
}
impl WheyMockedTrait {
    pub fn quote_attribute_generator(&self) -> TokenStream {
        match self {
            WheyMockedTrait::Transient(transient) => {
                let mocked_trait = &transient.mocked_trait;
                let whey_name = &transient.whey_name;
                let core_name = &transient.core_name;
                quote! {
                    #[generates(dyn #mocked_trait ~ #whey_name)] 
                    #[generates_singleton(#core_name)] 
                }
            },
            WheyMockedTrait::Singleton(singleton) => {
                let mocked_trait = &singleton.mocked_trait;
                let whey_name = &singleton.whey_name;
                quote! { #[generates_singleton(dyn #mocked_trait ~ #whey_name)] }
            },
        }
    }

    pub fn quote_assert(&self) -> TokenStream {
        let core_name = match self {
            WheyMockedTrait::Transient(transient) => &transient.core_name,
            WheyMockedTrait::Singleton(singleton) => &singleton.core_name,
        };

        quote! {
            {
                let core = curds_core_abstraction::dependency_injection::ServiceGenerator::<Rc<RwLock<#core_name>>>::generate(self);
                core.write().unwrap().assert();
            }
        }
    }
}

pub struct MockedTraitDefinition {
    pub mocked_trait: Path,
    pub whey_name: Path,
    pub core_name: Path,
}

impl Parse for MockedTraitDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Option<Token![dyn]>>()?;
        let mocked_trait: Path = input.parse()?;
        let whey_name = Self::generate_whey_name(&mocked_trait);
        let core_name = Self::generate_core_name(&mocked_trait);
        
        Ok(MockedTraitDefinition {
            mocked_trait,
            whey_name,
            core_name,
        })
    }
}

impl MockedTraitDefinition {
    pub fn generate_whey_name(mocked_trait: &Path) -> Path {
        let mut mocked_trait_segments = mocked_trait.segments.clone();
        let final_index = mocked_trait_segments.len() - 1;
        let type_segment = &mocked_trait_segments[final_index];
        let new_type_ident = WheyMockCore::whey_name(&type_segment.ident);
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
        let new_type_ident = WheyMockCore::core_name(&type_segment.ident);
        mocked_trait_segments[final_index].ident = new_type_ident;
        Path {
            leading_colon: None,
            segments: mocked_trait_segments,
        }
    }
}
