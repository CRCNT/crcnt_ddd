extern crate core;

use {crate::attributes::Command,
     quote::quote,
     syn::{parse_macro_input,
           DeriveInput}};

mod attributes;
mod generator;

#[proc_macro_derive(Domain, attributes(domain_commands, domain_value_impl))]
pub fn domain_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let derive_input = parse_macro_input!(input as DeriveInput);
  let commands = Command::commands(&derive_input);
  let token_streams = commands.iter()
                              .map(|command| generator::generate(command, &derive_input))
                              .collect::<Vec<_>>();
  (quote! {
    #(#token_streams)*
  }).into()
}

// easy for domain_value
//
// #[allow(unused_macros)]
// #[macro_export]
// macro_rules! domain_value {
//   ($type_name: ident, $inner_type_name: ident) => {
//     #[derive(Debug, Clone, crcnt_ddd_macros::Domain)]
//     #[domain_commands(value)]
//     pub struct $type_name(pub $inner_type_name);
//     impl $type_name {
//       pub fn value(&self) -> &$inner_type_name { &self.0 }
//     }
//   };
//   ($type_name: ident, $inner_type_name:ident <$t: ident>) => {
//     #[derive(Debug, Clone, crcnt_ddd_macros::Domain)]
//     #[domain_commands(value)]
//     pub struct $type_name(pub $inner_type_name<$t>);
//     impl $type_name {
//       pub fn value(&self) -> &$inner_type_name<$t> { &self.0 }
//     }
//   };
// }

// mod ast;
// mod gen_dto;
// mod gen_entity;
// mod gen_store;
// mod gen_value;
// mod utils;
//
// #[proc_macro_derive(DomainModel, attributes(domain_values))]
// pub fn domain_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//
//   let domain_def_ast = DomainDefAst::new(derive_input);
//
//   let values = gen_value::generate_values(&domain_def_ast);
//   let entity = gen_entity::gen_entity(&domain_def_ast);
//   let dto = gen_dto::gen_dto(&domain_def_ast);
//   let store = gen_store::gen_store(&domain_def_ast);
//
//   let expanded = quote! {
//     #values
//     #entity
//     #dto
//     #store
//   };
//
//   proc_macro::TokenStream::from(expanded)
// }
//
// #[proc_macro_derive(DomainStore, attributes(domain_values, domain_store))]
// pub fn domain_store_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//   let domain_def_ast = DomainDefAst::new(derive_input);
//   let store = gen_store::gen_store(&domain_def_ast);
//   store.into()
// }
//
// #[proc_macro_derive(DomainDTO, attributes(domain_values))]
// pub fn domain_dto_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//   let domain_def_ast = DomainDefAst::new(derive_input);
//
//   let dto = gen_dto::gen_dto(&domain_def_ast);
//   dto.into()
// }
//
// #[proc_macro_derive(DomainEntity, attributes(domain_values))]
// pub fn domain_entity_def(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//   let domain_def_ast = DomainDefAst::new(derive_input);
//   let entity = gen_entity::gen_entity(&domain_def_ast);
//   entity.into()
// }
//
// #[proc_macro_derive(DomainValues, attributes(domain_values))]
// pub fn domain_values(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//   let domain_def_ast = DomainDefAst::new(derive_input);
//   gen_value::generate_values(&domain_def_ast).into()
// }
//
// #[proc_macro_derive(DomainValue)]
// pub fn domain_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//   let derive_input = parse_macro_input!(input as DeriveInput);
//   let domain_value_ast = DomainValueAst::new(derive_input);
//   let value = gen_value::generate_value(&domain_value_ast);
//
//   proc_macro::TokenStream::from(value)
// }
