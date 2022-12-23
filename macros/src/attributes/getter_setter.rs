use {proc_macro2::Ident,
     syn::{Data,
           DeriveInput,
           Fields,
           Type}};

#[derive(Debug, Clone)]
pub struct GetterSetterMeta {
  pub ident:     Ident,
  pub raw_types: Vec<RawType>,
}

#[derive(Debug, Clone)]
pub struct RawType {
  pub ident: Ident,
  pub ty:    Type,
}

impl GetterSetterMeta {
  pub fn parse(derive_input: &DeriveInput) -> Self {
    let ident = derive_input.ident.clone();

    let raw_types = if let Data::Struct(ref data) = derive_input.data {
      if let Fields::Named(ref fields) = &data.fields {
        fields.named
              .iter()
              .map(|f| {
                let ident = f.ident.as_ref().unwrap().clone();
                let ty = f.ty.clone();
                RawType { ident, ty }
              })
              .collect()
      } else {
        vec![]
      }
    } else {
      vec![]
    };

    Self { ident, raw_types }
  }
}
