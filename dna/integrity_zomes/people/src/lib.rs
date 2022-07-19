mod person;
pub use person::Person;
use holochain_deterministic_integrity::prelude::*;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
#[entry_def()]
  Person(Person)
}

#[hdk_link_types]
pub enum LinkTypes {
  AgentToPerson
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
