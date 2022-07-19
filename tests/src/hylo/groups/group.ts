
import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import pkg from 'tape-promise/tape';
const { test } = pkg;

import { hyloDna } from  "../../utils";


export default () => test("group CRUD tests", async (t) => {
  await runScenario(async scenario => {

    const dnas: DnaSource[] = [{path: hyloDna }];

    const [alice, bob]  = await scenario.addPlayersWithHapps([dnas, dnas]);

    await scenario.shareAllAgents();

    const createInput = {
  "name": "toxic cry Must",
  "slug": "scientists what you"
};

    // Alice creates a group
    const createOutput: any = await alice.cells[0].callZome({
      zome_name: "groups",
      fn_name: "create_group",
      payload: createInput,
    });
    t.ok(createOutput.actionHash);  // test 1
    t.ok(createOutput.entryHash);   // test 2

    // Wait for the created entry to be propagated to the other node.
    await pause(100);

    
    // Bob gets the created group
    const readOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "groups",
      fn_name: "get_group",
      payload: createOutput.entryHash,
    });
    t.deepEqual(readOutput, createInput); // test 3
    
    
    // Alice updates the group
    const contentUpdate = {
  "name": "Hey what y'know",
  "slug": "It maybe go"
}

    const updateInput = {
      originalActionHash: createOutput.actionHash,
      updatedGroup: contentUpdate,
    }

    const updateOutput: any = await alice.cells[0].callZome({
      zome_name: "groups",
      fn_name: "update_group",
      payload: updateInput,
    });
    t.ok(updateOutput.actionHash);  // test 4
    t.ok(updateOutput.entryHash);   // test 5

    // Wait for the updated entry to be propagated to the other node.
    await pause(100);

      
    // Bob gets the updated group
    const readUpdatedOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "groups",
      fn_name: "get_group",
      payload: updateOutput.entryHash,
    });
    t.deepEqual(readUpdatedOutput, contentUpdate);  // test 6

    
    
    // Alice deletes the group
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "groups",
      fn_name: "delete_group",
      payload: createOutput.actionHash,
    })
    t.ok(deleteActionHash); // test 7

      
    // Wait for the deletion action to be propagated to the other node.
    await pause(100);

    // Bob tries to get the deleted group, but he doesn't get it because it has been deleted
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "groups",
      fn_name: "get_group",
      payload: createOutput.entryHash,
    });
    t.notOk(readDeletedOutput); // test 8

    
  });



});
