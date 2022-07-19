use holochain_deterministic_integrity::prelude::*;



#[hdk_entry_helper]
// #[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Group {
  pub name: String,
  pub slug: String,
}