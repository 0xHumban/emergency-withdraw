use std::sync::Arc;

use ethers::prelude::Http;
use ethers::providers::Provider;
use eyre::Result;

use crate::utils::load_env_variable;
use ethers::middleware::Middleware;

/// type alias to represent a http provider
pub type HttpProvider = Arc<Provider<Http>>;

pub fn create_http_provider(rpc_url: &str) -> Result<HttpProvider> {
    Ok(Arc::new(Provider::<Http>::try_from(rpc_url)?))
}

/// load from .env file the provider url and create instance
pub fn load_http_provider(provider_url_var_name: &str) -> Result<HttpProvider> {
    let provider_url: String = match load_env_variable(provider_url_var_name) {
        Ok(value) => value,
        _ => panic!("ERROR: NO PROVIDER URL SET (VIEW README for configuration)"),
    };
    create_http_provider(&provider_url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn tests_load_http_provider() -> Result<()> {
        assert!(load_http_provider("TEST_PROVIDER_URL").is_ok());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn panic_load_http_provider() {
        _ = load_http_provider("TEST_UNkNOW_VAR");
    }
}
