use {crate::{ast::value::DomainValueAst,
             utils::DomainDefAst},
     quote::quote,
     syn::{parse_macro_input,
           DeriveInput}};

mod ast;
mod gen_dto;
mod gen_entity;
mod gen_store;
mod gen_value;
mod utils;

#[proc_macro_derive(DomainModel, attributes(domain_values))]
pub fn domain_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);

  let domain_def_ast = DomainDefAst::new(derive_input);

  let values = gen_value::generate_values(&domain_def_ast);
  let entity = gen_entity::gen_entity(&domain_def_ast);
  let dto = gen_dto::gen_dto(&domain_def_ast);
  let store = gen_store::gen_store(&domain_def_ast);

  let expanded = quote! {
    #values
    #entity
    #dto
    #store
  };

  proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(DomainStore, attributes(domain_values, domain_store))]
pub fn domain_store_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let domain_def_ast = DomainDefAst::new(derive_input);
  let store = gen_store::gen_store(&domain_def_ast);
  store.into()
}

#[proc_macro_derive(DomainDTO, attributes(domain_values))]
pub fn domain_dto_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let domain_def_ast = DomainDefAst::new(derive_input);

  let dto = gen_dto::gen_dto(&domain_def_ast);
  dto.into()
}

#[proc_macro_derive(DomainEntity, attributes(domain_values))]
pub fn domain_entity_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let domain_def_ast = DomainDefAst::new(derive_input);
  let entity = gen_entity::gen_entity(&domain_def_ast);
  entity.into()
}

#[proc_macro_derive(DomainValues, attributes(domain_values))]
pub fn domain_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let domain_def_ast = DomainDefAst::new(derive_input);
  gen_value::generate_values(&domain_def_ast).into()
}

#[proc_macro_derive(DomainValue)]
pub fn domain_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let domain_value_ast = DomainValueAst::new(derive_input);
  let value = gen_value::generate_value(&domain_value_ast);

  proc_macro::TokenStream::from(value)
}
