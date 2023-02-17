use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;

use ego_types::app::{AppId, Category, Version};


#[derive(CandidType, Deserialize)]
pub struct AdminAppCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub version: Version,
  pub backend_data: Vec<u8>,
  pub backend_data_hash: String,
}


#[derive(CandidType, Deserialize)]
pub struct AdminWalletProviderAddRequest {
  pub wallet_provider: Principal,
  pub wallet_app_id: AppId,
}