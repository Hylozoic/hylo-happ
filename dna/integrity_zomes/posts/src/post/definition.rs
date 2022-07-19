use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_helper]
// #[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Post {
  pub title: String,
  pub details: String,
  pub post_type: String,
  pub announcement: bool
}