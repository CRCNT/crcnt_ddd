use {crate::attributes::BuilderMeta,
     proc_macro2::TokenStream,
     quote::quote,
     syn::DeriveInput};

pub fn generate_builder(derive_input: &DeriveInput) -> TokenStream {
  let meta = BuilderMeta::parse(derive_input);
  let ident = &meta.ident;
  let builder_ident = meta.builder_ident();

  let init_generic = meta.fields.init_builder_generic();
  let builder_generic = meta.builder_generic();
  let all_fields_generic_type = meta.fields.all_builder_generic();
  let all_fields_exact_type = meta.fields.all_builder_exact_types();
  let all_extracted_fields = meta.fields.extract_all_fields();

  let build_fns = meta.fields
                      .fields()
                      .iter()
                      .map(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        let exact_generic = &meta.fields.exact_generic_stepped(f);
                        let (l, f) = &meta.fields.extract_fields(f);
                        quote! {
                          pub fn #ident(self, #ident: #ty) -> #builder_ident<(#exact_generic)> {
                            let (#l) = self.fields;
                            #builder_ident {
                              fields: (#f),
                              phantom: ()
                            }
                          }
                        }
                      })
                      .collect::<Vec<_>>();
  // let first_field = meta.fields.fields().first().map(|x| x.clone()).unwrap();
  let mut all_generics = meta.fields
                             .fields()
                             .iter()
                             .map(|x| {
                               let ty = &x.generic_ty;
                               quote! {
                                 #ty,
                               }
                             })
                             .collect::<Vec<_>>();
  let mut all_set_tys = vec![];
  let build_error_fns = meta.fields
                            .fields()
                            .iter()
                            .map(|f| {
                              let missed_type = &f.missed_tip_ty;
                              let missed_type_str = missed_type.to_string();
                              let set_ty = (&f.ty).clone();
                              let set_ty = quote! {
                                #set_ty,
                              };
                              let _ = all_generics.remove(0);

                              let err_generics = quote! {
                                #(#all_generics)*
                              };
                              let exact_tys = quote! {
                                #(#all_set_tys)*
                                (),
                                #err_generics
                              };

                              all_set_tys.push(set_ty);
                              quote! {
                                impl <#err_generics> #builder_ident<(#exact_tys)> {
                                  #[deprecated(note = #missed_type_str)]
                                  pub fn build(self, _: #missed_type) -> #ident {
                                    panic!(#missed_type_str)
                                  }
                                }
                              }
                            })
                            .collect::<Vec<_>>();

  let missed_tip_enums = meta.fields
                             .fields()
                             .iter()
                             .map(|f| {
                               let ty = &f.missed_tip_ty;
                               quote! {
                                 #[doc(hidden)]
                                 #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
                                 pub enum #ty {}
                               }
                             })
                             .collect::<Vec<_>>();

  quote! {
    impl #ident {
      pub fn builder() -> #builder_ident<#init_generic> {
        #builder_ident {
          fields: #init_generic,
          phantom: ()
        }
      }
    }
    pub struct #builder_ident<#builder_generic> {
      fields: #builder_generic,
      phantom: ()
    }
    impl <#all_fields_generic_type> #builder_ident<(#all_fields_generic_type)> {
      #(#build_fns)*
    }
    impl #builder_ident<(#all_fields_exact_type)> {
      pub fn build(self) -> #ident {
        let (#all_extracted_fields) = self.fields;
        #ident {#all_extracted_fields}
      }
    }
    #(#missed_tip_enums)*
    #(#build_error_fns)*
  }
}
