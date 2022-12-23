use {crate::attributes::{entity::EntityFields,
                         EntityMeta},
     convert_case::{Case,
                    Casing},
     proc_macro2::{Ident,
                   TokenStream},
     quote::format_ident,
     serde::Deserialize,
     syn::DeriveInput};

#[derive(Clone)]
pub struct StoreMeta {
  pub table_name:        String,
  pub entity_ident:      Ident,
  pub method_ident:      Ident,
  pub stmt_ident:        Ident,
  pub gen_stmt:          bool,
  pub gen_method:        bool,
  pub fields:            EntityFields,
  pub value_type_prefix: String,
  pub entity_meta:       EntityMeta,
  pub params_extractor:  Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct StoreAttr {
  table_name:       Option<String>,
  rename:           Option<String>,
  features:         Option<String>,
  params_extractor: Option<String>,
}
pub struct StoreAttrParser;

impl StoreMeta {
  pub fn parse(derive_input: &DeriveInput) -> Self {
    // store are also on the meta Struct
    let entity_meta = EntityMeta::parse(derive_input);
    let store_attr = derive_input.attrs
                                 .iter()
                                 .filter_map(|attr| {
                                   if attr.path.segments.iter().find(|x| x.ident == "domain_store").is_some() {
                                     Some(attr.parse_args_with(StoreAttrParser).unwrap())
                                   } else {
                                     None
                                   }
                                 })
                                 .collect::<Vec<_>>();
    if store_attr.len() > 1 {
      panic!("domain_store only allowed once on the Meta Struct");
    }
    let store_attr = store_attr.first()
                               .map(|x| x.clone())
                               .unwrap_or_else(|| StoreAttr { table_name:       None,
                                                              rename:           None,
                                                              features:         None,
                                                              params_extractor: None, });

    let default_name = format!("{}CRUD", entity_meta.entity_ident());
    let default_features = "sql_statements|sql_methods".to_string();

    StoreMeta { table_name: store_attr.table_name
                                      .unwrap_or_else(|| format!("t_{}", entity_meta.root_ident.to_string().to_case(Case::Snake))),
                entity_ident: entity_meta.entity_ident(),
                method_ident: format_ident!("{}Exec", store_attr.rename.as_ref().unwrap_or_else(|| &default_name)),
                stmt_ident: format_ident!("{}Stmt", store_attr.rename.as_ref().unwrap_or_else(|| &default_name)),
                gen_stmt: store_attr.features
                                    .as_ref()
                                    .map(|x| x.clone())
                                    .unwrap_or_else(|| default_features.clone())
                                    .contains("sql_statement"),
                gen_method: store_attr.features
                                      .as_ref()
                                      .map(|x| x.clone())
                                      .unwrap_or_else(|| default_features.clone())
                                      .contains("sql_methods"),
                fields: entity_meta.fields.clone(),
                value_type_prefix: entity_meta.attr.value_type_prefix.clone(),
                entity_meta,
                params_extractor: store_attr.params_extractor }
  }
}

impl syn::parse::Parser for StoreAttrParser {
  type Output = StoreAttr;

  fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> { super::util::parse_attr_token_stream(tokens) }
}
