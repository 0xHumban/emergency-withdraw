use ethers::{
    abi::Address,
    providers::Middleware,
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
    types::U256,
    utils::format_ether,
};
use eyre::Result;

use crate::provider::HttpProvider;

/// represent a user wallet with his ETH balance
#[derive(Clone, Debug)]
pub struct Wallet {
    local_wallet: LocalWallet,
    eth_balance: U256,
}

impl Wallet {
    pub async fn new(
        provider: HttpProvider,
        wallet_builder: &MnemonicBuilder<English>,
        index: u32,
    ) -> Result<Wallet> {
        let local_wallet: LocalWallet = wallet_builder.clone().index(index)?.build()?;
        let eth_balance: U256 =
            Wallet::get_eth_balance_for_local_wallet(&local_wallet, provider).await?;
        Ok(Wallet {
            local_wallet,
            eth_balance,
        })
    }

    /// returns the balance for the localwallet givenn
    pub async fn get_eth_balance_for_local_wallet(
        local_wallet: &LocalWallet,
        provider: HttpProvider,
    ) -> Result<U256> {
        let balance = provider.get_balance(local_wallet.address(), None).await?;
        Ok(balance)
    }

    // --- transaction
    // todo

    // ---

    // --- getters
    pub fn address(&self) -> Address {
        self.local_wallet.address()
    }

    pub fn eth_balance(&self) -> U256 {
        self.eth_balance
    }

    /// convert the address to string
    pub fn address_to_string(&self) -> String {
        format!("{:#?}", self.local_wallet.address())
    }

    /// convert the eth balance (in wei) to ether
    /// returns a String
    pub fn eth_balance_to_string(&self) -> String {
        format_ether(self.eth_balance)
    }
    // ---
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{app_data::load_builder_wallet, provider::load_http_provider};

    #[tokio::test]
    async fn create_wallet() -> Result<()> {
        // --- init
        let provider = load_http_provider("TEST_PROVIDER_URL")?;

        let wallet_builder = load_builder_wallet("TEST_PHRASE_MNEMONIC", "TEST_PASSWORD")?;
        //---

        assert_eq!(
            Wallet::new(provider.clone(), &wallet_builder, 0u32)
                .await?
                .address_to_string(),
            "0x431a00da1d54c281aef638a73121b3d153e0b0f6"
        );

        assert_eq!(
            Wallet::new(provider.clone(), &wallet_builder, 1u32)
                .await?
                .address_to_string(),
            "0x995faa915cf9edb91ce9fe902cf530aca547a919"
        );

        Ok(())
    }
}
