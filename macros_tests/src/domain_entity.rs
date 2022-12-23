use anyhow::Result;

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
  use {crate::domain_value,
       crcnt_ddd::value::{CreateAt,
                          Deleted,
                          UpdateAt},
       crcnt_ddd_macros::Domain,
       typed_builder::TypedBuilder};

  #[derive(Domain, Debug)]
  #[domain_commands(builder)]
  pub struct PersonEntity {
    id:   String,
    name: String,
    desc: Option<String>,
  }

  #[derive(TypedBuilder)]
  pub struct CustomerEntity {
    id:   String,
    name: String,
    desc: Option<String>,
  }

  // #[derive(Domain, Debug, Clone)]
  // #[domain_commands(value)]
  // #[domain_value_impl(inner, into_inner, from)]
  // pub struct Name(String);

  // #[derive(Domain, Debug, Clone, PartialOrd, PartialEq)]
  // #[domain_commands(value)]
  // #[domain_value_impl(inner, into_inner, from)]
  // pub enum Status {
  //   Active,
  //   Inactive,
  // }

  // #[derive(Domain)]
  // #[domain_commands(entity)]
  // #[domain_entity(rename = "PersonEntityV1", value_type_prefix = "PE")]
  // struct __Person__ {
  //   id:        String,
  //   #[domain_value(optional = true, skip_new_type = true)]
  //   name:      String,
  //   #[domain_value(optional = true)]
  //   age:       u32,
  //   #[domain_value(enums = "Active|Inactive")]
  //   status:    String,
  //   create_at: CreateAt,
  //   update_at: UpdateAt,
  //   deleted:   Deleted,
  // }

  // domain_value!(MyName, String);
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
use test_domain::PersonEntity;
#[tokio::test]
async fn test_entity() -> Result<()> {
  let person_entity = PersonEntity::builder().build();
  //.build();
  // let builder = BTestBuilder::builder();
  // let bt = builder.id("1".to_string()).id("2".to_string()).age(40).build("hello".to_string());
  println!("{:?}", person_entity);
  Ok(())
}

// #[derive(Debug, Clone)]
// struct BTest {
//   id:   String,
//   name: String,
//   age:  u32,
// }
//
// struct BTestBuilder<AllFieldsType> {
//   fields:  AllFieldsType,
//   phantom: (),
// }
//
// impl BTestBuilder<((), (), ())> {
//   fn builder() -> Self {
//     Self { fields:  ((), (), ()),
//            phantom: (), }
//   }
// }
//
// impl BTestBuilder<(String, String, u32)> {
//   fn build(self) -> BTest {
//     let (id, name, age) = self.fields;
//     BTest { id, name, age }
//   }
// }
//
// impl<__name, __age> BTestBuilder<(String, __name, __age)> {
//   #[deprecated(note = "Missing Field: name")]
//   fn build(self, _: String) -> BTest { panic!("Missing Field: name") }
// }
//
// impl<__id, __name, __age> BTestBuilder<(__id, __name, __age)> {
//   fn id(self, id: String) -> BTestBuilder<(String, __name, __age)> {
//     let (_, name, age) = self.fields;
//     BTestBuilder { fields:  (id, name, age),
//                    phantom: (), }
//   }
//
//   fn name(self, name: String) -> BTestBuilder<(__id, String, __age)> {
//     let (id, _, age) = self.fields;
//     BTestBuilder { fields:  (id, name, age),
//                    phantom: (), }
//   }
//
//   fn age(self, age: u32) -> BTestBuilder<(__id, __name, u32)> {
//     let (id, name, _) = self.fields;
//     BTestBuilder { fields:  (id, name, age),
//                    phantom: (), }
//   }
// }
