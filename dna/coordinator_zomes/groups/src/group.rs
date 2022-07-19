use hdk::prelude::*;
use groups_integrity::Group;
use groups_integrity::EntryTypes;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  let default_group: Group = Group {
    name: String::from("Holochain"),
    slug: String::from("holochain")
  };

  create_entry(&EntryTypes::Group(default_group))?;
  
  Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_group(action_hash: ActionHash) -> ExternResult<Option<Group>> {
  let maybe_element = get(action_hash, GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(record) => {
      let group: Group = record.entry()
        .to_app_option()
        .map_err(|error| wasm_error!(WasmErrorInner::Guest(format!("Could not deserialize Record to Group: {}", error))))?
        .ok_or(wasm_error!(WasmErrorInner::Guest("No Group found for the given hash.".into())))?;

      Ok(Some(group))
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct NewGroupOutput {
  action_hash: ActionHash,
  entry_hash: EntryHash,
}

#[hdk_extern]
pub fn create_group(group: Group) -> ExternResult<NewGroupOutput> {
  let action_hash = create_entry(&EntryTypes::Group(group.clone()))?;

  let entry_hash = hash_entry(&EntryTypes::Group(group))?;

  let output = NewGroupOutput {
    action_hash,
    entry_hash
  };

  Ok(output)
}


#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
pub struct UpdateGroupInput {
  original_action_hash: ActionHash,
  updated_group: Group
}

#[hdk_extern]
pub fn update_group(input: UpdateGroupInput) -> ExternResult<NewGroupOutput> {
  let action_hash = update_entry(input.original_action_hash, &input.updated_group)?;

  let entry_hash = hash_entry(&input.updated_group)?;

  let output = NewGroupOutput {
    action_hash,
    entry_hash
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_group(action_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(action_hash)
}

