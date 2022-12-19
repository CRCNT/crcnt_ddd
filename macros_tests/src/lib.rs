use crcnt_ddd_macros::DomainEntityAndValues;

#[allow(dead_code)]
#[derive(DomainEntityAndValues, Debug, Clone)]
struct CustomerDTO {
  name:    String,
  my_name: Option<Result<String, String>>,
}

#[test]
fn test_macro() {
  let customer_name = CustomerName::new("Zenas");
  let customer_my_name = CustomerMyName(Ok("Zenas".to_string()));
  println!("{:?}", customer_name);
  println!("{:?}", customer_my_name);
  let entity = CustomerEntity { name:    customer_name,
                                my_name: Some(customer_my_name), };

  println!("{:?}", entity);
  println!("{:?}", entity.ref_my_name());
  let entity = entity.set_name(CustomerName("Zenas1".to_string()));
  println!("{:?}", entity.ref_name());
}
