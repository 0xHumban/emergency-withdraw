use ratatui::widgets::TableState;
use unicode_width::UnicodeWidthStr;

use crate::{app_data::AppData, constants::*, wallet::Wallet};

/*
    NOTE:
        May be using a vec of &Wallet instead Wallet is better
*/

/// used to store app data and state of the app
#[derive(Clone)]
pub struct Model {
    pub app_data: AppData,
    pub table_state: TableState,
    pub running: bool, // used to stop the app

    // used to set constraints of the Table (number, public_address, eth_balance)
    pub longuest_item_lens: (u16, u16, u16),
    // used to store the Wallets selected by the user
    pub wallets_selected: Vec<Wallet>,
}

impl Model {
    pub async fn new() -> Self {
        let app_data: AppData = AppData::load_appdata(
            PROVIDER_VAR_NAME,
            PHRASE_VAR_NAME,
            PASSWORD_VAR_NAME,
            TO_ADDRESS_VAR_NAME,
            WALLETS_NUMBER_VAR_NAME,
        )
        .await
        .unwrap();

        Model {
            running: true,
            app_data: app_data.clone(),
            table_state: TableState::default().with_selected(0),
            longuest_item_lens: Model::constraint_len_calculator(&app_data),
            wallets_selected: Vec::new(),
        }
    }

    /// returns the longuest wallet data possible
    /// used to set Constraints of Table
    fn constraint_len_calculator(items: &AppData) -> (u16, u16, u16) {
        let wallets: Vec<Wallet> = items.wallets();
        let number_len = UnicodeWidthStr::width(wallets.len().to_string().as_str());
        let address_len = wallets
            .iter()
            .map(|w| w.address_to_string())
            .map(|address| UnicodeWidthStr::width(address.as_str()))
            .max()
            .unwrap_or(0);

        let eth_balance_len = wallets
            .iter()
            .map(|w| w.eth_balance_to_string())
            .map(|bal| UnicodeWidthStr::width(bal.as_str()))
            .max()
            .unwrap_or(0);

        #[allow(clippy::cast_possible_truncation)]
        (
            number_len as u16,
            address_len as u16,
            eth_balance_len as u16,
        )
    }

    /// passed to the next row to focus on table_state
    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.app_data.wallets().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    /// passed to the previous row to focus on table_state
    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.app_data.wallets().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    /// toggle a wallet, if not selected, select it,
    /// otherwise unselect it: remove the wallet from the list
    pub fn toggle_wallet(&mut self) {
        if let Some(index) = self.table_state.selected() {
            if let Some(wallet) = self.app_data.wallets().get(index) {
                wallet.toggle_wallet_in_list(&mut self.wallets_selected);
            }
        }
    }

    /// toggle all wallets in the list
    pub fn toggle_all_wallets(&mut self) {
        // if all users already selected, clear the selected list
        if self.wallets_selected.len() == self.app_data.wallets().len() {
            self.wallets_selected = Vec::new();
        } else {
            self.wallets_selected = self.app_data.wallets();
        }
    }

    /// perform transfer foreach wallets selected
    pub async fn start_transfer_wallet_selected(&mut self) {
        for wallet in self.wallets_selected.iter_mut() {
            let res = wallet
                .start_transfer(self.app_data.to_address(), self.app_data.provider())
                .await;

            println!(
                "result: address: {:?}, status: {:?}",
                res.wallet.address(),
                res.status
            );
        }
    }
}

/*
TODO tests:
    - next
    - previous
    - toggle_wallet
    - toggle_all_wallets
    - start_transfer_walets_selected
*/
