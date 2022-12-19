use {crate::utils::DomainDefAst,
     quote::quote,
     syn::{parse_macro_input,
           DeriveInput}};

mod gen_dto;
mod gen_entity;
mod gen_value;
mod utils;

#[proc_macro_derive(DomainModel)]
pub fn domain_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);

  let domain_def_ast = DomainDefAst::new(derive_input);

  let values = gen_value::generate_values(&domain_def_ast);
  let entity = gen_entity::gen_entity(&domain_def_ast);
  let dto = gen_dto::gen_dto(&domain_def_ast);

  let expanded = quote! {
    #values
    #entity
    #dto
  };

  proc_macro::TokenStream::from(expanded)
}
