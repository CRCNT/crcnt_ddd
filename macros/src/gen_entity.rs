use {crate::{ast::value::DomainValueAttr,
             utils::DomainDefAst},
     convert_case::{Case,
                    Casing},
     proc_macro2::{Ident,
                   TokenStream},
     quote::{format_ident,
             quote,
             ToTokens},
     syn::Field};

pub fn gen_entity(ast: &DomainDefAst) -> TokenStream {
  let entity_name_ident = format_ident!("{}Entity", ast.root_name_ident);
  let entity_builder_name_ident = format_ident!("{}Builder", entity_name_ident);
  let entity_fields = ast.fields_named
                         .named
                         .iter()
                         .map(|f| {
                           let name = &f.ident;
                           let name = name.as_ref().unwrap();
                           let is_option = super::utils::is_type_option(&f.ty);
                           let value_type = value_type(&ast.root_name_ident, f);
                           if is_option {
                             quote! {
                               #name: Option<#value_type>,
                             }
                           } else {
                             quote! {
                               #name: #value_type,
                             }
                           }
                         })
                         .collect::<Vec<_>>();
  let builder_fields = ast.fields_named
                          .named
                          .iter()
                          .map(|f| {
                            let name = &f.ident;
                            let name = name.as_ref().unwrap();
                            let is_option = super::utils::is_type_option(&f.ty);
                            let value_type = value_type(&ast.root_name_ident, f);
                            if is_option {
                              quote! {
                                #name: Option<Option<#value_type>>,
                              }
                            } else {
                              quote! {
                                #name: Option<#value_type>,
                              }
                            }
                          })
                          .collect::<Vec<_>>();
  // getters
  let entity_getters_and_setters = ast.fields_named.named.iter().map(|f| {
                                                                  let name = &f.ident;
                                                                  let name = name.as_ref().unwrap();
                                                                  let is_option = super::utils::is_type_option(&f.ty);
                                                                  let value_type = value_type(&ast.root_name_ident, f);

                                                                  let getter_name = format_ident!("ref_{}", name);
                                                                  let mv_name = format_ident!("move_{}", name);
                                                                  let setter_name = format_ident!("set_{}", name);
                                                                  if is_option {
                                                                    quote! {
                                                                      pub fn #getter_name(&self) -> &Option<#value_type> {
                                                                        &self.#name
                                                                      }
                                                                      pub fn #mv_name(self) -> Option<#value_type> {
                                                                        self.#name
                                                                      }
                                                                      pub fn #setter_name(self, new_value: Option<#value_type>) -> Self {
                                                                        Self {
                                                                          #name: new_value,
                                                                          ..self
                                                                        }
                                                                      }
                                                                    }
                                                                  } else {
                                                                    quote! {
                                                                      pub fn #getter_name(&self) -> &#value_type {
                                                                        &self.#name
                                                                      }
                                                                      pub fn #mv_name(self) -> #value_type {
                                                                        self.#name
                                                                      }
                                                                      pub fn #setter_name(self, new_value: #value_type) -> Self {
                                                                        Self {
                                                                          #name: new_value,
                                                                          ..self
                                                                        }
                                                                      }
                                                                    }
                                                                  }
                                                                });
  let builder_setters = ast.fields_named
                           .named
                           .iter()
                           .map(|f| {
                             let name = &f.ident;
                             let name = name.as_ref().unwrap();
                             let is_option = super::utils::is_type_option(&f.ty);
                             let value_type = value_type(&ast.root_name_ident, f);
                             if is_option {
                               quote! {
                                 pub fn #name<T: Into<#value_type>>(mut self, new_value: Option<T>) -> Self {
                                   self.#name = Some(new_value.map(|x|x.into()));
                                   self
                                 }
                               }
                             } else {
                               quote! {
                                 pub fn #name<T: Into<#value_type>>(mut self, new_value: T) -> Self {
                                   self.#name = Some(new_value.into());
                                   self
                                 }
                               }
                             }
                           })
                           .collect::<Vec<_>>();

  let building_var = ast.fields_named
                        .named
                        .iter()
                        .map(|f| {
                          let name = &f.ident;
                          let name = name.as_ref().unwrap();
                          let error_msg = format!("{}::{} not set", entity_name_ident, name);
                          quote! {
                            let #name = self.#name.ok_or_else(|| #error_msg)?;
                          }
                        })
                        .collect::<Vec<_>>();

  let building_var_names = ast.fields_named
                              .named
                              .iter()
                              .map(|f| {
                                let name = &f.ident;
                                let name = name.as_ref().unwrap();

                                quote! {
                                  #name,
                                }
                              })
                              .collect::<Vec<_>>();

  quote! {
    #[derive(Debug, Clone)]
    pub struct #entity_name_ident {
      #(#entity_fields)*
    }
    #[derive(Default)]
    pub struct #entity_builder_name_ident {
      #(#builder_fields)*
    }

    // implementation for entity
    impl #entity_name_ident {
      pub fn builder() -> #entity_builder_name_ident {
        #entity_builder_name_ident::default()
      }
      #(#entity_getters_and_setters)*
    }

    // implementation for entity builders
    impl #entity_builder_name_ident {
      #[doc = "Build the entity, if some field is not set, then return error message to hint the missed field"]
      pub fn build(self) -> std::result::Result<#entity_name_ident, &'static str> {
        #(#building_var)*
        Ok(#entity_name_ident {
          #(#building_var_names)*
        })
      }

      #[doc = "Build the entity, if some field is not set, it will panic"]
      pub fn unsafe_build(self) -> #entity_name_ident {
        self.build().unwrap()
      }
      #(#builder_setters)*
    }
  }
}

fn value_type(root_name_ident: &Ident, f: &Field) -> Box<dyn ToTokens> {
  let name = &f.ident;
  let name = name.as_ref().unwrap();
  let skip = DomainValueAttr::parse_from(f).skip;
  let value_type: Box<dyn ToTokens> = if skip {
    Box::new(f.ty.clone())
  } else {
    Box::new(format_ident!("{}{}", root_name_ident, name.to_string().to_case(Case::Pascal)))
  };
  value_type
}
