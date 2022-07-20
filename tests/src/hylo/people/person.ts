
import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import { hyloDna } from  "../../utils";
import pkg from 'tape-promise/tape';
import { decode } from "@msgpack/msgpack";

const { test } = pkg;

export default () => test("person CRUD tests", async (t) => {
  await runScenario(async scenario => {
    const dnas: DnaSource[] = [{path: hyloDna }];
    const [bob]  = await scenario.addPlayersWithHapps([dnas]);

    await scenario.shareAllAgents();

    const createInput = {
      "name": "Bob Tester",
      "avatar_url": "https://tiny.url/lkjasdfklj.jpg"
    };

    // Bob registers himself
    const createOutput: any = await bob.cells[0].callZome({
      zome_name: "people",
      fn_name: "create",
      payload: createInput,
    });
    t.ok(createOutput.entry.Present.entry);  // test 1
    t.ok(createOutput.entry.Present.entry_type);   // test 2

    // Wait for the created entry to be propagated to the other node.
    // await pause(100);

    // Bob gets his record
    const readOutput: any = await bob.cells[0].callZome({
      zome_name: "people",
      fn_name: "get",
      payload: null
    });
    t.deepEqual(decode(readOutput.entry.Present.entry), { ...createInput, agent_pub_key: bob.agentPubKey }); // test 3
  });
});

// // Alice updates the person
// const contentUpdate = {
//   "name": "tour Do dependent",
//   "avatarUrl": "the know travel"
// }

// const updateInput = {
//   originalActionHash: createOutput.actionHash,
//   updatedPerson: contentUpdate,
// }

// const updateOutput: any = await alice.cells[0].callZome({
//   zome_name: "people",
//   fn_name: "update_person",
//   payload: updateInput,
// });
// t.ok(updateOutput.actionHash);  // test 4
// t.ok(updateOutput.entryHash);   // test 5

// // Wait for the updated entry to be propagated to the other node.
// await pause(100);

// // Bob gets current agent person
// const readUpdatedOutput: typeof createInput = await bob.cells[0].callZome({
//   zome_name: "people",
//   fn_name: "get"
// });

// t.deepEqual(readUpdatedOutput, contentUpdate);  // test 6

// // Alice deletes the person
// const deleteActionHash = await alice.cells[0].callZome({
//   zome_name: "people",
//   fn_name: "delete_person",
//   payload: createOutput.actionHash,
// })
// t.ok(deleteActionHash); // test 7

  
// // Wait for the deletion action to be propagated to the other node.
// await pause(100);

// // Bob tries to get the deleted person, but he doesn't get it because it has been deleted
// const readDeletedOutput = await bob.cells[0].callZome({
//   zome_name: "people",
//   fn_name: "get_person",
//   payload: createOutput.entryHash,
// });

// t.notOk(readDeletedOutput); // test 8    
