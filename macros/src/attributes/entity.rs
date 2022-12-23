use {convert_case::{Case,
                    Casing},
     proc_macro2::{Ident,
                   TokenStream},
     quote::{format_ident,
             quote,
             ToTokens},
     serde::Deserialize,
     syn::{spanned::Spanned,
           Attribute,
           Data,
           DeriveInput,
           Fields,
           Type}};

#[derive(Debug, Clone)]
pub struct EntityMeta {
  pub root_ident: Ident,
  pub attr:       ExactEntityAttr,
  pub fields:     EntityFields,
}

#[derive(Debug, Clone)]
pub struct ExactEntityAttr {
  pub rename:            String,
  pub value_type_prefix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EntityAttr {
  pub rename:            Option<String>,
  pub value_type_prefix: Option<String>,
}
pub struct EntityAttrParser;

#[derive(Debug, Clone)]
pub struct EntityField {
  pub name:          String,
  pub primary_type:  Type,
  pub optional:      bool,
  pub skip_new_type: bool,
  pub is_enum:       bool,
  pub enum_items:    Vec<String>,
}
impl EntityField {
  pub fn field_type_without_option(&self, entity_meta: &EntityMeta) -> Box<dyn ToTokens> {
    let ty: Box<dyn ToTokens> = if self.skip_new_type {
      Box::new(self.primary_type.clone())
    } else {
      Box::new(format_ident!("{}{}", entity_meta.attr.value_type_prefix, self.name.to_case(Case::Pascal)))
    };
    ty
  }

  pub fn field_type(&self, entity_meta: &EntityMeta) -> Box<dyn ToTokens> {
    let ty: Box<dyn ToTokens> = self.field_type_without_option(entity_meta);

    if self.optional {
      // let ty = ty.to_token_stream().to_string();
      Box::new(quote! {
        Option<#ty>
      })
    } else {
      ty
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EntityFieldAttr {
  pub optional:      Option<bool>,
  pub skip_new_type: Option<bool>,
  pub enums:         Option<String>,
}
pub struct EntityFieldAttrParser;
impl EntityFieldAttr {
  pub fn parse(_entity_attr: &ExactEntityAttr, attrs: &Vec<Attribute>) -> Self {
    let field_attrs = attrs.iter()
                           .filter_map(|attr| {
                             if let Some(_s) = attr.path.segments.iter().find(|s| s.ident == "domain_value") {
                               Some(attr.parse_args_with(EntityFieldAttrParser).unwrap())
                             } else {
                               None
                             }
                           })
                           .collect::<Vec<_>>();
    if field_attrs.len() > 1 {
      panic!("only ONE domain_value allowed");
    }
    let xs = field_attrs.first();
    xs.map(|x| x.clone()).unwrap_or_else(|| EntityFieldAttr { optional:      Some(false),
                                                              skip_new_type: Some(false),
                                                              enums:         None, })
  }
}

#[derive(Debug, Clone)]
pub struct EntityFields(Vec<EntityField>);
impl EntityFields {
  pub fn all_fields(&self) -> &Vec<EntityField> { &self.0 }

  pub fn new_value_type_fields(&self) -> Vec<EntityField> {
    self.0
        .iter()
        .filter_map(|field| if field.skip_new_type { None } else { Some(field.clone()) })
        .collect::<Vec<_>>()
  }
}

impl EntityMeta {
  pub fn entity_ident(&self) -> Ident { format_ident!("{}", &self.attr.rename) }

  pub fn parse(derive_input: &DeriveInput) -> Self {
    let ident = derive_input.ident.to_string();
    if !(ident.starts_with("__") && ident.ends_with("__")) {
      panic!("Domain meta data should be wrapped within __, e.g., __Person__");
    }
    let root_ident = &ident[2 .. (ident.len() - 2)];
    let root_ident = format_ident!("{}", root_ident);

    let attr = EntityAttr::parse(&root_ident, &derive_input.attrs);
    let attr = ExactEntityAttr { rename:            attr.rename.unwrap_or_else(|| format!("{}Entity", root_ident)),
                                 value_type_prefix: attr.value_type_prefix.unwrap_or_else(|| root_ident.to_string()), };

    let fields = EntityFields::parse(&attr, &derive_input.data);

    EntityMeta { root_ident, attr, fields }
  }
}

impl syn::parse::Parser for EntityAttrParser {
  type Output = EntityAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> { super::util::parse_attr_token_stream(tokens) }
}

impl EntityAttr {
  pub fn parse(root_ident: &Ident, attrs: &Vec<Attribute>) -> Self {
    let attrs = attrs.iter()
                     .filter_map(|attr| {
                       if let Some(_segment) = attr.path.segments.iter().find(|s| s.ident == "domain_entity") {
                         Some(attr.parse_args_with(EntityAttrParser).unwrap())
                       } else {
                         None
                       }
                     })
                     .collect::<Vec<_>>();
    if attrs.len() > 1 {
      panic!("domain_entity only be allowed to declared once");
    }
    attrs.first()
         .map(|x| x.clone())
         .unwrap_or_else(|| EntityAttr { rename:            Some(format!("{}Entity", root_ident)),
                                         value_type_prefix: Some(format!("{}", root_ident)), })
  }
}

impl syn::parse::Parser for EntityFieldAttrParser {
  type Output = EntityFieldAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> { super::util::parse_attr_token_stream(tokens) }
}

impl EntityFields {
  pub fn parse(entity_attr: &ExactEntityAttr, data: &Data) -> Self {
    if let Data::Struct(ref struct_data) = data {
      if let Fields::Named(ref fields) = struct_data.fields {
        let xs = fields.named
                       .iter()
                       .map(|f| {
                         let name = f.ident.as_ref().unwrap().to_string();
                         let primary_type = f.ty.clone();
                         let field_attr = EntityFieldAttr::parse(entity_attr, &f.attrs);
                         let enum_items = field_attr.enums
                                                    .as_ref()
                                                    .map(|x| {
                                                      let xs = x.split("|").map(|x| x.to_string()).collect::<Vec<_>>();
                                                      xs
                                                    })
                                                    .unwrap_or_else(|| vec![]);
                         EntityField { name,
                                       primary_type,
                                       optional: field_attr.optional.unwrap_or_else(|| false),
                                       skip_new_type: field_attr.skip_new_type.unwrap_or_else(|| false),
                                       is_enum: field_attr.enums.is_some(),
                                       enum_items }
                       })
                       .collect::<Vec<_>>();
        EntityFields(xs)
      } else {
        panic!("domain_commands(entity) can only be on Named Struct");
      }
    } else {
      panic!("domain_commands(entity) can only be on Struct");
    }
  }
}
