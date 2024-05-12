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

    /// returns if the current Wallet is equal to the givenn one
    /// public address are unique, so we use it to compare
    pub fn equals(&self, wallet: &Wallet) -> bool {
        self.address() == wallet.address()
    }

    /// if the wallet is find in the list-> remove it
    /// otherwise -> add it
    pub fn toggle_wallet_in_list(&self, wallets_list: &mut Vec<Wallet>) {
        let mut res = false;
        let mut i = 0;
        while !res && i < wallets_list.len() {
            if let Some(wallet) = wallets_list.get(i) {
                if self.equals(wallet) {
                    res = true;
                }
            }

            i += 1;
        }

        if res {
            wallets_list.swap_remove(i - 1);
        } else {
            wallets_list.push((*self).clone());
        }
    }

    /// returns if the wallet is in a list
    pub fn is_wallet_in_list(&self, wallet_list: &Vec<Wallet>) -> bool {
        let mut res = false;
        let mut i = 0;
        while !res && i < wallet_list.len() {
            if let Some(wallet) = wallet_list.get(i) {
                if self.equals(wallet) {
                    res = true;
                }
            }

            i += 1;
        }

        return res;
    }

    /// calculate total balance of eth in list
    /// here we just add eth balance foreach wallet
    pub fn calculate_total_eth_balance_in_list(wallets_list: &Vec<Wallet>) -> Result<String> {
        let mut res = U256::zero();
        for i in 0..wallets_list.len() {
            res = res + wallets_list.get(i).unwrap().eth_balance();
        }

        Ok(format_ether(res))
    }

    // --- transaction
    // todo

    // ---

    // --- getters
    pub fn local_wallet(&self) -> LocalWallet {
        self.local_wallet.clone()
    }
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

    #[tokio::test]
    async fn test_wallets_equals() -> Result<()> {
        // --- init
        let provider = load_http_provider("TEST_PROVIDER_URL")?;

        let wallet_builder = load_builder_wallet("TEST_PHRASE_MNEMONIC", "TEST_PASSWORD")?;

        let w1 = Wallet::new(provider.clone(), &wallet_builder, 0u32).await?;

        let w1_bis = Wallet::new(provider.clone(), &wallet_builder, 0u32).await?;

        let w2 = Wallet::new(provider.clone(), &wallet_builder, 1u32).await?;
        //---

        assert!(w1.equals(&w1_bis));
        assert_eq!(w2.equals(&w1_bis), false);

        Ok(())
    }

    // todo test: toggle_wallet_in_list
    //            is_wallet_in_list()
    //            calculate_total_eth_balance_in_list
}
