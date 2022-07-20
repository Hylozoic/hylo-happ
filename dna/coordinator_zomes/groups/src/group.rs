use hdk::prelude::*;
use groups_integrity::*;
use hdk::hash_path::path::*;

const ROOT_PATH_STRING: &str = "all_groups";

fn path_string(string: String) -> String {
  if string.len() < 2 {
    return string
  }

  return format!(
    "{}.{}.{}",
    ROOT_PATH_STRING,
    string.chars().nth(0).unwrap(),
    string.chars().nth(1).unwrap()
  )
}

// starts at root if Group is not provided
fn typed_path_from_string(string: String) -> TypedPath {
  let typed_path = Path::from(path_string(string))
    .typed(LinkTypes::PathToGroup).unwrap();

  typed_path.ensure().unwrap();

  return typed_path
}

pub fn links_from_typed_path(typed_path: TypedPath) -> ExternResult<Vec<Link>> {
  let typed_path_entry_hash = typed_path.path_entry_hash()?;

  get_links(
    typed_path_entry_hash,
    LinkTypes::PathToGroup,
    None
  )
}

pub fn links_from_typed_paths(typed_paths: Vec<TypedPath>, all_links: &mut Vec<Link>) -> ExternResult<Vec<Link>> {
  for typed_path in typed_paths {
    let mut links = links_from_typed_path(typed_path.clone())?;

    all_links.append(&mut links);

    return links_from_typed_paths(typed_path.children_paths()?, all_links)
  };

  Ok(
    all_links.to_vec()
  )
}

#[hdk_extern]
pub fn all(_: Option<String>) -> ExternResult<Vec<Record>> {
  let links = links_from_typed_paths(
    typed_path_from_string(ROOT_PATH_STRING.into()).children_paths()?,
    &mut vec![]
  )?;

  return links
    .iter()
    .map(|link| {
      println!("{}", link.target.clone());
      hylo_utils::get_latest_update_for(ActionHash::from(link.target.clone()))
    })
    .collect()
}

#[hdk_extern]
pub fn get_by_slug(group_slug: String) -> ExternResult<Record> {
  let links = get_links(
    typed_path_from_string(group_slug.clone()).path_entry_hash()?,
    LinkTypes::PathToGroup,
    None
  )?;

  match links.iter().last() {
    Some(found_link) => {
      return hylo_utils::get_latest_update_for(ActionHash::from(found_link.target.clone()));
    },
    _ => Err(wasm_error!(WasmErrorInner::Guest(String::from("Something went wrong"))))
  }
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  let default_group: Group = Group {
    name: String::from("Holochain"),
    slug: String::from("holochain")
  };
  let group_action_hash = create_entry(&EntryTypes::Group(default_group.clone()))?;

  create_link(
    // Entry hash of the path
    typed_path_from_string(default_group.slug).path_entry_hash()?,
    group_action_hash, 
    LinkTypes::PathToGroup, 
    ()
  )?;

  Ok(InitCallbackResult::Pass)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_path_string() {
    assert_eq!(path_string(String::from("holochain")), String::from("all_groups.h.o"));
  }

  // #[test]
  // fn test_all() {
  //   let test = all(None).unwrap();

  //   assert_eq!(test, vec![]);
  // }

}

// Ok()
// // for path in month_paths {
// // let mut links = get_links(root_path.path_entry_hash()?, LinkTypes::PathToComment, None)?;
// // all_links.append(&mut links); // Collect the links
// // }

// let action_hashes: Vec<ActionHash> = all_links
//   .into_iter()
//   .map(|link| ActionHash::from(link.target))
//   .collect();

// let group_records = action_hashes
//   .iter()
//   .map(|action_hash| hylo_utils::get_latest_update_for(action_hash))
//   .collect();

// Ok()
// }

// #[hdk_extern]
// pub fn get(action_hash: ActionHash) -> ExternResult<Option<Group>> {
  // let maybe_element = hylo_utils::get_latest_update_for(action_hash)?;

  // match maybe_element {
  //   None => Ok(None),
  //   Some(record) => {
  //     let group: Group = record.entry()
  //       .to_app_option()
  //       .map_err(|error| wasm_error!(WasmErrorInner::Guest(format!("Could not deserialize Record to Group: {}", error))))?
  //       .ok_or(wasm_error!(WasmErrorInner::Guest("No Group found for the given hash.".into())))?;

  //     Ok(Some(group))
  //   }
  // }
// }

// #[derive(Serialize, Deserialize, Debug)]
// // #[serde(rename_all = "camelCase")]
// pub struct NewGroupOutput {
//   action_hash: ActionHash,
//   entry_hash: EntryHash,
// }

// #[hdk_extern]
// pub fn create_group(group: Group) -> ExternResult<NewGroupOutput> {
//   let action_hash = create_entry(&EntryTypes::Group(group.clone()))?;

//   let entry_hash = hash_entry(&EntryTypes::Group(group))?;

//   let output = NewGroupOutput {
//     action_hash,
//     entry_hash
//   };

//   Ok(output)
// }


// #[derive(Serialize, Deserialize, Debug)]
// // #[serde(rename_all = "camelCase")]
// pub struct UpdateGroupInput {
//   original_action_hash: ActionHash,
//   updated_group: Group
// }

// #[hdk_extern]
// pub fn update_group(input: UpdateGroupInput) -> ExternResult<NewGroupOutput> {
//   let action_hash = update_entry(input.original_action_hash, &input.updated_group)?;

//   let entry_hash = hash_entry(&input.updated_group)?;

//   let output = NewGroupOutput {
//     action_hash,
//     entry_hash
//   };

//   Ok(output)
// }


// #[hdk_extern]
// pub fn delete_group(action_hash: ActionHash) -> ExternResult<ActionHash> {
//   delete_entry(action_hash)
// }


// fn typed_group_path(group_slug: String) -> ExternResult<TypedPath> {
//   // "TypedPath" is a path with all the links of a certain type -- Add the link type to the path
//   Ok(
//     group_path(group_slug)?.typed(LinkTypes::PathToGroup)?
//   )
// }

// // Creates the path tree structure, and all the necessary links -- Build the path's anchor tree
// typed_group_path.ensure()?;

// pub fn gather_all_links_for_path (path: TypedPath) -> ExternResult<Vec<Link>> {

// }


// // Paths

// fn group_path(group_slug: String) -> ExternResult<Path> {
//   if group_slug.len() < 2 {
//     return Err(wasm_error!(WasmErrorInner::Guest(String::from("Group Slug needs to be at least 2 characters"))));
//   }

//   let unknown_error = format!("Unknown error deriving path from `group_slug`: \"{}\"", group_slug);

//   Ok(
//     Path::from(
//       format!(
//         "{}.{}.{}",
//         ROOT_GROUPS_PATH_STRING,
//         group_slug.chars().nth(0).ok_or(wasm_error!(WasmErrorInner::Guest(unknown_error.clone())))?,
//         // Note: returns 2nd character as 1st is consumed by iterator above
//         group_slug.chars().nth(0).ok_or(wasm_error!(WasmErrorInner::Guest(unknown_error.clone())))?
//       )
//     )
//   )
// }

// fn typed_group_path(group_slug: String) -> ExternResult<TypedPath> {
//   // "TypedPath" is a path with all the links of a certain type -- Add the link type to the path
//   Ok(
//     group_path(group_slug)?.typed(LinkTypes::PathToGroup)?
//   )
// }

