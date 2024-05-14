use dotenv::dotenv;
use eyre::Result;
use std::env;

/// returns the value of the env variable
pub fn load_env_variable(var_name: &str) -> Result<String> {
    dotenv().ok();
    Ok(env::var(var_name)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn tests_load_env_variable() -> Result<()> {
        assert!(load_env_variable("TEST_PHRASE_MNEMONIC").is_ok());
        assert!(load_env_variable("TEST_UNKNOW_VAR").is_err());

        Ok(())
    }
}
