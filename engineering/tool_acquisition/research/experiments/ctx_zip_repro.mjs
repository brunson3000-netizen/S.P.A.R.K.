import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFile, mkdtemp, rm} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join, resolve} from "node:path";
import {pathToFileURL} from "node:url";

const [strategyPath, adapterPath] = process.argv.slice(2);
if (!strategyPath || !adapterPath) throw new Error("Pass pinned strategies/index.ts and file-adapter.ts paths.");
const sourceRecords = [];
for (const p of [strategyPath, adapterPath]) {
  const bytes = await readFile(p);
  sourceRecords.push({
    file: p,
    sha256: createHash("sha256").update(bytes).digest("hex"),
    git_blob_sha1: createHash("sha1").update(Buffer.from("blob " + bytes.length + "\0")).update(bytes).digest("hex")
  });
}
assert.deepEqual(sourceRecords.map(r=>r.git_blob_sha1), [
  "946ede569a747961c430575d72d9169b152366b0",
  "ae56ebbc77b75f472dfac53c6ce645d0925459f6"
], "Source bytes must match the studied pin before importing.");
const {writeToolResultsToFileStrategy} = await import(pathToFileURL(resolve(strategyPath)).href);
const {LocalFileAdapter} = await import(pathToFileURL(resolve(adapterPath)).href);
const dir = await mkdtemp(join(tmpdir(), "ctx-zip-research-"));
try {
  const adapter = new LocalFileAdapter({baseDir:dir, sessionId:"repro"});
  const mk = (id, value) => ({role:"tool",content:[{type:"tool-result",toolName:"lookup",toolCallId:id,output:{type:"json",value}}]});
  const messages = [mk("one",{value:"FIRST"}),mk("two",{value:"SECOND"}),{role:"assistant",content:"done"}];
  const before = JSON.stringify(messages);
  let serializerCalls = 0;
  const output = await writeToolResultsToFileStrategy(messages,{
    boundary:"all",adapter,sessionId:"repro",fileReaderTools:[],
    toolResultSerializer:()=>{serializerCalls++;return "CUSTOM";}
  });
  const persisted=JSON.parse(await readFile(join(dir,adapter.resolveKey("lookup.json")),"utf8"));
  const refs=output.slice(0,2).map(m=>m.content[0].output.value);
  assert.equal(refs[0],refs[1]);
  assert.equal(persisted.metadata.toolCallId,"two");
  assert.equal(persisted.output.value,"SECOND");
  assert.notEqual(JSON.stringify(messages),before);
  assert.equal(serializerCalls,0);
  console.log(JSON.stringify({
    node:process.version,upstream_pin:"76580f7ba1555c891928702743ac74412c7fac60",
    sourceRecords,checks:{
      repeated_tool_calls_share_reference:true,
      first_payload_overwritten_by_second:true,
      caller_messages_mutated:true,
      supplied_serializer_not_called:true
    },
    result:"Four adverse behaviors reproduced; this is not a qualification PASS.",
    scope:"Exact upstream strategy and actual LocalFileAdapter, native type stripping; no models, network or package installs."
  },null,2));
} finally { await rm(dir,{recursive:true,force:true}); }
