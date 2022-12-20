use {crate::utils::DomainDefAst,
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote}};

pub fn gen_dto(ast: &DomainDefAst) -> TokenStream {
  let dto_name_ident = format_ident!("{}DTO", ast.root_name_ident);
  let dto_fields = ast.fields_named
                      .named
                      .iter()
                      .map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote! {
                          pub #name: #ty,
                        }
                      })
                      .collect::<Vec<_>>();
  quote! {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct #dto_name_ident {
      #(#dto_fields)*
    }
    impl #dto_name_ident {
      pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
      }
      pub fn unsafe_to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
      }
    }
  }
}
