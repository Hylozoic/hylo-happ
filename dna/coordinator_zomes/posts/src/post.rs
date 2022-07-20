use hdk::prelude::*;
use posts_integrity::*;

#[hdk_extern]
// Can base (ActionHashB64) be an agent_pub_key?
// pub fn all_for_base(base: holo_hash::ActionHashB64) -> ExternResult<Vec<Record>> {
pub fn all(base: ActionHash) -> ExternResult<Vec<Record>> {
  let links = get_links(base, LinkTypes::PostedToGroup, None)?;

  return links
    .iter()
    .map(|link| {
      let record = hylo_utils::get_latest_update_for(ActionHash::from(link.target.clone()))?;
      Ok(record)
    })
    // turns the iterator into actual vector
    .collect::<ExternResult<Vec<Record>>>();
}

#[hdk_extern]
pub fn get(action_hash: ActionHash) -> ExternResult<Record> {
  return hylo_utils::get_latest_update_for(action_hash);
}

#[hdk_extern]
pub fn create(post: Post) -> ExternResult<Record> {
  let action_hash = create_entry(&EntryTypes::Post(post.clone()))?;

  return get(action_hash);
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct UpdatePostInput {
//   original_action_hash: ActionHash,
//   updated_post: Post
// }

// #[hdk_extern]
// pub fn update_post(input: UpdatePostInput) -> ExternResult<NewPostOutput> {
//   let action_hash = update_entry(input.original_action_hash, &input.updated_post)?;

//   let entry_hash = hash_entry(&input.updated_post)?;

//   let output = NewPostOutput {
//     action_hash,
//     entry_hash
//   };

//   Ok(output)
// }

// #[hdk_extern]
// pub fn delete_post(action_hash: ActionHash) -> ExternResult<ActionHash> {
//   delete_entry(action_hash)
// }

// let maybe_post: Option<Post> = record
//   .entry()
//   .to_app_option()
//   .map_err(|err| wasm_error!(err.into()))?;

// match record {
//   Some(valid_record) => {
//     // Serialize...
//     Ok(valid_record)
//   },
//   None => Err(wasm_error!(WasmErrorInner::Guest(String::from("Post not found."))))
// }
