use {crate::attributes::Command,
     proc_macro2::TokenStream,
     syn::DeriveInput};

mod gen_builder;
mod gen_entity;
mod gen_value;

pub fn generate(command: &Command, derive_input: &DeriveInput) -> TokenStream {
  match command {
    | Command::GenValue => gen_value::generate_value_token_stream(derive_input),
    | Command::GenEntity => gen_entity::generate_entity_token_stream(derive_input),
    | Command::GenBuilder => gen_builder::generate_builder(derive_input),
    | _ => todo!(),
  }
}
