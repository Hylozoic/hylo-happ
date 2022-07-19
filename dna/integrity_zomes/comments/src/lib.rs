mod comment;
pub use comment::Comment;
use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
#[entry_def()]
  Comment(Comment),
}

#[hdk_link_types]
pub enum LinkTypes {
  CommentedOn
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
