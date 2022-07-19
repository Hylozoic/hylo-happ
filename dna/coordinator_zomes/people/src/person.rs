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
pub fn get(maybe_agent_pub_key: Option<AgentPubKey>) -> ExternResult<AgentPubKey> {
  let agent_pub_key: AgentPubKey = match maybe_agent_pub_key {
    Some(agent_pub_key) => agent_pub_key,
    _ => agent_info()?.agent_latest_pubkey
  };

  return Ok(agent_pub_key)
  // let person_links: Vec<Link> = get_links(agent_pub_key, LinkTypes::AgentToPerson, None)?;
  // let maybe_person_link = person_links.iter().last();

  // match maybe_person_link {
  //   Some(person_link) => {
  //     let person_record = hylo_utils::get_latest_update_for(person_link.clone().target.into())?;

  //     Ok(person_record)
  //   },
  //   _ => {
  //     Err(wasm_error!(WasmErrorInner::Guest(String::from("Something went wrong"))))
  //   }
  // }
}
