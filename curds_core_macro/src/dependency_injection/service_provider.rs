use super::*;

pub struct ServiceProviderDefinition {
    item: ItemStruct,
    library: Vec<ServiceProduction>,
    singletons: SingletonCollection,
}
impl Parse for ServiceProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let library = Self::parse_productions(&mut item)?;
        let mut singletons = SingletonCollection::new();
        singletons.register_singletons(&item, &library)?;
        singletons.add_singletons(&mut item)?;

        Ok(Self {
            item: item,
            library: library,
            singletons: singletons,
        })
    }
}

impl ServiceProviderDefinition {
    pub fn name(&self) -> &Ident { &self.item.ident }
    pub fn visibility(&self) -> &Visibility { &self.item.vis }
    pub fn generics(&self) -> &Generics { &self.item.generics }
    pub fn lifetimes(&self) -> Vec<&LifetimeDef> { self.item.generics.lifetimes().collect() }
    pub fn singleton(&self, ty: &Type) -> Ident { self.singletons.singleton(ty) }
    pub fn provider(&self, name: &Ident) -> Type {
        for field in &self.item.fields {
            if field.ident.clone().unwrap().to_string() == name.to_string() {
                return field.ty.clone()
            }
        }
        panic!("no provider found");
    }

    pub fn item(&mut self) -> &mut ItemStruct { &mut self.item }
    
    pub fn add_production(&mut self, transient_type: Type) {
        self.library.push(ServiceProduction::GenerateTransient(transient_type.into()))
    }

    fn parse_productions(item: &mut ItemStruct) -> Result<Vec<ServiceProduction>> {
        let mut parsed: Vec<ServiceProduction> = Vec::new();
        let length = item.attrs.len();
        if length > 0 {
            let mut attribute_index = length - 1;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                    parsed.push(ServiceProduction::GenerateTransient(attribute.parse_args::<GeneratedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                    parsed.push(ServiceProduction::GenerateSingleton(attribute.parse_args::<GeneratedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(CLONES_SELF_IDENTIFIER) {
                    parsed.push(ServiceProduction::CloneSelf());
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(CLONES_IDENTIFIER) {
                    parsed.push(ServiceProduction::Clone(attribute.parse_args::<ProviderDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                    parsed.push(ServiceProduction::ForwardTransient(attribute.parse_args::<ForwardedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                    parsed.push(ServiceProduction::ForwardSingleton(attribute.parse_args::<ForwardedDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(SCOPES_SELF_IDENTIFIER) {
                    parsed.push(ServiceProduction::ScopeSelf());
                    item.attrs.remove(attribute_index);
                }
                else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                    parsed.push(ServiceProduction::ScopeTransient(attribute.parse_args::<ProviderDefinition>()?));
                    item.attrs.remove(attribute_index);
                }
    
                if attribute_index == 0 {
                    break;
                }
                attribute_index = attribute_index - 1;
            }
            parsed.reverse();
        }

        Ok(parsed)
    }

    pub fn quote(&self) -> TokenStream {
        let mut library: Vec<TokenStream> = Vec::new();
        for production in &self.library {
            library.push(production.quote(&self));
        }
        let item = &self.item;

        quote! {
            #[injected]
            #item
            #(#library)*
        }
    }
}