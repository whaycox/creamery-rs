use super::*;

pub struct MockedCollection {
    consumed: HashSet<Ident>,
    mocked_traits: HashMap<Path, MockedCollectionItem>,
}

impl MockedCollection {
    pub fn new() -> Self {
        Self {
            consumed: HashSet::new(),
            mocked_traits: HashMap::new(), 
        }
    }

    pub fn add(&mut self, mocked_trait: WheyMockedTrait) {
        let trait_path = mocked_trait.definition().mocked_trait.clone();
        if !self.mocked_traits.contains_key(&trait_path) {
            let identifier = self.unconsumed_identifier();
            self.consumed.insert(identifier.clone());
            self.mocked_traits.insert(trait_path, MockedCollectionItem { 
                identifier, 
                item: mocked_trait, 
            });
        }
    }
    fn unconsumed_identifier(&self) -> Ident {
        let mut generated = Self::generate_identifier();
        while self.consumed.contains(&generated) {
            generated = Self::generate_identifier();
        }

        generated
    }
    fn generate_identifier() -> Ident {
        let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
        let mut identifier = String::new();
        for byte in random_bytes {
            identifier.push_str(&format!("{:X}", byte));
        }

        format_ident!("whey_mock_core_{}", identifier)
    }


    pub fn add_fields(&self, item: &mut ItemStruct) {
        for collection_item in &self.mocked_traits {
            Self::add_field(collection_item.1, item)
        }
    }
    fn add_field(collection_item: &MockedCollectionItem, item: &mut ItemStruct) {
        match &mut item.fields {
            Fields::Named(named) => {
                let core_name = &collection_item.item.definition().core_name;
                let mut core_field = Field {
                    attrs: Default::default(),
                    vis: Visibility::Inherited,
                    ident: Some(collection_item.identifier.clone()),
                    colon_token: None,
                    ty: syn::parse2(quote! { std::cell::Cell<#core_name> }).unwrap(),
                };
                let defaulted_attribute: Attribute = Attribute { 
                    pound_token: Default::default(), 
                    style: AttrStyle::Outer, 
                    bracket_token: Default::default(), 
                    path: syn::parse2(quote! { defaulted }).unwrap(), 
                    tokens: quote! { (std::cell::Cell::new(#core_name::construct())) }, 
                };
                core_field.attrs.push(defaulted_attribute);
                named.named.push(core_field);
            },
            _ => panic!("Only named fields are supported"),
        }
    }

    pub fn quote_attributes(&self) -> Vec<TokenStream> {
        let generators: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|item| item.1.item.quote_attribute_generator())
            .collect();
        
        generators
    }

    pub fn quote_core_reference(&self, context: &ItemStruct) -> Vec<TokenStream> {
        let references: Vec<TokenStream> = self.mocked_traits
        .iter()
        .map(|item| item.1.item.quote_core_reference(context, &item.1.identifier))
        .collect();
    
        references
    }

    pub fn quote_core_generators(&self, context: &ItemStruct) -> Vec<TokenStream> {
        let generators: Vec<TokenStream> = self.mocked_traits
            .iter()
            .map(|item| item.1.item.quote_core_generator(context, &item.1.identifier))
            .collect();
        
        generators
    }
}

struct MockedCollectionItem {
    identifier: Ident,
    item: WheyMockedTrait,
}