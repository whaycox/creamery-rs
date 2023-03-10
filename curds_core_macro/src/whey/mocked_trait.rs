use super::*;

pub enum WheyMockedTrait {
    Transient(MockedTraitDefinition),
    Singleton(MockedTraitDefinition),
}
impl WheyMockedTrait {
    pub fn definition(&self) -> &MockedTraitDefinition {
        match self {
            WheyMockedTrait::Transient(transient) => &transient,
            WheyMockedTrait::Singleton(singleton) => &singleton,
        }
    }

    pub fn quote_attribute_generator(&self) -> TokenStream {
        match self {
            WheyMockedTrait::Transient(transient) => {
                let mocked_trait = &transient.mocked_trait;
                let whey_name = &transient.whey_name;
                quote! { #[generates(dyn #mocked_trait ~ #whey_name)] }
            },
            WheyMockedTrait::Singleton(singleton) => {
                let mocked_trait = &singleton.mocked_trait;
                let whey_name = &singleton.whey_name;
                quote! { #[generates_singleton(dyn #mocked_trait ~ #whey_name)] }
            },
        }
    }

    pub fn quote_core_reference(&self, context: &ItemStruct, identifier: &Ident) -> TokenStream {
        let definition = self.definition();
        let core_name = &definition.core_name;
        let reference_name = WheyMockCore::load_expectation_ident(definition.mocked_trait.get_ident().unwrap());

        quote! {
            pub fn #reference_name(&mut self) -> &mut #core_name {
                self.#identifier.get_mut()
            }
        }
    }

    pub fn quote_core_generator(&self, context: &ItemStruct, identifier: &Ident) -> TokenStream {
        let definition = self.definition();
        let core_name = &definition.core_name;
        let context_ident = &context.ident;
        let (impl_generics, type_generics, where_clause) = context.generics.split_for_impl();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::ServiceGenerator<#core_name> for #context_ident #type_generics #where_clause {
                fn generate(&mut self) -> #core_name {
                    self.#identifier.replace(#core_name::construct())
                }
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
