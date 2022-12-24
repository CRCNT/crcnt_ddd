use {crate::domain::application::Application,
     async_trait::async_trait};

#[async_trait]
pub trait ApplicationUpdate {}

#[async_trait]
impl ApplicationUpdate for Application {}
