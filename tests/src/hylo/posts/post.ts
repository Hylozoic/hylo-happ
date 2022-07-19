
import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import pkg from 'tape-promise/tape';
import { hyloDna } from  "../../utils";

const { test } = pkg;

export default () => test("post CRUD tests", async (t) => {
  await runScenario(async scenario => {

    const dnas: DnaSource[] = [{path: hyloDna }];

    const [alice, bob]  = await scenario.addPlayersWithHapps([dnas, dnas]);

    await scenario.shareAllAgents();

    const createInput = {
  "title": "for something movie",
  "details": "AM/FM radio, reclining bucket seats, and power windows. It's a delight to trust somebody so completely. This thing comes fully loaded.",
  "post_type": "You're the planet",
  "announcement": false
};

    // Alice creates a post
    const createOutput: any = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "create_post",
      payload: createInput,
    });
    t.ok(createOutput.actionHash);  // test 1
    t.ok(createOutput.entryHash);   // test 2

    // Wait for the created entry to be propagated to the other node.
    await pause(100);

    
    // Bob gets the created post
    const readOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: createOutput.entryHash,
    });
    t.deepEqual(readOutput, createInput); // test 3
    
    
    // Alice updates the post
    const contentUpdate = {
  "title": "right I ticking",
  "details": "If any movie people are watching this show, please, for me, have some respect. Yeah, but your scientists were so preoccupied with whether or not they could, they didn't stop to think if they should. And the clock is ticking.",
  "post_type": "you love no",
  "announcement": false
}

    const updateInput = {
      originalActionHash: createOutput.actionHash,
      updatedPost: contentUpdate,
    }

    const updateOutput: any = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "update_post",
      payload: updateInput,
    });
    t.ok(updateOutput.actionHash);  // test 4
    t.ok(updateOutput.entryHash);   // test 5

    // Wait for the updated entry to be propagated to the other node.
    await pause(100);

      
    // Bob gets the updated post
    const readUpdatedOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: updateOutput.entryHash,
    });
    t.deepEqual(readUpdatedOutput, contentUpdate);  // test 6

    
    
    // Alice deletes the post
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "posts",
      fn_name: "delete_post",
      payload: createOutput.actionHash,
    })
    t.ok(deleteActionHash); // test 7

      
    // Wait for the deletion action to be propagated to the other node.
    await pause(100);

    // Bob tries to get the deleted post, but he doesn't get it because it has been deleted
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "posts",
      fn_name: "get_post",
      payload: createOutput.entryHash,
    });
    t.notOk(readDeletedOutput); // test 8

    
  });



});
