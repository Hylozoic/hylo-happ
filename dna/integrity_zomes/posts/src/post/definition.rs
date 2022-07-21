use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Post {
  pub title: String,
  pub details: String,
  pub post_type: String,
  pub announcement: bool,
  pub author_pub_key: AgentPubKey
}
