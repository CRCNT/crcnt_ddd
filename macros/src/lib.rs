use {crate::utils::DomainDTOAst,
     quote::quote,
     syn::{parse_macro_input,
           DeriveInput}};

mod gen_entity;
mod gen_value;
mod utils;

#[proc_macro_derive(DomainEntityAndValues)]
pub fn domain_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);

  let domain_dto_ast = DomainDTOAst::new(derive_input);

  let values = gen_value::generate_values(&domain_dto_ast);
  let entity = gen_entity::gen_entity(&domain_dto_ast);
  let expanded = quote! {
  #values
  #entity
  };

  proc_macro::TokenStream::from(expanded)
}
