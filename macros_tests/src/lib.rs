mod domain;

use domain::*;

#[test]
fn test_macro() {
  let entity = CustomerInfoEntity::builder().id("01234")
                                            .name("zenas")
                                            .create_at(CreateAt(10u64))
                                            .update_at(0u64)
                                            .description(Some("Hello"))
                                            .build();

  println!("{:?}", entity);

  let entity = entity.unwrap();
  let id = entity.ref_id().clone().into_inner();
  let name = entity.move_id().into_inner();

  println!("id = {}", id);
  println!("name = {}", name);
}
