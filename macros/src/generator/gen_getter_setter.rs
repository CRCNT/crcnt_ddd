use {crate::attributes::GetterSetterMeta,
     proc_macro2::TokenStream,
     quote::{format_ident,
             quote},
     syn::DeriveInput};

pub fn generate_getter_setter(derive_input: &DeriveInput) -> TokenStream {
  let meta = GetterSetterMeta::parse(derive_input);
  let ident = &meta.ident;
  let getters = meta.raw_types
                    .iter()
                    .map(|rt| {
                      let f_ident = &rt.ident;
                      let ref_ident = format_ident!("ref_{}", rt.ident);
                      let set_ident = format_ident!("set_{}", rt.ident);
                      let mv_ident = format_ident!("mv_{}", rt.ident);
                      let r_ty = &rt.ty;
                      //<editor-fold desc="Getter && Setter Quotes">
                      quote! {
                        pub fn #ref_ident(&self) -> &#r_ty {
                          &self.#f_ident
                        }
                        pub fn #mv_ident(self) -> #r_ty {
                          self.#f_ident
                        }
                        pub fn #set_ident(self, #f_ident: #r_ty) -> Self {
                          Self {
                            #f_ident,
                            ..self
                          }
                        }
                      }
                      //</editor-fold>
                    })
                    .collect::<Vec<_>>();
  quote! {
    impl #ident {
      #(#getters)*
    }
  }
}
