use {crate::domain_entity::test_domain::{MyName,
                                         Name,
                                         Status},
     anyhow::Result};

mod test_macro {
  #[allow(unused_macros)]
  #[macro_export]
  macro_rules! domain_value {
    ($type_name: ident, $inner_type_name: ident) => {
      #[derive(Debug, Clone, crcnt_ddd_macros::Domain)]
      #[domain_commands(value)]
      pub struct $type_name(pub $inner_type_name);
      impl $type_name {
        pub fn value(&self) -> &$inner_type_name { &self.0 }
      }
    };
    ($type_name: ident, $inner_type_name:ident <$t: ident>) => {
      #[derive(Debug, Clone, crcnt_ddd_macros::Domain)]
      #[domain_commands(value)]
      pub struct $type_name(pub $inner_type_name<$t>);
      impl $type_name {
        pub fn value(&self) -> &$inner_type_name<$t> { &self.0 }
      }
    };
  }
}

mod test_domain {
  use {crate::{domain_entity::test_domain::Status::{Active,
                                                    Inactive},
               domain_value},
       crcnt_ddd_macros::Domain};

  #[derive(Domain, Debug, Clone)]
  #[domain_commands(value)]
  #[domain_value_impl(inner, into_inner, from)]
  pub struct Name(String);

  #[derive(Domain, Debug)]
  #[domain_commands(value)]
  #[domain_value_impl(inner, into_inner, from)]
  pub enum Status {
    Active,
    Inactive,
  }

  domain_value!(MyName, String);
  // impl<'a> TryFrom<&'a str> for Status {
  // type Error = String;
  //
  // fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
  // use Status::*;
  // match value {
  // | x => Err(format!("{}", x)),
  // }
  // }
  // }
}

#[tokio::test]
async fn test_entity() -> Result<()> {
  let status = Status::Inactive;
  let status = status.inner();
  let status = Status::try_from(status);
  println!("{:?}", status);

  let name = Name::from("a");
  println!("{:?}", name);

  let my_name = MyName::from("my_name");
  println!("my_name: {:?}", my_name);
  Ok(())
}
