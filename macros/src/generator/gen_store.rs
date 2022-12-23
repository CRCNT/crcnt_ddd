use {crate::attributes::StoreMeta,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote},
     syn::DeriveInput};

pub fn generate_store(derive_input: &DeriveInput) -> TokenStream {
  let meta = StoreMeta::parse(derive_input);

  let table_name = &meta.table_name;
  let stmt_indent = &meta.stmt_ident;
  let stmt_select_ident = format_ident!("stmt_select_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_get_ident = format_ident!("stmt_get_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_insert_ident = format_ident!("stmt_insert_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_update_ident = format_ident!("stmt_update_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_update_by_id_ident = format_ident!("stmt_update_by_id_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_delete_ident = format_ident!("stmt_delete_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_delete_by_id_ident = format_ident!("stmt_delete_by_id_{}", meta.entity_ident.to_string().to_case(Case::Snake));

  let update_sets = meta.fields
                        .all_fields()
                        .iter()
                        .map(|f| format!("{} = :{}", f.name, f.name))
                        .collect::<Vec<_>>();
  let select_fields = meta.fields.all_fields().iter().map(|f| f.name.clone()).collect::<Vec<_>>();
  let params_fields = meta.fields.all_fields().iter().map(|f| format!(":{}", f.name)).collect::<Vec<_>>();
  let select_fields = select_fields.join(", ");
  let params_fields = params_fields.join(", ");
  let update_sets = update_sets.join(", ");

  let select_sql = format!("SELECT {} FROM {}", select_fields, table_name);
  let get_sql = format!("SELECT {} FROM {} WHERE id = :id", select_fields, table_name);
  let insert_sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, select_fields, params_fields);
  let update_sql = format!("UPDATE {} SET {}", table_name, update_sets);
  let update_by_id_sql = format!("UPDATE {} SET {} WHERE id = :id", table_name, update_sets);
  let delete_sql = format!("DELETE FROM {}", table_name);
  let delete_by_id_sql = format!("DELETE FROM {} WHERE id = :id", table_name);

  quote! {
    pub trait #stmt_indent {
      fn #stmt_select_ident() -> &'static str {
        #select_sql
      }
      fn #stmt_get_ident() -> &'static str {
        #get_sql
      }
      fn #stmt_insert_ident() -> &'static str {
        #insert_sql
      }
      fn #stmt_update_ident() -> &'static str {
        #update_sql
      }
      fn #stmt_update_by_id_ident() -> &'static str {
        #update_by_id_sql
      }
      fn #stmt_delete_ident() -> &'static str {
        #delete_sql
      }
      fn #stmt_delete_by_id_ident() -> &'static str {
        #delete_by_id_sql
      }
    }
  }
}
