use {proc_macro2::{Ident,
                   TokenStream},
     quote::{format_ident,
             ToTokens},
     syn::{Data,
           DeriveInput,
           Fields}};

#[derive(Debug, Clone)]
pub struct ValueMeta {
  pub ident:       Ident,
  pub impls:       Vec<ValueImpl>,
  pub is_enum:     bool,
  pub inner_ident: Ident,
  pub enum_items:  Vec<Ident>,
}

impl ValueMeta {
  pub fn parse(derive_input: &DeriveInput) -> Self {
    let attrs = &derive_input.attrs;
    let impls = attrs.iter()
                     .filter_map(|attr| {
                       if let Some(_) = attr.path.segments.iter().find(|s| s.ident == "domain_value_impl") {
                         let impls = attr.parse_args_with(ValueImplParser).unwrap();
                         Some(impls)
                       } else {
                         None
                       }
                     })
                     .collect::<Vec<_>>();
    let impls = impls.into_iter().flatten().collect::<Vec<_>>();
    let impls = if impls.is_empty() {
      vec![ValueImpl::IntoInner, ValueImpl::Inner, ValueImpl::From, ValueImpl::IntoMysqlValue]
    } else {
      impls
    };
    let ident = derive_input.ident.clone();
    let (is_enum, inner_ident, enum_items) = match derive_input.data {
      | Data::Struct(ref data_struct) => {
        let inner_indent = match data_struct.fields {
          | Fields::Unnamed(ref fields_unnamed) => {
            if fields_unnamed.unnamed.len() != 1 {
              panic!("Value can be only on Single Unnamed Field Struct")
            } else {
              let field = fields_unnamed.unnamed.first().unwrap();
              let inner_type = field.ty.to_token_stream().to_string();
              format_ident!("{}", inner_type)
            }
          }
          | _ => panic!("Value can be only on Unnamed Struct"),
        };
        (false, inner_indent, vec![])
      }
      | Data::Enum(ref data_enum) => {
        let variants = &data_enum.variants;
        let enum_items = variants.iter()
                                 .map(|v| {
                                   // dbg!(&v.ident);
                                   v.ident.clone()
                                 })
                                 .collect::<Vec<_>>();
        (true, format_ident!("String"), enum_items)
      }
      | Data::Union(_) => panic!("Value can be only on Struct and Enum"),
    };
    ValueMeta { impls,
                ident,
                is_enum,
                inner_ident,
                enum_items }
  }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ValueImpl {
  Inner,
  IntoInner,
  From,
  IntoMysqlValue,
}

pub struct ValueImplParser;

impl syn::parse::Parser for ValueImplParser {
  type Output = Vec<ValueImpl>;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
    let xs = tokens.to_string()
                   .split(",")
                   .map(|x| match x.trim() {
                     | "inner" => ValueImpl::Inner,
                     | "into_inner" => ValueImpl::IntoInner,
                     | "from" => ValueImpl::From,
                     | "into_mysql_value" => ValueImpl::Inner,
                     | x => panic!("domain_value_impl can only be inner, into_inner, but found: {}", x),
                   })
                   .collect::<Vec<_>>();
    Ok(xs)
  }
}
