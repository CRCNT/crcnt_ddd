use {crate::domain_entity::test_domain::*,
     anyhow::Result,
     crcnt_ddd::value::{CreateAt,
                        Deleted,
                        UpdateAt}};

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

  // #[derive(Domain, Debug)]
  // #[domain_commands(builder)]
  // pub struct PersonEntity {
  //   id:   String,
  //   name: String,
  //   desc: Option<String>,
  // }
  //
  // #[derive(TypedBuilder)]
  // pub struct CustomerEntity {
  //   id:   String,
  //   name: String,
  //   desc: Option<String>,
  // }

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

  #[derive(Domain)]
  #[domain_commands(entity)]
  #[domain_entity(rename = "PersonEntityV1", value_type_prefix = "PE")]
  struct __Person__ {
    id:        String,
    #[domain_value(optional = true, skip_new_type = true)]
    name:      String,
    #[domain_value(optional = true)]
    age:       u32,
    #[domain_value(enums = "Active|Inactive")]
    status:    String,
    create_at: CreateAt,
    update_at: UpdateAt,
    deleted:   Deleted,
  }

  // #[derive(TypedBuilder)]
  // struct TestPublic {
  //   a: String,
  //   b: String,
  //   c: String,
  // }
}

#[tokio::test]
async fn test_entity() -> Result<()> {
  let ent = PersonEntityV1::builder().id("peid".into())
                                     .name(Some("zenas".into()))
                                     .age(Some(10u32.into()))
                                     .status(PEStatus::Active)
                                     .create_at(CreateAt::now().into())
                                     .update_at(UpdateAt::now().into())
                                     .deleted(Deleted::new(true).into())
                                     .build();
  let new_id = PEId::new("2");
  println!("ent: {:?}", ent);
  eprintln!("new_id: {:?}", new_id);
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
