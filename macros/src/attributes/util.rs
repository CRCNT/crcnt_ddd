use {proc_macro2::TokenStream,
     quote::ToTokens,
     serde::de::DeserializeOwned,
     syn::spanned::Spanned};

pub fn parse_attr_token_stream<T: DeserializeOwned>(tokens_stream: TokenStream) -> syn::Result<T> {
  let lines = tokens_stream.to_token_stream().to_string();
  let lines = lines.split(",").collect::<Vec<_>>();
  let lines = lines.iter()
                   .map(|line| {
                     let kv = line.split("=").map(|xs| xs.trim()).collect::<Vec<_>>();
                     format!("{}={}", kv[0], kv[1])
                   })
                   .collect::<Vec<_>>();
  let lines = lines.join("\n");
  let lines = lines.as_str();
  let t: syn::Result<T> = toml::from_str(lines).map_err(|e| {
                                                 eprintln!("parse toml-ish attribute failed: {}", e.to_string());
                                                 syn::Error::new(tokens_stream.span(), format!("parse toml-ish attribute failed: {}", e.to_string()))
                                               });
  t
}
