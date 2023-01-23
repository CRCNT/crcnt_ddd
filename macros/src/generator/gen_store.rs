use {crate::attributes::StoreMeta,
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote},
     std::str::FromStr,
     syn::DeriveInput};

pub fn generate_store(derive_input: &DeriveInput) -> TokenStream {
  let meta = StoreMeta::parse(derive_input);

  let table_name = &meta.table_name;
  let stmt_indent = &meta.stmt_ident;
  let method_indent = &meta.method_ident;

  let stmt_select_ident = format_ident!("stmt_select_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_count_ident = format_ident!("stmt_count_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_get_ident = format_ident!("stmt_get_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_insert_ident = format_ident!("stmt_insert_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_update_ident = format_ident!("stmt_update_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_update_by_id_ident = format_ident!("stmt_update_by_id_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_delete_ident = format_ident!("stmt_delete_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let stmt_delete_by_id_ident = format_ident!("stmt_delete_by_id_{}", meta.entity_ident.to_string().to_case(Case::Snake));

  let entity_params_fn_name = format_ident!("mysql_params_{}", meta.entity_ident.to_string().to_case(Case::Snake));

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
  let count_sql = format!("SELECT count(1) FROM {}", table_name);
  let get_sql = format!("SELECT {} FROM {} WHERE id = :id", select_fields, table_name);
  let insert_sql = format!("INSERT INTO {} ({}) VALUES ({})", table_name, select_fields, params_fields);
  let update_sql = format!("UPDATE {} SET {}", table_name, update_sets);
  let update_by_id_sql = format!("UPDATE {} SET {} WHERE id = :id", table_name, update_sets);
  let delete_sql = format!("DELETE FROM {}", table_name);
  let delete_by_id_sql = format!("DELETE FROM {} WHERE id = :id", table_name);

  let entity_name_ident = &meta.entity_ident;

  let sql_param_items =
    meta.fields
        .all_fields()
        .iter()
        .map(|f| {
          let name = f.name.clone();
          let getter_name = format_ident!("ref_{}", name);

          if f.optional {
            if f.skip_new_type {
              quote! {
                param_map.insert(Vec::<u8>::from(#name), mysql_async::Value::from(entity.#getter_name()));
              }
            } else {
              quote! {
                param_map.insert(Vec::<u8>::from(#name), mysql_async::Value::from(entity.#getter_name().as_ref().map(|x|x.inner())));
              }
            }
          } else {
            if f.skip_new_type {
              quote! {
                param_map.insert(Vec::<u8>::from(#name), mysql_async::Value::from(entity.#getter_name()));
              }
            } else {
              quote! {
                param_map.insert(Vec::<u8>::from(#name), mysql_async::Value::from(entity.#getter_name().inner()));
              }
            }
          }
        })
        .collect::<Vec<_>>();

  let insert_fn_name = format_ident!("exec_insert_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let select_fn_name = format_ident!("exec_select_where_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let count_fn_name = format_ident!("exec_count_where_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let get_fn_name = format_ident!("exec_get_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let update_fn_name = format_ident!("exec_update_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let delete_where_fn_name = format_ident!("exec_delete_where_{}", meta.entity_ident.to_string().to_case(Case::Snake));
  let delete_by_id_fn_name = format_ident!("exec_delete_by_id_{}", meta.entity_ident.to_string().to_case(Case::Snake));

  let field_id = meta.entity_meta
                     .fields
                     .all_fields()
                     .iter()
                     .find(|&x| "id".eq(&x.name))
                     .expect("no `id` field");
  let entity_id_ident = field_id.field_type(&meta.entity_meta); //format_ident!("{}Id", meta.value_type_prefix);

  let get_row_items =
    meta.fields
        .all_fields()
        .iter()
        .map(|f| {
          let name = f.name.clone();
          let name_ident = format_ident!("{}", name);
          let error_msg = format!("can't get {} from row", name);
          let ty_inner = f.primary_type.clone();
          let value_type = f.field_type_without_option(&meta.entity_meta);
          if f.optional {
            if f.skip_new_type {
              quote! {
                let #name_ident = row.get_opt::<#ty_inner, &'static str>(#name).map(|x| x.ok()).flatten();
              }
            } else {
              if f.is_enum {
                quote! {
                  let #name_ident = row.get_opt::<#ty_inner, &'static str>(#name).map(|x| x.ok()).flatten().map(|x| #value_type::try_from(x.as_str()).ok()).flatten();
                }
              } else {
                quote! {
                  let #name_ident = row.get_opt::<#ty_inner, &'static str>(#name).map(|x| x.ok()).flatten().map(|x| #value_type::new(x));
                }
              }
            }
          } else {
            if f.skip_new_type {
              quote! {
                let #name_ident = row.get::<#ty_inner, &'static str>(#name).ok_or_else(||{
                  tracing::error!(#error_msg);
                  mysql_common::row::convert::FromRowError(row.clone())
                })?;
              }
            } else {
              if f.is_enum {
                quote! {
                  let #name_ident = row.get::<#ty_inner, &'static str>(#name).map(|x| #value_type::try_from(x.as_str())).ok_or_else(||{
                    tracing::error!(#error_msg);
                    mysql_common::row::convert::FromRowError(row.clone())
                  })?.map_err(|e| {
                    tracing::error!("{}", format!("{}, error: {}", #error_msg, e.to_string()));
                    mysql_common::row::convert::FromRowError(row.clone())
                  })?;
                }
              } else {
                quote! {
                  let #name_ident = row.get::<#ty_inner, &'static str>(#name).map(|x| #value_type::new(x)).ok_or_else(||{
                    tracing::error!(#error_msg);
                    mysql_common::row::convert::FromRowError(row.clone())
                  })?;
                }
              }
            }
          }
        })
        .collect::<Vec<_>>();

  let build_entity_items = meta.fields
                               .all_fields()
                               .iter()
                               .map(|f| {
                                 let name = format_ident!("{}", f.name);
                                 quote! {
                                   .#name(#name)
                                 }
                               })
                               .collect::<Vec<_>>();

  let extract_params_tokens_stream = if let Some(fun) = meta.params_extractor {
    let src = format!("{}(&params)", fun);
    let tracing_debug = format!(r#"tracing::debug!("{{}}, {{}}", sql, {});"#, src);
    TokenStream::from_str(tracing_debug.as_str()).unwrap()
  } else {
    TokenStream::new()
  };

  let stmt_token_stream = quote! {
    pub trait #stmt_indent {
      fn #stmt_select_ident(&self) -> &'static str {
        #select_sql
      }
      fn #stmt_count_ident(&self) -> &'static str {
        #count_sql
      }
      fn #stmt_get_ident(&self) -> &'static str {
        #get_sql
      }
      fn #stmt_insert_ident(&self) -> &'static str {
        #insert_sql
      }
      fn #stmt_update_ident(&self) -> &'static str {
        #update_sql
      }
      fn #stmt_update_by_id_ident(&self) -> &'static str {
        #update_by_id_sql
      }
      fn #stmt_delete_ident(&self) -> &'static str {
        #delete_sql
      }
      fn #stmt_delete_by_id_ident(&self) -> &'static str {
        #delete_by_id_sql
      }
      fn #entity_params_fn_name(&self, entity: &#entity_name_ident) -> mysql_async::Params {
        let mut param_map = std::collections::HashMap::new();
        #(#sql_param_items)*
        mysql_async::Params::Named(param_map)
      }
    }
  };
  let method_token_stream = quote! {
    #[async_trait::async_trait]
    pub trait #method_indent:#stmt_indent {
      async fn #insert_fn_name<'a, 't: 'a, C>(&self, entity: &#entity_name_ident, conn: C) -> mysql_async::Result<()>
      where C: mysql_async::prelude::ToConnection<'a, 't> + 'a
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#stmt_insert_ident();
        let params = self.#entity_params_fn_name(entity);
        #extract_params_tokens_stream
        sql.with(params).ignore(conn).await
      }
      async fn #select_fn_name<'a, 't: 'a, C, S, T>(&self, condition: S, params: mysql_async::prelude::params::Params, conn: C) -> mysql_async::Result<Vec<T>>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             S: Into<String> + Send,
             T: mysql_async::prelude::FromRow + Send + 'static,
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#stmt_select_ident();
        let condition: String = condition.into();
        let sql = format!("{} {}", sql, condition);
        #extract_params_tokens_stream
        sql.with(params).fetch(conn).await
      }

      async fn #count_fn_name<'a, 't: 'a, C, S>(&self, condition: S, params: mysql_async::prelude::params::Params, conn: C) -> mysql_async::Result<u64>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             S: Into<String> + Send,
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#stmt_count_ident();
        let condition: String = condition.into();
        let sql = format!("{} {}", sql, condition);
        #extract_params_tokens_stream
        let xs: Vec<u64> = sql.with(params).fetch(conn).await?;
        Ok(xs.first().map(|x|x.clone()).unwrap_or(0u64))
      }

      async fn #delete_where_fn_name<'a, 't: 'a, C, S>(&self, condition: S, params: mysql_async::prelude::params::Params, conn: C) -> mysql_async::Result<()>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             S: Into<String> + Send,
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#stmt_delete_ident();
        let condition: String = condition.into();
        let sql = format!("{} {}", sql, condition);
        #extract_params_tokens_stream
        sql.with(params).ignore(conn).await
      }

      async fn #delete_by_id_fn_name<'a, 't: 'a, C>(&self, id: &#entity_id_ident, conn: C) -> mysql_async::Result<()>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
      {
        use mysql_async::prelude::{Query, WithParams, params};
        let sql = self.#stmt_delete_by_id_ident();
        let params = params! {"id" => id.inner()};
        #extract_params_tokens_stream
        sql.with(params).ignore(conn).await
      }

      async fn #get_fn_name<'a, 't: 'a, C, T>(&self, id: &#entity_id_ident, conn: C) -> mysql_async::Result<Option<T>>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             T: mysql_async::prelude::FromRow + Send + 'static,
      {
        use mysql_async::prelude::{Query, WithParams, params};
        let sql = self.#stmt_select_ident();
        let sql = format!("{} WHERE id = :id", sql);
        let params = params! {"id" => id.inner()};
        #extract_params_tokens_stream
        sql.with(params).first(conn).await
      }

      async fn #update_fn_name<'a, 't: 'a, C>(&self, entity: &#entity_name_ident, conn: C) -> mysql_async::Result<()>
      where C: mysql_async::prelude::ToConnection<'a, 't> + 'a
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#stmt_update_by_id_ident();
        let params = self.#entity_params_fn_name(entity);
        #extract_params_tokens_stream
        sql.with(params).ignore(conn).await
      }
    }

    // FromRow
    impl mysql_async::prelude::FromRow for #entity_name_ident {
      fn from_row_opt(row: mysql_common::row::Row) -> Result<Self, mysql_common::row::convert::FromRowError> where Self: Sized {
        #(#get_row_items)*
        Ok(#entity_name_ident::builder()
          #(#build_entity_items)*
          .build()
        )
      }
    }
  };

  let mut all_stream = vec![];

  if meta.gen_stmt {
    all_stream.push(stmt_token_stream);
  }
  if meta.gen_method {
    all_stream.push(method_token_stream);
  }

  quote! {
    #(#all_stream)*
  }
}
