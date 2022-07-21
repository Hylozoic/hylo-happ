// use hdk::prelude::*;
// use comments_integrity::Comment;

// #[hdk_extern]
// pub fn get(entry_hash: EntryHash) -> ExternResult<Option<Comment>> {
//   // let maybe_element = get(entry_hash, GetOptions::default())?;

//   match maybe_element {
//     None => Ok(None),
//     Some(record) => {
//       let comment: Comment = record.entry()
//         .to_app_option()
//         .map_err(|error| wasm_error!(WasmErrorInner::Guest(format!("Could not deserialize Record to Comment: {}", error))))?
//         .ok_or(wasm_error!(WasmErrorInner::Guest("No Comment found for the given hash.".into())))?;

//       Ok(Some(comment))
//     }
//   }
// }
