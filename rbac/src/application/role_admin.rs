use {crate::application::Application,
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationRoleAdmin {}

#[async_trait]
impl ApplicationRoleAdmin for Application {}
