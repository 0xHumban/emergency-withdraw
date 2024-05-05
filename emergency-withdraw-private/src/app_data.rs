use core::panic;

use ethers::{
    signers::{coins_bip39::English, MnemonicBuilder},
    types::Address,
};
use eyre::Result;

use crate::{
    provider::{load_http_provider, HttpProvider},
    utils::load_env_variable,
    wallet::Wallet,
};

/// represent / store the data used in the application
/// view if add timestamp of latest update (eth_balance)
#[derive(Clone, Debug)]
pub struct AppData {
    wallet_builder: MnemonicBuilder<English>,
    wallets: Vec<Wallet>,
    next_index: u32,
    to_address: Address,
    provider: HttpProvider,
}

impl AppData {
    pub fn new(
        wallet_builder: MnemonicBuilder<English>,
        wallets: Vec<Wallet>,
        next_index: u32,
        to_address: Address,
        provider: HttpProvider,
    ) -> Self {
        AppData {
            wallet_builder,
            wallets,
            next_index,
            to_address,
            provider,
        }
    }

    /// create Wallet instance for amount asked for
    pub async fn create_wallets_list(
        number: u32,
        provider: HttpProvider,
        wallet_builder: MnemonicBuilder<English>,
    ) -> Result<Vec<Wallet>> {
        let mut wallets: Vec<Wallet> = Vec::new();

        for i in 0..number {
            wallets.push(Wallet::new(provider.clone(), &wallet_builder, i).await?);
        }
        Ok(wallets)
    }

    /// load from .env file the data and create instance
    pub async fn load_appdata(
        provider_var_name: &str,
        phrase_var_name: &str,
        password_var_name: &str,
        to_var_name: &str,
        wallets_number_var_name: &str,
    ) -> Result<AppData> {
        let wallet_builder = load_builder_wallet(phrase_var_name, password_var_name)?;
        let provider = load_http_provider(provider_var_name)?;
        let to_address = load_to_address(to_var_name)?;
        let number_of_wallets_to_load = load_wallets_number(wallets_number_var_name)?;
        let wallets = AppData::create_wallets_list(
            number_of_wallets_to_load,
            provider.clone(),
            wallet_builder.clone(),
        )
        .await?;
        Ok(AppData::new(
            wallet_builder,
            wallets,
            number_of_wallets_to_load,
            to_address,
            provider,
        ))
    }

    // --- getters
    pub fn wallet_builder(&self) -> MnemonicBuilder<English> {
        self.wallet_builder.clone()
    }

    pub fn wallets(&self) -> Vec<Wallet> {
        self.wallets.clone()
    }

    pub fn provider(&self) -> HttpProvider {
        self.provider.clone()
    }

    pub fn next_index(&self) -> u32 {
        self.next_index
    }

    pub fn to_address(&self) -> Address {
        self.to_address
    }

    /// convert the address to string
    pub fn to_address_to_string(&self) -> String {
        format!("{:#?}", self.to_address())
    }

    // ---
}

// --- LOAD PART

/// load from .env file the number of wallets to load and return it
pub fn load_wallets_number(wallets_number_var_name: &str) -> Result<u32> {
    match load_env_variable(wallets_number_var_name) {
        Ok(value) => Ok(value.parse::<u32>()?),
        _ => panic!("ERROR: NO NUMBER OF WALLET TO LOAD (VIEW README for configuration)"),
    }
}

/// load from .env file the address to send ether and create instance
pub fn load_to_address(to_var_name: &str) -> Result<Address> {
    match load_env_variable(to_var_name) {
        Ok(address) => Ok(address.parse::<Address>()?),
        _ => panic!("ERROR: NO TO ADDRESS (VIEW README for configuration)"),
    }
}

/// load from .env file the phrase and (Optional password) and create instance
pub fn load_builder_wallet(
    phrase_var_name: &str,
    password_var_name: &str,
) -> Result<MnemonicBuilder<English>> {
    let phrase: String = match load_env_variable(phrase_var_name) {
        Ok(value) => value,
        _ => panic!("ERROR: NO MNEMONIC PHRASE SET (VIEW README for configuration)"),
    };

    let mut wallet_builder = MnemonicBuilder::<English>::default().phrase(phrase.as_str());

    if let Ok(password) = load_env_variable(password_var_name) {
        wallet_builder = wallet_builder.password(&password);
    }
    Ok(wallet_builder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn tests_load_builder_wallet() -> Result<()> {
        assert!(load_builder_wallet("TEST_PHRASE_MNEMONIC", "TEST_PASSWORD").is_ok());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn panic_load_builder_wallet() {
        _ = load_builder_wallet("TEST_UNkNOW_VAR", "TEST_PASSWORD");
    }

    #[test]
    fn tests_load_to_address() -> Result<()> {
        assert!(load_to_address("TEST_TO_ADDRESS").is_ok());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn panic_load_to_address() {
        _ = load_to_address("TEST_UNKNOW_VARIABLE");
    }

    #[test]
    fn tests_load_wallets_number() -> Result<()> {
        assert!(load_wallets_number("TEST_WALLETS_NUMBER").is_ok());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn panic_load_wallets_number() {
        _ = load_wallets_number("TEST_UNKNOW_VARIABLE");
    }

    #[tokio::test]
    pub async fn test_create_wallets_list() -> Result<()> {
        // --- init
        let provider = load_http_provider("TEST_PROVIDER_URL")?;

        let wallet_builder = load_builder_wallet("TEST_PHRASE_MNEMONIC", "TEST_PASSWORD")?;

        let number_of_wallets_to_load = load_wallets_number("TEST_WALLETS_NUMBER")?;
        //---

        let wallets: Vec<Wallet> =
            AppData::create_wallets_list(number_of_wallets_to_load, provider, wallet_builder)
                .await?;

        assert_eq!(wallets.len(), number_of_wallets_to_load as usize);

        Ok(())
    }

    #[tokio::test]
    pub async fn test_load_appdata() -> Result<()> {
        assert!(AppData::load_appdata(
            "TEST_PROVIDER_URL",
            "TEST_PHRASE_MNEMONIC",
            "TEST_PASSWORD",
            "TEST_TO_ADDRESS",
            "TEST_WALLETS_NUMBER",
        )
        .await
        .is_ok());

        Ok(())
    }
}
