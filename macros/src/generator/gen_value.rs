use {crate::attributes::{ValueImpl,
                         ValueMeta},
     proc_macro2::TokenStream,
     quote::quote,
     syn::DeriveInput};

pub fn generate_value_token_stream(derive_input: &DeriveInput) -> TokenStream {
  let value_attr = ValueMeta::parse(derive_input);
  // dbg!(&value_attr);

  let ValueMeta { ident,
                  impls,
                  is_enum,
                  inner_ident,
                  enum_items, } = value_attr;
  let lines = enum_items.iter()
                        .map(|item| {
                          let value = item.to_string();
                          quote! {
                            #ident => #value,
                          }
                        })
                        .collect::<Vec<_>>();
  let basic_impls = impls.iter()
                         .map(|imp| match imp {
                           | ValueImpl::Inner => {
                             if is_enum {
                               quote! {
                                 pub fn inner(&self) -> &'static str {
                                   use #ident::*;
                                   match self {
                                     #(#lines)*
                                   }
                                 }
                               }
                             } else {
                               quote! {
                                 pub fn inner(&self) -> &#inner_ident {
                                   &self.0
                                 }
                               }
                             }
                           }
                           | ValueImpl::IntoInner => {
                             if is_enum {
                               quote! {
                                 pub fn into_inner(self) -> &'static str {
                                   use #ident::*;
                                   match self {
                                     #(#lines)*
                                   }
                                 }
                               }
                             } else {
                               quote! {
                                 pub fn into_inner(self) -> #inner_ident {
                                   self.0
                                 }
                               }
                             }
                           }
                           | ValueImpl::From => {
                             quote! {}
                           }
                         })
                         .collect::<Vec<_>>();

  let new_impls = if is_enum {
    quote! {}
  } else {
    quote! {
      pub fn new<T: Into<#inner_ident>>(value: T) -> Self {
        Self(value.into())
      }
    }
  };

  let from_impls = impls.iter()
                        .filter_map(|imp| {
                          if imp == &ValueImpl::From {
                            let ident_str = ident.to_string();
                            let err = format!("{{}} can't be transformed to {}", ident_str);
                            let match_ok_lines = enum_items.iter()
                                                           .map(|item| {
                                                             let item_str = item.to_string();
                                                             quote! {
                                                               #item_str => Ok(#item),
                                                             }
                                                           })
                                                           .collect::<Vec<_>>();
                            let from_impl = if is_enum {
                              quote! {
                                impl <'a> TryFrom<&'a str> for #ident {
                                  type Error = String;
                                  fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
                                    use #ident::*;
                                    match value {
                                      #(#match_ok_lines)*
                                      x => Err(format!(#err, x)),
                                    }
                                  }
                                }
                              }
                            } else {
                              quote! {
                                impl<T: Into<#inner_ident>> From<T> for #ident {
                                  fn from(value: T) -> Self { Self(value.into()) }
                                }
                              }
                            };

                            Some(from_impl)
                          } else {
                            None
                          }
                        })
                        .collect::<Vec<_>>();
  quote! {
    impl #ident {
      #new_impls
      #(#basic_impls)*
    }

    #(#from_impls)*
  }
}
