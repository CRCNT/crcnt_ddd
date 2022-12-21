use {proc_macro2::{Ident,
                   TokenStream},
     serde::Deserialize,
     syn::{Data,
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

#[derive(Debug, Clone, Deserialize)]
pub struct DomainValueAttr {
  pub skip: bool,
}

pub struct DomainValueAttrParser;
impl syn::parse::Parser for DomainValueAttrParser {
  type Output = DomainValueAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
    let tokens_str = tokens.to_string();
    dbg!(&tokens_str);
    let attr: DomainValueAttr = toml::from_str(tokens_str.split(",").collect::<Vec<_>>().join("\n").as_str()).unwrap();
    Ok(attr)
  }
}

impl DomainValueAttr {
  pub fn parse_from(f: &Field) -> Self {
    let attrs = &f.attrs;
    let domain_values_attr = attrs.iter()
                                  .filter_map(|attr| {
                                    if attr.path.segments.iter().find(|s| s.ident == "domain_values").is_some() {
                                      let domain_values_attr =
                                        attr.parse_args_with(DomainValueAttrParser).expect("parse domain value attributes failed");
                                      Some(domain_values_attr)
                                    } else {
                                      None
                                    }
                                  })
                                  .collect::<Vec<_>>();
    domain_values_attr.first().cloned().unwrap_or_else(|| DomainValueAttr { skip: false })
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DomainAttr {
  pub table_name: String,
}

impl DomainAttr {
  pub fn parse_from(derive_input: &DeriveInput) -> Option<Self> {
    let attrs = &derive_input.attrs;
    let domain_attrs = attrs.iter()
                            .filter_map(|attr| {
                              let domain_store_attr = attr.path.segments.iter().find(|s| &s.ident == "domain_store");
                              if domain_store_attr.is_some() {
                                let domain_attr = attr.parse_args_with(DomainAttrParser).expect("parse domain attr failed");
                                Some(domain_attr)
                              } else {
                                None
                              }
                            })
                            .collect::<Vec<_>>();
    let domain_attr = domain_attrs.first();
    domain_attr.map(|x| x.clone())
  }
}

pub struct DomainAttrParser;
impl syn::parse::Parser for DomainAttrParser {
  type Output = DomainAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
    let domain_attr_token = tokens.to_string();
    let line = domain_attr_token.split(",").collect::<Vec<_>>();
    let line = line.join("\n");
    let domain_attr: DomainAttr = toml::from_str(line.as_str()).unwrap();
    Ok(domain_attr)
  }
}
