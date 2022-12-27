use {crate::initializer::login,
     anyhow::Result,
     crcnt_ddd::value::Owner,
     crcnt_rbac::includes::{FeatureCode,
                            FeatureDescription,
                            FeatureName,
                            OperatorName,
                            RBACApplicationFeatureAdmin,
                            RBACApplicationOperatorAdmin,
                            RBACApplicationRoleAdmin,
                            RBACApplicationSessionAdmin,
                            RoleCode,
                            RoleLevel,
                            RoleName,
                            SessionId},
     tracing::info};

mod initializer;

#[tokio::test]
async fn test_add_operator() -> Result<()> {
  let app = initializer::init();
  let session = login(&app).await?;
  let session_id: SessionId = session.ref_id().clone();
  let name = OperatorName::new("admin");
  let owner = Owner::new("PROMO");
  let operator = app.create_operator_with_login_name(session_id, owner, name).await?;
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
  let owner = Owner::new("PROMO");
  let code: RoleCode = "SYS_ADMIN".into();
  let name: RoleName = "system administrator".into();
  let description = None;
  let level = RoleLevel::new(0);
  let role = app.create_role(session.mv_id(), owner, code, name, level, description).await?;
  info!("created role: {:?}", role);
  Ok(())
}

#[tokio::test]
async fn test_set_role_features() -> Result<()> { todo!() }

#[tokio::test]
async fn test_set_role_operators() -> Result<()> { todo!() }

#[tokio::test]
async fn test_get_operator_features() -> Result<()> { todo!() }
