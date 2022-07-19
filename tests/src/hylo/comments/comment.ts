
import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import pkg from 'tape-promise/tape';
const { test } = pkg;

import { hyloDna } from  "../../utils";


export default () => test("comment CRUD tests", async (t) => {
  await runScenario(async scenario => {

    const dnas: DnaSource[] = [{path: hyloDna }];

    const [alice, bob]  = await scenario.addPlayersWithHapps([dnas, dnas]);

    await scenario.shareAllAgents();

    const createInput = {
  "text": "They can trust me and go. They can trust me and go. Eventually, you do plan to have dinosaurs on your dinosaur tour, right?",
  "base": Buffer.from(new Uint8Array([132,33,36,189,2,13,138,118,148,128,46,9,233,223,87,58,29,18,26,64,21,192,153,241,34,18,158,118,33,187,232,140,169,169,133,136,135,20,18]))
};

    // Alice creates a comment
    const createOutput: any = await alice.cells[0].callZome({
      zome_name: "comments",
      fn_name: "create_comment",
      payload: createInput,
    });
    t.ok(createOutput.actionHash);  // test 1
    t.ok(createOutput.entryHash);   // test 2

    // Wait for the created entry to be propagated to the other node.
    await pause(100);

    
    // Bob gets the created comment
    const readOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "comments",
      fn_name: "get_comment",
      payload: createOutput.entryHash,
    });
    t.deepEqual(readOutput, createInput); // test 3
    
    
    // Alice updates the comment
    const contentUpdate = {
  "text": "God creates dinosaurs. Goodbye! God help us, we're in the hands of engineers.",
  "base": Buffer.from(new Uint8Array([132,33,36,167,137,251,226,30,59,113,10,11,68,139,128,24,83,136,199,108,127,120,134,147,184,89,248,39,198,172,239,51,186,129,210,108,189,155,168]))
}

    const updateInput = {
      originalActionHash: createOutput.actionHash,
      updatedComment: contentUpdate,
    }

    const updateOutput: any = await alice.cells[0].callZome({
      zome_name: "comments",
      fn_name: "update_comment",
      payload: updateInput,
    });
    t.ok(updateOutput.actionHash);  // test 4
    t.ok(updateOutput.entryHash);   // test 5

    // Wait for the updated entry to be propagated to the other node.
    await pause(100);

      
    // Bob gets the updated comment
    const readUpdatedOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "comments",
      fn_name: "get_comment",
      payload: updateOutput.entryHash,
    });
    t.deepEqual(readUpdatedOutput, contentUpdate);  // test 6

    
    
    // Alice deletes the comment
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "comments",
      fn_name: "delete_comment",
      payload: createOutput.actionHash,
    })
    t.ok(deleteActionHash); // test 7

      
    // Wait for the deletion action to be propagated to the other node.
    await pause(100);

    // Bob tries to get the deleted comment, but he doesn't get it because it has been deleted
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "comments",
      fn_name: "get_comment",
      payload: createOutput.entryHash,
    });
    t.notOk(readDeletedOutput); // test 8

    
  });



});
