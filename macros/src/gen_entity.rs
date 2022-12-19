use {crate::utils::DomainDTOAst,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn gen_entity(ast: &DomainDTOAst) -> TokenStream {
  let entity_name_ident = format_ident!("{}Entity", ast.root_name_ident);
  let entity_builder_name_ident = format_ident!("{}Builder", entity_name_ident);
  let entity_fields = ast.fields_named
                         .named
                         .iter()
                         .map(|f| {
                           let name = &f.ident;
                           let name = name.as_ref().unwrap();
                           let is_option = super::utils::is_type_option(&f.ty);
                           let value_type = format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));
                           let ts = if is_option {
                             quote! {
                               #name: Option<#value_type>,
                             }
                           } else {
                             quote! {
                               #name: #value_type,
                             }
                           };
                           ts
                         })
                         .collect::<Vec<_>>();
  let builder_fields = ast.fields_named
                          .named
                          .iter()
                          .map(|f| {
                            let name = &f.ident;
                            let name = name.as_ref().unwrap();
                            let is_option = super::utils::is_type_option(&f.ty);
                            let value_type = format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));
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
  let entity_getters_and_setters =
    ast.fields_named.named.iter().map(|f| {
                                   let name = &f.ident;
                                   let name = name.as_ref().unwrap();
                                   let is_option = super::utils::is_type_option(&f.ty);
                                   let value_type = format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));
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
                             let value_type = format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));

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

                          quote! {
                            let #name = self.#name.ok_or_else(|| format!("{}::{} not set", stringify!(#entity_name_ident), stringify!(#name)))?;
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
      pub fn build(self) -> std::result::Result<#entity_name_ident, String> {
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
