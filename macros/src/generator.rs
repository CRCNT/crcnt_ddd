use {crate::attributes::Command,
     proc_macro2::TokenStream,
     syn::DeriveInput};

mod gen_value;

pub fn generate(command: &Command, derive_input: &DeriveInput) -> TokenStream {
  match command {
    | Command::GenValue => gen_value::generate_value_token_stream(derive_input),
    | _ => todo!(),
  }
}
