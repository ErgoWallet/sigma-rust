//! Wallet-like features
use derive_more::{From, Into};
use ergo_lib::ergotree_ir::sigma_protocol::sigma_boolean::SigmaBoolean;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

pub mod derivation_path;
pub mod ext_pub_key;
pub mod ext_secret_key;
pub mod mnemonic;

use crate::address::Address;
use crate::input::Input;
use crate::transaction::TransactionHintsBag;
use crate::{
    box_coll::ErgoBoxes,
    ergo_state_ctx::ErgoStateContext,
    error_conversion::to_js,
    secret_key::{SecretKey, SecretKeys},
    transaction::reduced::ReducedTransaction,
    transaction::Transaction,
    transaction::UnsignedTransaction,
};

/// A collection of secret keys. This simplified signing by matching the secret keys to the correct inputs automatically.
#[wasm_bindgen]
#[derive(From, Into)]
pub struct Wallet(ergo_lib::wallet::Wallet);

#[wasm_bindgen]
impl Wallet {
    /// Create wallet instance loading secret key from mnemonic
    /// Returns None if a DlogSecretKey cannot be parsed from the provided phrase
    #[wasm_bindgen]
    pub fn from_mnemonic(mnemonic_phrase: &str, mnemonic_pass: &str) -> Result<Wallet, JsValue> {
        crate::utils::set_panic_hook();
        ergo_lib::wallet::Wallet::from_mnemonic(mnemonic_phrase, mnemonic_pass)
            .map(Wallet)
            .map_err(to_js)
    }

    /// Create wallet using provided secret key
    #[wasm_bindgen]
    pub fn from_secrets(secret: &SecretKeys) -> Wallet {
        crate::utils::set_panic_hook();
        Wallet(ergo_lib::wallet::Wallet::from_secrets(secret.into()))
    }

    /// Add a secret to the wallets prover
    #[wasm_bindgen]
    pub fn add_secret(&mut self, secret: &SecretKey) {
        self.0.add_secret(secret.clone().into());
    }

    /// Sign a transaction:
    /// `tx` - transaction to sign
    /// `boxes_to_spend` - boxes corresponding to [`UnsignedTransaction::inputs`]
    /// `data_boxes` - boxes corresponding to [`UnsignedTransaction::data_inputs`]
    #[wasm_bindgen]
    pub fn sign_transaction(
        &self,
        _state_context: &ErgoStateContext,
        tx: &UnsignedTransaction,
        boxes_to_spend: &ErgoBoxes,
        data_boxes: &ErgoBoxes,
    ) -> Result<Transaction, JsValue> {
        let boxes_to_spend = boxes_to_spend.clone().into();
        let data_boxes = data_boxes.clone().into();
        let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
            tx.0.clone(),
            boxes_to_spend,
            data_boxes,
        )
        .map_err(to_js)?;
        self.0
            .sign_transaction(tx_context, &_state_context.clone().into(), None)
            .map_err(to_js)
            .map(Transaction::from)
    }

    /// Sign a multi signature transaction:
    /// `tx` - transaction to sign
    /// `boxes_to_spend` - boxes corresponding to [`UnsignedTransaction::inputs`]
    /// `data_boxes` - boxes corresponding to [`UnsignedTransaction::data_inputs`]
    /// `tx_hints` - transaction hints bag corresponding to [`TransactionHintsBag`]
    #[wasm_bindgen]
    pub fn sign_transaction_multi(
        &self,
        _state_context: &ErgoStateContext,
        tx: &UnsignedTransaction,
        boxes_to_spend: &ErgoBoxes,
        data_boxes: &ErgoBoxes,
        tx_hints: &TransactionHintsBag,
    ) -> Result<Transaction, JsValue> {
        let boxes_to_spend = boxes_to_spend.clone().into();
        let data_boxes = data_boxes.clone().into();
        let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
            tx.0.clone(),
            boxes_to_spend,
            data_boxes,
        )
        .map_err(to_js)?;
        self.0
            .sign_transaction(
                tx_context,
                &_state_context.clone().into(),
                Some(&tx_hints.0),
            )
            .map_err(to_js)
            .map(Transaction::from)
    }

    /// Sign a transaction:
    /// `reduced_tx` - reduced transaction, i.e. unsigned transaction where for each unsigned input
    /// added a script reduction result.
    #[wasm_bindgen]
    pub fn sign_reduced_transaction(
        &self,
        reduced_tx: &ReducedTransaction,
    ) -> Result<Transaction, JsValue> {
        self.0
            .sign_reduced_transaction(reduced_tx.clone().into(), None)
            .map_err(to_js)
            .map(Transaction::from)
    }

    /// Sign a multi signature reduced transaction:
    /// `reduced_tx` - reduced transaction, i.e. unsigned transaction where for each unsigned input
    /// added a script reduction result.
    /// `tx_hints` - transaction hints bag corresponding to [`TransactionHintsBag`]
    #[wasm_bindgen]
    pub fn sign_reduced_transaction_multi(
        &self,
        reduced_tx: &ReducedTransaction,
        tx_hints: &TransactionHintsBag,
    ) -> Result<Transaction, JsValue> {
        self.0
            .sign_reduced_transaction(reduced_tx.clone().into(), Some(&tx_hints.0))
            .map_err(to_js)
            .map(Transaction::from)
    }

    /// Generate Commitments for unsigned tx
    #[wasm_bindgen]
    pub fn generate_commitments(
        &self,
        _state_context: &ErgoStateContext,
        tx: &UnsignedTransaction,
        boxes_to_spend: &ErgoBoxes,
        data_boxes: &ErgoBoxes,
    ) -> Result<TransactionHintsBag, JsValue> {
        let boxes_to_spend = boxes_to_spend.clone().into();
        let data_boxes = data_boxes.clone().into();
        let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
            tx.0.clone(),
            boxes_to_spend,
            data_boxes,
        )
        .map_err(to_js)?;
        self.0
            .generate_commitments(tx_context, &_state_context.clone().into())
            .map_err(to_js)
            .map(TransactionHintsBag::from)
    }

    /// Generate Commitments for reduced Transaction
    #[wasm_bindgen]
    pub fn generate_commitments_for_reduced_transaction(
        &self,
        reduced_tx: &ReducedTransaction,
    ) -> Result<TransactionHintsBag, JsValue> {
        self.0
            .generate_commitments_for_reduced_transaction(reduced_tx.clone().into())
            .map_err(to_js)
            .map(TransactionHintsBag::from)
    }

    /// Sign an arbitrary message using a P2PK address
    #[wasm_bindgen]
    pub fn sign_message_using_p2pk(
        &self,
        address: &Address,
        message: &[u8],
    ) -> Result<Uint8Array, JsValue> {
        if let Address(ergo_lib::ergotree_ir::chain::address::Address::P2Pk(d)) = address.clone() {
            let sb = SigmaBoolean::from(d);
            self.0
                .sign_message(sb, message)
                .map_err(to_js)
                .map(|v| Uint8Array::from(v.as_slice()))
        } else {
            Err(JsValue::from_str(
                "wallet::sign_message_using_p2pk: Address:P2Pk expected",
            ))
        }
    }

    /// Sign a given tx input
    #[wasm_bindgen]
    pub fn sign_tx_input(
        &self,
        input_idx: usize,
        state_context: &ErgoStateContext,
        tx: &UnsignedTransaction,
        boxes_to_spend: &ErgoBoxes,
        data_boxes: &ErgoBoxes,
    ) -> Result<Input, JsValue> {
        let boxes_to_spend = boxes_to_spend.clone().into();
        let data_boxes = data_boxes.clone().into();
        let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
            tx.0.clone(),
            boxes_to_spend,
            data_boxes,
        )
        .map_err(to_js)?;
        let state_context_inner = state_context.clone().into();
        self.0
            .sign_tx_input(input_idx, tx_context, &state_context_inner, None)
            .map_err(to_js)
            .map(Input::from)
    }
}
