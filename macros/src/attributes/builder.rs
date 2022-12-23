use {convert_case::{Case,
                    Casing},
     proc_macro2::Ident,
     quote::{format_ident,
             quote,
             ToTokens},
     syn::{Data,
           DeriveInput,
           Fields,
           Type}};

pub struct BuilderMeta {
  pub ident:  Ident,
  pub fields: StructFields,
}

impl BuilderMeta {
  pub fn parse(derive_input: &DeriveInput) -> Self {
    let ident = derive_input.ident.clone();
    let fields = StructFields::parse(&derive_input);

    Self { ident, fields }
  }

  pub fn builder_ident(&self) -> Ident { format_ident!("{}Builder", self.ident) }

  pub fn builder_generic(&self) -> Ident { format_ident!("All{}Fields", self.ident) }
}

pub struct StructFields(Vec<StructField>);
impl StructFields {
  pub fn fields(&self) -> &Vec<StructField> { &self.0 }

  pub fn init_builder_generic(&self) -> Box<dyn ToTokens> {
    let units = self.fields()
                    .iter()
                    .map(|_x| {
                      quote! {
                        (),
                      }
                    })
                    .collect::<Vec<_>>();
    Box::new(quote! {
      (#(#units)*)
    })
  }

  pub fn all_builder_generic(&self) -> Box<dyn ToTokens> {
    let generics = self.fields()
                       .iter()
                       .map(|x| {
                         let ty = &x.generic_ty;
                         quote! {
                           #ty,
                         }
                       })
                       .collect::<Vec<_>>();
    Box::new(quote! {
      #(#generics)*
    })
  }

  pub fn all_builder_exact_types(&self) -> Box<dyn ToTokens> {
    let types = self.fields()
                    .iter()
                    .map(|x| {
                      let ty = &x.ty;
                      quote! {
                        #ty,
                      }
                    })
                    .collect::<Vec<_>>();
    Box::new(quote! {
      #(#types)*
    })
  }

  pub fn exact_generic_stepped(&self, field: &StructField) -> Box<dyn ToTokens> {
    let generic = self.fields().iter().fold(vec![], |mut acc, n| {
                                        if field.ident == n.ident {
                                          let ty = &n.ty;
                                          acc.push(quote! {#ty,});
                                          acc
                                        } else {
                                          let generic_ty = &n.generic_ty;
                                          acc.push(quote! {#generic_ty,});
                                          acc
                                        }
                                      });
    Box::new(quote! {
      #(#generic)*
    })
  }

  pub fn extract_fields(&self, field: &StructField) -> (Box<dyn ToTokens>, Box<dyn ToTokens>) {
    let (let_statement, fields) = self.fields().iter().fold((vec![], vec![]), |(mut l, mut f), n| {
                                                        let ident = &n.ident;
                                                        if field.ident == n.ident {
                                                          l.push(quote! {_, });
                                                          f.push(quote! {#ident, });
                                                          (l, f)
                                                        } else {
                                                          l.push(quote! {#ident, });
                                                          f.push(quote! {#ident, });
                                                          (l, f)
                                                        }
                                                      });

    (Box::new(quote! { #(#let_statement)* }), Box::new(quote! { #(#fields)* }))
  }

  pub fn extract_all_fields(&self) -> Box<dyn ToTokens> {
    let field_names = self.fields().iter().fold(vec![], |mut acc, n| {
                                            let ident = &n.ident;
                                            acc.push(quote! {#ident,});
                                            acc
                                          });
    Box::new(quote! {
      #(#field_names)*
    })
  }
}
pub struct StructField {
  pub ident:         Ident,
  pub ty:            Type,
  pub missed_tip_ty: Ident,
  pub generic_ty:    Ident,
}

impl StructFields {
  pub fn parse(derive_input: &DeriveInput) -> Self {
    if let Data::Struct(ref data) = derive_input.data {
      if let Fields::Named(ref fields) = data.fields {
        let fields = fields.named
                           .iter()
                           .map(|f| {
                             let ident = f.ident.clone().unwrap();
                             let ty = f.ty.clone();
                             let missed_tip_ty = format_ident!("{}_Miss_{}", derive_input.ident, ident);
                             let generic_ty = format_ident!("{}Type", ident.to_string().to_case(Case::Pascal));
                             StructField { ident,
                                           ty,
                                           missed_tip_ty,
                                           generic_ty }
                           })
                           .collect::<Vec<_>>();
        StructFields(fields)
      } else {
        panic!("domain_commands(builder) can only be on a named struct");
      }
    } else {
      panic!("domain_commands(builder) can only be on a named struct");
    }
  }
}
