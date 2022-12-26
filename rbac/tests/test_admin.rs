use {anyhow::Result,
     crcnt_ddd::value::Owner,
     crcnt_rbac::includes::{OperatorName,
                            OperatorPassword,
                            RBACApplicationCreate,
                            RBACApplicationSessionLogin,
                            SessionId},
     tracing::info};

mod initializer;

#[tokio::test]
async fn test_add_operator() -> Result<()> {
  let app = initializer::init();

  let owner = Owner::new("SYS");
  let name = OperatorName::new("admin");
  let password = OperatorPassword::new("passw0rd!");

  let session = app.login_with_name_password(owner, name.clone(), password.clone()).await?;

  let session: SessionId = session.ref_id().clone();

  let owner = Owner::new("PROMO");
  let operator = app.create_operator_with_login_name(session, owner, name).await?;

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
async fn test_add_feature() -> Result<()> { todo!() }

#[tokio::test]
async fn test_add_role() -> Result<()> { todo!() }

#[tokio::test]
async fn test_set_role_features() -> Result<()> { todo!() }

#[tokio::test]
async fn test_set_role_operators() -> Result<()> { todo!() }

#[tokio::test]
async fn test_get_operator_features() -> Result<()> { todo!() }
