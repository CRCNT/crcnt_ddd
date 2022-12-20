use {crate::{ast::value::DomainValueAttr,
             utils::{is_type_option,
                     type_in_option_or_itself,
                     value_type,
                     DomainDefAst}},
     convert_case::{Case,
                    Casing},
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn gen_store(ast: &DomainDefAst) -> TokenStream {
  let entity_name_ident = format_ident!("{}Entity", ast.root_name_ident);
  // let store_name_ident = format_ident!("{}Store", ast.root_name_ident);
  let basic_store_helper_ident = format_ident!("{}BasicStoreHelper", ast.root_name_ident);
  let table_name = format!("t_{}", ast.root_name_ident.to_string().to_case(Case::Snake));
  let field_names = ast.fields_named
                       .named
                       .iter()
                       .map(|f| (f.ident.as_ref().unwrap().to_string()).clone())
                       .collect::<Vec<String>>();
  let value_names = ast.fields_named
                       .named
                       .iter()
                       .map(|f| format!(":{}", (f.ident.as_ref().unwrap())))
                       .collect::<Vec<String>>();
  let set_values = ast.fields_named
                      .named
                      .iter()
                      .map(|f| format!("{} = :{}", (f.ident.as_ref().unwrap()), (f.ident.as_ref().unwrap())))
                      .collect::<Vec<_>>();
  let sql_insert = format!("INSERT INTO {} ({}) VALUES ({})",
                           table_name,
                           field_names.join(", "),
                           value_names.join(", "));
  let sql_update = format!("UPDATE {} SET {} WHERE id = :id", table_name, set_values.join(", "));
  let sql_delete = format!("DELETE FROM {} WHERE id = :id", table_name);
  let sql_select = format!("SELECT {} FROM {}", field_names.join(", "), table_name);

  let sql_param_items =
    ast.fields_named
       .named
       .iter()
       .map(|f| {
         let name = &f.ident.as_ref().unwrap();
         let getter_name = format_ident!("ref_{}", name);
         let is_option = is_type_option(&f.ty);
         let name_string = name.to_string();
         if is_option {
           quote! {
             param_map.insert(Vec::<u8>::from(#name_string), mysql_async::Value::from(entity.#getter_name().as_ref().map(|x|x.inner())));
           }
         } else {
           quote! {
             param_map.insert(Vec::<u8>::from(#name_string), mysql_async::Value::from(entity.#getter_name().inner()));
           }
         }
       })
       .collect::<Vec<_>>();

  let sql_insert_fn_name = format_ident!("sql_insert_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let insert_fn_name = format_ident!("exec_insert_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let sql_update_fn_name = format_ident!("sql_update_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let update_fn_name = format_ident!("exec_update_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let sql_select_fn_name = format_ident!("sql_select_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let select_fn_name = format_ident!("exec_select_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let get_fn_name = format_ident!("exec_get_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let sql_delete_fn_name = format_ident!("sql_delete_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));
  let entity_params_fn_name = format_ident!("mysql_params_{}", &ast.root_name_ident.to_string().to_case(Case::Snake));

  let get_row_items = ast.fields_named
                         .named
                         .iter()
                         .map(|f| {
                           let name = &f.ident.as_ref().unwrap();
                           let name_str = name.to_string();

                           let value_type = value_type(&ast.root_name_ident, f);
                           let is_opt = is_type_option(&f.ty);
                           let ty_inner = type_in_option_or_itself(f.ty.clone());
                           let error_msg = format!("can't get {} from row", name_str);
                           let skip = DomainValueAttr::parse_from(f).skip;
                           if is_opt {
                             if skip {
                               quote! {
                                 let #name = row.get_opt::<#ty_inner, &'static str>(#name_str).map(|x| x.ok()).flatten();
                               }
                             } else {
                               quote! {
                                 let #name = row.get_opt::<#ty_inner, &'static str>(#name_str).map(|x| x.ok()).flatten().map(|x| #value_type::new(x));
                               }
                             }
                           } else {
                             if skip {
                               quote! {
                                 let #name = row.get::<#ty_inner, &'static str>(#name_str).ok_or_else(||{
                                   tracing::info!(#error_msg);
                                   mysql_common::row::convert::FromRowError(row.clone())
                                 })?;
                               }
                             } else {
                               quote! {
                                 let #name = row.get::<#ty_inner, &'static str>(#name_str).map(|x| #value_type::new(x)).ok_or_else(||{
                                   tracing::info!(#error_msg);
                                   mysql_common::row::convert::FromRowError(row.clone())
                                 })?;
                               }
                             }
                           }
                         })
                         .collect::<Vec<_>>();
  let build_entity_items = ast.fields_named
                              .named
                              .iter()
                              .map(|f| {
                                let name = &f.ident.as_ref().unwrap();
                                quote! {
                                  .#name(#name)
                                }
                              })
                              .collect::<Vec<_>>();

  let entity_id_ident = format_ident!("{}Id", ast.root_name_ident);
  quote! {
    #[async_trait::async_trait]
    pub trait #basic_store_helper_ident {
      fn #sql_insert_fn_name(&self) -> &'static str {
        #sql_insert
      }
      fn #sql_update_fn_name(&self) -> &'static str {
        #sql_update
      }
      fn #sql_select_fn_name(&self) -> &'static str {
        #sql_select
      }
      fn #sql_delete_fn_name(&self) -> &'static str {
        #sql_delete
      }
      fn #entity_params_fn_name(&self, entity: &#entity_name_ident) -> mysql_async::Params {
        let mut param_map = std::collections::HashMap::new();
        #(#sql_param_items)*
        mysql_async::Params::Named(param_map)
      }

      async fn #insert_fn_name<'a, 't: 'a, C>(&self, entity: &#entity_name_ident, conn: C) -> mysql_async::Result<()>
      where C: mysql_async::prelude::ToConnection<'a, 't> + 'a
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#sql_insert_fn_name();
        let params = self.#entity_params_fn_name(entity);
        sql.with(params).ignore(conn).await

      }
      async fn #update_fn_name<'a, 't: 'a, C>(&self, entity: &#entity_name_ident, conn: C) -> mysql_async::Result<()>
      where C: mysql_async::prelude::ToConnection<'a, 't> + 'a
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#sql_update_fn_name();
        let params = self.#entity_params_fn_name(entity);
        sql.with(params).ignore(conn).await
      }
      async fn #select_fn_name<'a, 't: 'a, C, S, T>(&self, condition: S, params: mysql_async::prelude::params::Params, conn: C) -> mysql_async::Result<Vec<T>>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             S: Into<String> + Send,
             T: mysql_async::prelude::FromRow + Send + 'static,
      {
        use mysql_async::prelude::{Query, WithParams};
        let sql = self.#sql_select_fn_name();
        let condition: String = condition.into();
        let sql = format!("{} {}", sql, condition);
        sql.with(params).fetch(conn).await
      }
      async fn #get_fn_name<'a, 't: 'a, C, T>(&self, id: &#entity_id_ident, conn: C) -> mysql_async::Result<Option<T>>
      where  C: mysql_async::prelude::ToConnection<'a, 't> + 'a,
             T: mysql_async::prelude::FromRow + Send + 'static,
      {
        use mysql_async::prelude::{Query, WithParams, params};
        let sql = self.#sql_select_fn_name();
        let sql = format!("{} WHERE id = :id", sql);
        let params = params! {"id" => id.inner()};
        sql.with(params).first(conn).await
      }
    }
    // FromRow
    impl mysql_async::prelude::FromRow for #entity_name_ident {
      fn from_row_opt(row: mysql_common::row::Row) -> Result<Self, mysql_common::row::convert::FromRowError> where Self: Sized {
        #(#get_row_items)*
        Ok(#entity_name_ident::builder()
          #(#build_entity_items)*
          .unsafe_build()
        )
      }
    }
  }
}
