use {crate::{error::{Error,
                     Result},
             feature::{FeatureEntityCRUDExec,
                       FeatureEntityCRUDStmt},
             operator::{OperatorEntityCRUDExec,
                        OperatorEntityCRUDStmt,
                        OperatorRoleEntityCRUDExec,
                        OperatorRoleEntityCRUDStmt},
             role::{RoleEntityCRUDExec,
                    RoleEntityCRUDStmt,
                    RoleFeatureEntityCRUDExec,
                    RoleFeatureEntityCRUDStmt},
             session::{SessionEntityCRUDExec,
                       SessionEntityCRUDStmt}},
     mysql_async::{Conn,
                   Pool}};

#[derive(Clone)]
pub struct Store {
  pub(crate) pool: Pool,
}

impl Store {
  pub async fn get_conn(&self) -> Result<Conn> { self.pool.get_conn().await.map_err(|e| Error::DatabaseError(e.to_string())) }
}

impl FeatureEntityCRUDStmt for Store {}
impl FeatureEntityCRUDExec for Store {}

impl OperatorEntityCRUDStmt for Store {}
impl OperatorEntityCRUDExec for Store {}

impl OperatorRoleEntityCRUDStmt for Store {}
impl OperatorRoleEntityCRUDExec for Store {}

impl RoleEntityCRUDStmt for Store {}
impl RoleEntityCRUDExec for Store {}

impl RoleFeatureEntityCRUDStmt for Store {}
impl RoleFeatureEntityCRUDExec for Store {}

impl SessionEntityCRUDStmt for Store {}
impl SessionEntityCRUDExec for Store {}

mod create;
mod delete;
mod query;

pub use {create::StoreCreate,
         delete::StoreDelete,
         query::StoreQuery};
