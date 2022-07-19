use hdk::prelude::*;

// ** ActionHash
// from base64: `ActionHash::from(base)` or `&base.into()`, if in context
// to base64:
// `let action_hash: ActionHash = ActionHashB64::from(action_hash_in_base_64)`

// Put in a hylo utils library for use in multiple zomes
// Ignores multiple action updates, returning only the last update for each action
#[hdk_extern]
pub fn get_latest_update_for(action_hash: ActionHash) -> ExternResult<Record> {
  let details = get_details(action_hash, GetOptions::default())?;

  match details {
    Some(Details::Record(details)) => {
      match details.updates.last() {
        Some(last_action) => {
          return get_latest_update_for(last_action.action_address().clone())
        }
        None => {
          return Ok(details.record)
        }
      }
    },
    _ => Err(wasm_error!(WasmErrorInner::Guest(String::from("Something went wrong"))))
  }
}
