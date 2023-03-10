use {crate::initializer::{login,
                          login_with},
     anyhow::Result,
     crcnt_ddd::value::Owner,
     crcnt_rbac::includes::{FeatureCode,
                            FeatureDescription,
                            FeatureId,
                            FeatureName,
                            OperatorId,
                            OperatorName,
                            OperatorPassword,
                            RBACApplicationFeatureAdmin,
                            RBACApplicationOperatorAdmin,
                            RBACApplicationRoleAdmin,
                            RBACApplicationSessionAdmin,
                            RoleCode,
                            RoleId,
                            RoleLevel,
                            RoleName,
                            SessionId},
     tracing::info};

mod initializer;

#[tokio::test]
async fn test_add_operator() -> Result<()> {
  let app = initializer::init();
  // let session = login(&app).await?;
  // let name = OperatorName::new("admin");
  // let operator = app.create_operator_with_login_name(session.ref_id(), name).await?;
  // info!("{:?}", operator);

  // create admin operator
  let operator = app.create_admin_operator(Owner::new("TestOwner"),
                                           OperatorName::new("TestOwnerAdmin"),
                                           OperatorPassword::new("Easy"))
                    .await?;
  info!("{:?}", operator);
  Ok(())
}

#[tokio::test]
async fn test_hit_session() -> Result<()> {
  let app = initializer::init();
  let session = app.hit_session(&SessionId::new("SS01GN78WFB8CRY39C3F1QF0DVGE")).await?;
  info!("new session: {:?}", session);
  Ok(())
}

#[tokio::test]
async fn test_add_feature() -> Result<()> {
  let app = initializer::init();
  let session = login(&app).await?;

  let feature_code = FeatureCode::new("feature-admin");
  let feature_name = FeatureName::new("Feature Admin");
  let endpoint = None;
  let feature_description = FeatureDescription::new("a bunch of administration functions");
  let feature = app.create_top_feature(session.mv_id(), feature_code, feature_name, endpoint, Some(feature_description))
                   .await?;

  info!("created feature: {:?}", feature);
  Ok(())
}

#[tokio::test]
async fn test_add_role() -> Result<()> {
  let app = initializer::init();
  let session = login(&app).await?;
  let code: RoleCode = "SYS_ADMIN".into();
  let name: RoleName = "system administrator".into();
  let description = None;
  let level = RoleLevel::new(0);
  let role = app.create_role(session.mv_id(), code, name, level, description).await?;
  info!("created role: {:?}", role);
  Ok(())
}

#[tokio::test]
async fn test_set_role_features() -> Result<()> {
  let app = initializer::init();
  let session = login(&app).await?;

  let role_id = RoleId::new("RL01GNA0AQ40BWT8A01B0BPAR80B");
  let feature_id = FeatureId::new("FT01GN9QK37X45X9CS18NY5EZJDX");

  let _ = app.set_role_features(session.mv_id(), role_id, vec![feature_id]).await?;

  Ok(())
}

#[tokio::test]
async fn test_set_role_operators() -> Result<()> {
  let app = initializer::init();
  let session = login(&app).await?;

  let role_id = RoleId::new("RL01GNA0AQ40BWT8A01B0BPAR80B");
  let operator_id = OperatorId::new("OP01GNEVTRKGZC6RRJ76CQQ3782Y");

  let _ = app.set_role_operators(session.mv_id(), role_id, vec![operator_id]).await?;

  Ok(())
}

#[tokio::test]
async fn test_get_operator_features() -> Result<()> {
  let app = initializer::init();
  let session = login_with(&app, "admin".into(), "ChangeMe!".into()).await?;

  let features = app.fetch_session_features(session.ref_id()).await?;

  info!("{:?}", features);
  Ok(())
}

#[tokio::test]
async fn test_change_password() -> Result<()> {
  let app = initializer::init();
  let session = login_with(&app, "admin".into(), "ChangeMe!".into()).await?;

  let old_password = OperatorPassword::change_me();
  let new_password = OperatorPassword::new("A_newPassword!");
  let _ = app.change_password(session.ref_id(), old_password, new_password).await?;

  let session = login_with(&app, "admin".into(), "A_newPassword!".into()).await?;
  info!("new session: {:?}", session);

  Ok(())
}
