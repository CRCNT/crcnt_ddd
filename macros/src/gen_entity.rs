use {crate::utils::DomainDTOAst,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn gen_entity(ast: &DomainDTOAst) -> TokenStream {
  let entity_name_ident = format_ident!("{}Entity", ast.root_name_ident);
  let entity_builder_name_ident = format_ident!("{}Builder", entity_name_ident);
  let recurse = ast.fields_named
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
  // getters
  let getters_and_setters = ast.fields_named.named.iter().map(|f| {
                                                           let name = &f.ident;
                                                           let name = name.as_ref().unwrap();
                                                           let is_option = super::utils::is_type_option(&f.ty);
                                                           let value_type =
                                                             format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));
                                                           let getter_name = format_ident!("ref_{}", name);
                                                           let setter_name = format_ident!("set_{}", name);
                                                           if is_option {
                                                             quote! {
                                                               pub fn #getter_name(&self) -> &Option<#value_type> {
                                                                 &self.#name
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
                                                               pub fn #setter_name(self, new_value: #value_type) -> Self {
                                                                 Self {
                                                                   #name: new_value,
                                                                   ..self
                                                                 }
                                                               }
                                                             }
                                                           }
                                                         });
  quote! {
    #[derive(Debug, Clone)]
    pub struct #entity_name_ident {
      #(#recurse)*
    }
    impl #entity_name_ident {
      #(#getters_and_setters)*
    }
  }
}
