use {crate::{ast::value::{DomainValueAst,
                          DomainValueAttr},
             utils::DomainDefAst},
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

                     let skip = DomainValueAttr::parse_from(f).skip;

                     if skip {
                       quote! {}
                     } else {
                       quote! {
                         #[derive(Debug, Clone, crcnt_ddd_macros::DomainValue)]
                         pub struct #value_name(#typ);
                       }
                     }
                   })
                   .collect::<Vec<_>>();
  quote! {
    #(#recurse)*
  }
}

pub fn generate_value(ast: &DomainValueAst) -> TokenStream {
  let DomainValueAst { ident, inner_type } = ast;
  quote! {
    impl #ident {
      pub fn new<T: Into<#inner_type>>(x: T) -> Self {
        Self(x.into())
      }
      pub fn inner(&self) -> &#inner_type {
        &self.0
      }
      pub fn into_inner(self) -> #inner_type {
        self.0
      }
    }
    impl <T: Into<#inner_type>> From<T> for #ident {
      fn from(x: T) -> Self {
        Self(x.into())
      }
    }
  }
}
