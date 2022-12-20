use {proc_macro2::{Ident,
                   TokenStream},
     syn::{parse::{Parse,
                   ParseStream},
           Data,
           DeriveInput,
           Field,
           Fields,
           Type}};

pub struct DomainValueAst {
  pub ident:      Ident,
  pub inner_type: Type,
}
impl DomainValueAst {
  pub fn new(derive_input: DeriveInput) -> Self {
    let ident = derive_input.ident;
    if let Data::Struct(data) = derive_input.data {
      if let Fields::Unnamed(unnamed_fields) = data.fields {
        if unnamed_fields.unnamed.len() != 1 {
          panic!("DomainValue only works for tuple1")
        }
        let field = unnamed_fields.unnamed.first().unwrap();
        DomainValueAst { ident,
                         inner_type: field.ty.clone() }
      } else {
        panic!("DomainValue only works for tuple struct")
      }
    } else {
      panic!("DomainValue only works on Struct")
    }
  }
}

#[derive(Debug, Clone)]
pub struct DomainValueAttr {
  pub skip: bool,
}

pub struct DomainValueAttrParser;
impl syn::parse::Parser for DomainValueAttrParser {
  type Output = DomainValueAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
    let tokens_str = tokens.to_string();
    dbg!(&tokens_str);
    Ok(DomainValueAttr { skip: tokens_str.eq("skip") })
  }
}

impl DomainValueAttr {
  pub fn parse_from(f: &Field) -> Self {
    let attrs = &f.attrs;
    let domain_values_attr = attrs.iter()
                                  .filter_map(|attr| {
                                    let domain_values_attr =
                                      attr.parse_args_with(DomainValueAttrParser).expect("parse domain value attributes failed");
                                    return Some(domain_values_attr);
                                  })
                                  .collect::<Vec<_>>();
    domain_values_attr.first().cloned().unwrap_or_else(|| DomainValueAttr { skip: false })
  }
}

impl Parse for DomainValueAttr {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let skip = input.to_string();

    if skip.eq("skip") {
      dbg!(&skip);
      Ok(DomainValueAttr { skip: true })
    } else {
      println!("only `skip` allowed, see: domain_values(skip)");
      Ok(DomainValueAttr { skip: false })
    }
  }
}
