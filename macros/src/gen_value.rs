use {crate::utils::DomainDefAst,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn generate_values(ast: &DomainDefAst) -> TokenStream {
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
                         pub fn inner(&self) -> &#typ {
                           &self.0
                         }
                         pub fn into_inner(self) -> #typ {
                           self.0
                         }
                       }
                       impl <T: Into<#typ>> From<T> for #value_name {
                         fn from(x: T) -> Self {
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
