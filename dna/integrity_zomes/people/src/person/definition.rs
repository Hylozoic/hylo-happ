use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_helper]
// #[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Person {
  pub agent_pub_key: AgentPubKey,
  pub name: String,
  pub avatar_url: String,
}