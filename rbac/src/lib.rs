mod access;
mod feature;
mod operator;
mod role;
mod session;

#[allow(unused)]
pub(crate) fn params_extractor(_params: &mysql_async::Params) -> String { "No dump".to_string() }
