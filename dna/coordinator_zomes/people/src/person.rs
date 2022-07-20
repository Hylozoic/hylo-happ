use hdk::prelude::*;
use people_integrity::Person;
use people_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePersonInput {
  name: String,
  avatar_url: String
}

#[hdk_extern]
pub fn create(person: CreatePersonInput) -> ExternResult<Record> {
  let agent_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey.into();
  let action_hash = create_entry(
    &EntryTypes::Person(
      Person {
        name: person.name,
        avatar_url: person.avatar_url,
        agent_pub_key: agent_pub_key.clone()
      }
    )
  )?;

  create_link(
    agent_pub_key,
    action_hash.clone(),
    LinkTypes::AgentToPerson,
    ()
  )?;

  match get_details(action_hash.clone(), GetOptions::default())? {
    Some(Details::Record(details)) => {
      Ok(details.record)
    },
    _ => {
      Err(wasm_error!(WasmErrorInner::Guest(String::from("Something went wrong"))))
    }
  }
}

#[hdk_extern]
// TODO: Probably change to this for consistency
// pub fn get(maybe_peson_action_hash: Option<AgentPubKey>) -> ExternResult<Record> {
pub fn get(maybe_agent_pub_key: Option<AgentPubKey>) -> ExternResult<Record> {
  let agent_pub_key: AgentPubKey = match maybe_agent_pub_key {
    Some(agent_pub_key) => agent_pub_key,
    _ => agent_info()?.agent_latest_pubkey
  };
  let person_links: Vec<Link> = get_links(agent_pub_key, LinkTypes::AgentToPerson, None)?;
  let maybe_person_link = person_links.iter().last();

  match maybe_person_link {
    Some(person_link) => {
      let person_record = hylo_utils::get_latest_update_for(person_link.clone().target.into())?;

      Ok(person_record)
    },
    _ => {
      Err(wasm_error!(WasmErrorInner::Guest(String::from("No Person found for Agent"))))
    }
  }
}

// #[hdk_extern]
// pub fn all(maybe_group_action_hash: Option<ActionHash>) -> ExternResult<Vec<Record>> {
//   match maybe_group_action_hash {
//     Some(group_action_hash) => {
//       let group_links: Vec<Link> = get_links(group_action_hash, LinkTypes::GroupToPerson, None)?;
//       let group_records = group_links.iter().collect(<Vec<Record>>);

//       Ok(group_records)
//     }
//     _ => {
//       let group_links: Vec<Link> = get_links(ROOT_PATH_OR_ANCHOR, LinkTypes::GroupToPerson, None)?;
//       let group_records = group_links.iter().map(|group_link| hylo_utils::get_latest_update_for(group_link.clone().target.into())).collect();

//       Ok(group_records)
//     }
//   };
// }