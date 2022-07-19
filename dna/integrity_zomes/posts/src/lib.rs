mod post;
pub use post::Post;
use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  #[entry_def()]
  Post(Post)
}

#[hdk_link_types]
pub enum LinkTypes {
  PostedToGroup,
  PostedToPost,
  PostedToMessageThread
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
