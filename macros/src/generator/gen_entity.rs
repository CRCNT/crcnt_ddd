use {crate::attributes::EntityMeta,
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote},
     syn::DeriveInput};

pub fn generate_entity_token_stream(derive_input: &DeriveInput) -> TokenStream {
  //
  let entity_meta = EntityMeta::parse(derive_input);
  let entity_ident = entity_meta.entity_ident();

  let field_token_streams = entity_meta.fields
                                       .all_fields()
                                       .iter()
                                       .map(|field| {
                                         let name_ident = format_ident!("{}", field.name);
                                         let ty = field.field_type(&entity_meta);
                                         // dbg!(&ty.to_token_stream().to_string());
                                         quote! {
                                           #name_ident: #ty,
                                         }
                                       })
                                       .collect::<Vec<_>>();

  let new_value_types = entity_meta.fields
                                   .new_value_type_fields()
                                   .iter()
                                   .map(|field| {
                                     let new_type = field.field_type_without_option(&entity_meta);
                                     if field.is_enum {
                                       let enum_items = (&field.enum_items).iter()
                                                                           .map(|x| {
                                                                             let ident = format_ident!("{}", x);
                                                                             quote! {
                                                                               #ident,
                                                                             }
                                                                           })
                                                                           .collect::<Vec<_>>();
                                       quote! {
                                         #[derive(crcnt_ddd_macros::Domain, Debug, Clone, PartialOrd, PartialEq)]
                                         #[domain_commands(value)]
                                         pub enum #new_type {
                                           #(#enum_items)*
                                         }
                                       }
                                     } else {
                                       let raw_type = &field.primary_type;
                                       quote! {
                                         #[derive(crcnt_ddd_macros::Domain, Debug, Clone)]
                                         #[domain_commands(value)]
                                         pub struct #new_type(#raw_type);
                                       }
                                     }
                                   })
                                   .collect::<Vec<_>>();

  // builders

  quote! {
    #(#new_value_types)*
    #[derive(crcnt_ddd_macros::Domain, Debug, Clone)]
    #[domain_commands(builder)]
    pub struct #entity_ident {
      #(#field_token_streams)*
    }
  }
}
