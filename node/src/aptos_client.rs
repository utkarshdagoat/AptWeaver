use anyhow::{Context, Ok, Result};
use aptos_sdk::{
    move_types::{identifier::Identifier, language_storage::ModuleId},
    rest_client::{Client, PendingTransaction},
    transaction_builder::TransactionBuilder,
    types::{
        account_address::AccountAddress,
        chain_id::ChainId,
        transaction::{EntryFunction, TransactionPayload},
        LocalAccount,
    },
};
use once_cell::sync::Lazy;
use sha3::digest::generic_array::sequence;
use std::{str::FromStr, time::SystemTime};
use url::Url;

pub static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.testnet.aptoslabs.com"),
    )
    .unwrap()
});

pub struct MpcContractClient<'a> {
    api_client: &'a Client,
}

impl<'a> MpcContractClient<'a> {
    pub fn new(api_client: &'a Client) -> Self {
        MpcContractClient { api_client }
    }

    pub async fn register_node(
        &self,
        from_account: &mut LocalAccount,
        options: Option<GenericOptions>,
    ) -> Result<PendingTransaction> {
        let options = options.unwrap_or_default();

        let chain_id = self
            .api_client
            .get_index()
            .await
            .context("Failed to get chain id")?
            .inner()
            .chain_id;
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    AccountAddress::from_hex_literal(
                        "0x4d50f43a3b7790d1cc4bfb00adfb16d7a46ac55ee3de30af757910215a533e88",
                    )
                    .unwrap(),
                    Identifier::new("mpc_contract").unwrap(),
                ),
                Identifier::new("request_join").unwrap(),
                vec![],
                vec![],
            )),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + options.timeout_secs,
            ChainId::new(chain_id),
        )
        .sender(from_account.address())
        .sequence_number(from_account.sequence_number())
        .max_gas_amount(options.max_gas_amount)
        .gas_unit_price(options.gas_unit_price);
        let signed_tx = from_account.sign_with_transaction_builder(transaction_builder);

        Ok(self
            .api_client
            .submit(&signed_tx)
            .await
            .context("Failed to submit transaction")?
            .into_inner())
    }


}

pub struct GenericOptions {
    pub max_gas_amount: u64,
    pub gas_unit_price: u64,
    pub timeout_secs: u64,
}

impl Default for GenericOptions {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 100,
            timeout_secs: 100,
        }
    }
}
