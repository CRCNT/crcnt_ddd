use mysql_common::{params::Params,
                   value::Value};

pub fn params_inspect(params: &Params) -> String {
  fn value_to_string(v: &Value) -> String {
    match v {
      | Value::NULL => "NULL".to_string(),
      | Value::Bytes(b) => String::from_utf8_lossy(b).to_string(),
      | Value::Int(i) => i.to_string(),
      | Value::UInt(i) => i.to_string(),
      | Value::Float(f) => f.to_string(),
      | Value::Double(d) => d.to_string(),
      | Value::Date(y, m, d, h, mm, s, ms) => {
        format!("{}-{}-{} {}:{}:{},{}", y, m, d, h, mm, s, ms)
      }
      | Value::Time(is_neg, d, h, m, s, ms) => {
        format!("{}{},{}:{}:{},{}", if *is_neg { "-" } else { "+" }, d, h, m, s, ms)
      }
    }
  }
  let xs = match params {
    | Params::Empty => "Empty Params".to_string(),
    | Params::Named(ref named) => {
      let xs = named.iter()
                    .map(|(k, v)| {
                      let k = String::from_utf8_lossy(k);
                      let v = value_to_string(v);
                      format!("{}:{}", k, v)
                    })
                    .collect::<Vec<_>>()
                    .join(",");
      xs
    }
    | Params::Positional(ref values) => values.iter().map(|v| value_to_string(v)).collect::<Vec<_>>().join(","),
  };
  format!("Params({})", xs)
}
