use {crate::utils::DomainDTOAst,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn generate_values(ast: &DomainDTOAst) -> TokenStream {
  let recurse = ast.fields_named
                   .named
                   .iter()
                   .map(|f| {
                     let name = &f.ident;
                     let name = name.as_ref().unwrap();
                     let typ = super::utils::type_in_option_or_itself(f.ty.clone());
                     let value_name = format_ident!("{}{}", ast.root_name_ident, name.to_string().to_case(Case::Pascal));
                     let ts = quote! {
                       #[derive(Debug, Clone)]
                       pub struct #value_name(#typ);
                       impl #value_name {
                         pub fn new<T: Into<#typ>>(x: T) -> Self {
                           Self(x.into())
                         }
                       }
                     };
                     ts
                   })
                   .collect::<Vec<_>>();
  quote! {
    #(#recurse)*
  }
}
