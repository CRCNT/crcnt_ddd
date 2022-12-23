use {proc_macro2::TokenStream,
     syn::DeriveInput};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Command {
  GenValue,
  GenEntity,
  GenValues,
  GenStore,
  GenBuilder,
  GenGetterSetter,
}

impl Command {
  pub fn commands(input: &DeriveInput) -> Vec<Command> {
    let xs = input.attrs
                  .iter()
                  .filter_map(|attr| {
                    if let Some(_segment) = attr.path.segments.iter().find(|s| s.ident == "domain_commands") {
                      match attr.parse_args_with(CommandsParser) {
                        | Ok(xs) => Some(xs),
                        | Err(e) => panic!("can't parse domain_commands: {}", e.to_string()),
                      }
                    } else {
                      None
                    }
                  })
                  .collect::<Vec<_>>();
    xs.into_iter().flatten().collect()
  }
}

pub struct CommandsParser;
impl syn::parse::Parser for CommandsParser {
  type Output = Vec<Command>;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
    use Command::*;

    let domain_commands = tokens.to_string();
    // dbg!(&domain_commands);

    let commands = domain_commands.split(",")
                                  .map(|x| match x.trim() {
                                    | "entity" => GenEntity,
                                    | "values" => GenValues,
                                    | "value" => GenValue,
                                    | "store" => GenStore,
                                    | "builder" => GenBuilder,
                                    | "getter_setter" => GenGetterSetter,
                                    | x => panic!("domain_commands can only be: entity, values, value, store, but the input is {}", x),
                                  })
                                  .collect::<Vec<_>>();

    Ok(commands)
  }
}

mod builder;
mod entity;
mod getter_setter;
mod value;

pub use {builder::BuilderMeta,
         entity::EntityMeta,
         getter_setter::GetterSetterMeta,
         value::{ValueImpl,
                 ValueMeta}};
