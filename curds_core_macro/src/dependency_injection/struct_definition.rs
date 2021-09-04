use super::*;

#[derive(Clone)]
pub struct StructDefinition {
    visibility: Option<Token![pub]>,
    pub name: Ident,
    dependencies: RefCell<Vec<InjectedDependency>>,
    defaults: HashSet<Ident>,
}
impl StructDefinition {
    pub fn dependency_type(&self, name: &Ident) -> TokenStream {
        for dependency in self.dependencies.borrow().clone() {
            if dependency.eq(name) {
                return dependency.ty.clone()
            }
        }
        quote_spanned! { name.span() => compile_warning!("provider not found") }
    }

    pub fn add_dependencies(&mut self, explicit_dependencies: Vec<InjectedDependency>) {
        let mut dependencies = self.dependencies.take();
        for dependency in explicit_dependencies {
            dependencies.push(dependency)
        }
        self.dependencies.replace(dependencies);
    }

    pub fn parse(input: ParseStream) -> Result<Self> {
        let defaults: HashSet<Ident> = DefaultedField::parse_defaults(input)?
            .into_iter()
            .collect();
        let visibility: Option<Token![pub]> = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let dependencies = InjectedDependency::parse(content, &defaults)?;

        Ok(Self {
            visibility: visibility,
            name: name,
            dependencies: RefCell::new(dependencies),
            defaults: defaults,
        })
    }

    pub fn quote(self) -> TokenStream {
        let struct_tokens = self.clone().struct_tokens();
        let injected_tokens = self.clone().injected_tokens();
        let construct_tokens = self.construct_tokens();

        quote! {
            #struct_tokens
            #injected_tokens
            #construct_tokens
        }
    }
    fn struct_tokens(self) -> TokenStream {
        let visibility = self.visibility;
        let name = self.name;
        let struct_fields = self.dependencies
            .take()
            .into_iter()
            .map(|dependency| dependency.struct_tokens());

        quote! {
            #visibility struct #name {
                #(#struct_fields),*
            }
        }
    }
    fn injected_tokens(self) -> TokenStream {
        let name = self.name;
        let has_constraints = self.dependencies
            .borrow()
            .clone()
            .into_iter()
            .any(|field| !field.default);
        let constraint_tokens =
        if has_constraints {
            let mapped_constraints = self.dependencies
                .borrow()
                .clone()
                .into_iter()
                .filter_map(|dependency| if !dependency.default { Some(dependency.constraint_tokens()) } else { None });

            quote! { where TProvider : #(#mapped_constraints)+* }
        }
        else {
            quote! {}
        };
        let generator_tokens =
        if has_constraints {
            let mapped_generators = self.dependencies
                .take()
                .into_iter()
                .filter_map(|dependency| if !dependency.default { Some(dependency.generator_tokens()) } else { None });

            quote! { #(#mapped_generators),* }
        }
        else {
            quote! {}
        };
        
        quote! {
            impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider> for #name
            #constraint_tokens {
                fn inject(provider: &TProvider) -> std::rc::Rc<Self> {
                    Self::construct(#generator_tokens)
                }
            }
        }
    }
    fn construct_tokens(self) -> TokenStream {
        let name = self.name;
        let argument_tokens = self.dependencies
            .borrow()
            .clone()
            .into_iter()
            .filter_map(|dependency| if !dependency.default { Some(dependency.argument_tokens()) } else { None });
        let initializer_tokens = self.dependencies
            .take()
            .into_iter()
            .map(|dependency| dependency.initializer_tokens());

        quote! {
            impl #name {
                pub fn construct(#(#argument_tokens),*) -> std::rc::Rc<Self> {
                    std::rc::Rc::new(Self {
                        #(#initializer_tokens),*
                    })
                }
            }
        }
    }

    pub fn scope_tokens(self) -> TokenStream {
        let name = self.name;
        let initializer_tokens = self.dependencies
            .take()
            .into_iter()
            .map(|dependency| dependency.scope_tokens());

        quote! {
            impl curds_core_abstraction::dependency_injection::Scoped for #name {
                fn scope(&self) -> Self {
                    Self {
                        #(#initializer_tokens),*
                    }
                }
            }
        }
    }
}