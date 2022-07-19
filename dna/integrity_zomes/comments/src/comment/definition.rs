use holochain_deterministic_integrity::prelude::*;





#[hdk_entry_helper]
// #[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Comment {
  pub text: String,
  pub base: EntryHash,
}