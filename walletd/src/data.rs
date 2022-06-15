use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_index: u32,
    pub balance: u64,
    pub base_address: String,
    pub label: String,
    pub tag: String,
    pub unlocked_balance: u64,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct SubAddress {
    pub major: u32,
    pub minor: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub address: String,
    pub amount: u64,
    pub confirmations: u32,
    pub height: u32,
    pub fee: u64,
    pub note: String,
    pub payment_id: String,
    pub subaddr_index: SubAddress,
    pub suggested_confirmations_threshold: u32,
    pub timestamp: u64,
    pub txid: String,
    pub r#type: String,
    pub unlock_time: u32,
}

