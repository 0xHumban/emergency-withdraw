// file used for transfer functions
// check validity + perform transfer

use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::types::{Address, TransactionReceipt, TransactionRequest, H256, U256};
use eyre::Result;

use crate::{provider::HttpProvider, wallet::Wallet};

/// used to represent the transaction reason failed
#[derive(Clone, Debug)]
pub enum OptFailed {
    NoEnoughEther, // the wallet donesn't have enough ether to pay fees + send ether
    Reverted,      // transaction reverted
}

/// represent the success of the transaction
#[derive(Clone, Debug)]
pub struct Status {
    //success: bool, // may be useless, just have to check if "failed_opt" != None
    pub failed_opt: Option<OptFailed>,
}

/// represent the result for a transfer transaction
#[derive(Clone, Debug)]
pub struct TransferResult {
    pub wallet: Wallet,
    pub tx_receipt: Option<TransactionReceipt>,
    pub status: Status,
}

impl Wallet {
    /**
        start the transfer
        verify the wallet balance
        & send transfer transaction
    */
    pub async fn start_transfer(&mut self, to: Address, provider: HttpProvider) -> TransferResult {
        let mut res: TransferResult = TransferResult {
            wallet: self.clone(),
            tx_receipt: None,
            status: Status { failed_opt: None },
        };

        if !self
            .verify_wallet_balance_is_ok(provider.clone())
            .await
            .unwrap_or(false)
        {
            res.status.failed_opt = Some(OptFailed::NoEnoughEther);
            return res;
        }

        match self.perform_transfer(to, provider.clone()).await {
            Ok(tx) => res.tx_receipt = tx,
            Err(err) => res.status.failed_opt = Some(OptFailed::Reverted),
        };

        res
    }

    /**
        Calculate if the wallet balance is higher strict than gas fees
    */
    async fn verify_wallet_balance_is_ok(&mut self, provider: HttpProvider) -> Result<bool> {
        let gas_price = provider.get_gas_price().await?;
        let balance = provider.get_balance(self.address(), None).await?;
        let total_fees = gas_price * U256::from(21000);

        Ok(balance > total_fees)
    }

    /**
        send the transaction transfer to the address
    */
    async fn perform_transfer(
        &mut self,
        to: Address,
        provider: HttpProvider,
    ) -> Result<Option<TransactionReceipt>> {
        let client = SignerMiddleware::new(provider.clone(), self.local_wallet());
        let gas_price = provider.get_gas_price().await?;
        let balance = provider.get_balance(self.address(), None).await?;
        let total_fees = gas_price * U256::from(21000);

        let ether_to_send = balance - total_fees;
        let tx = TransactionRequest::new()
            .to(to)
            .value(ether_to_send)
            .from(self.address()) // specify the `from` field so that the client knows which account to use
        ;

        let tx: Option<TransactionReceipt> = client.send_transaction(tx, None).await?.await?;
        let balance = provider.get_balance(self.address(), None).await?;
        Ok(tx)
    }
}

/*
    TODO: tests

    - start_transfer
    - verify wallet
    - perform transfer

*/
