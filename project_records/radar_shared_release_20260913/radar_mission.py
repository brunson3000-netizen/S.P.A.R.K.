"""Mission-local RADAR V1.1 fixtures, scorer, and bounded runner.

This file is deliberately outside production infrastructure. It imports the
existing OpenRouter adapter from the toolkit worktree at runtime. V1.0 files
remain preserved in ``frozen_package``.
"""
from __future__ import annotations
import argparse, hashlib, json, os, re, sys, time, uuid, fcntl
from dataclasses import dataclass, asdict
from pathlib import Path

DIMENSIONS = ("CODE_REPAIR", "CODE_REVIEW", "STRUCTURED_OUTPUT", "TOOL_USE",
              "INSTRUCTION_ADHERENCE", "REASONING_PLANNING",
              "EXTRACTION_TRANSFORMATION", "LONG_CONTEXT_RETRIEVAL")
BATTERY_VERSION = "RADAR-V1.1.0"
SCORER_VERSION = "RADAR-V1.1-MECHANICAL-1"
MAX_INPUT_TOKENS = 3500
MAX_OUTPUT_TOKENS = 1024
MAX_TOOL_TURNS = 3
MAX_RETRIES = 1
TIMEOUT_SECONDS = 120
# The production InferenceRequest/InferenceResponse contracts do not carry
# native tool definitions/results. Until the mission-local tool zipper exists,
# TOOL_USE is explicitly not scored as a model capability.
RUNNER_UNSUPPORTED_DIMENSIONS = {"TOOL_USE"}
SHARED_LEDGER = Path("/home/chromikey/Projects/SPARK/project_records/radar_shared_release_20260913/mission_spend_ledger.json")

@dataclass(frozen=True)
class Fixture:
    fixture_id: str
    dimension: str
    slot: int
    prompt: str
    oracle: dict

def _f(dim, slot, prompt, oracle):
    return Fixture(f"{dim.lower()}-{slot+1:02d}-v11", dim, slot, prompt, oracle)

def build_fixtures():
    rows=[]
    repairs=[
        ("rhea", "def clamp(x):\n    return max(0, min(x, 10))", "clamp(-2) == 0 and clamp(14) == 10", "repair-clamp-bounds"),
        ("sable", "def first_unique(xs):\n    return list(set(xs))[0]", "preserve input order and return every first occurrence", "repair-stable-unique"),
        ("tamar", "def parse_port(text):\n    return int(text.split(':')[-1])", "reject ports outside 1..65535 and malformed suffixes", "repair-port-validation"),
        ("umber", "def merge(a,b):\n    return {**a, **b}", "the left value wins on conflicting keys", "repair-left-biased-merge"),
        ("vesper", "def retryable(code):\n    return code >= 400", "only 408, 429, and 503 are retryable", "repair-retry-set"),
    ]
    for i,(name,code,inv,ans) in enumerate(repairs):
        rows.append(_f("CODE_REPAIR",i,f"Repair this function for the stated invariant.\nNAME={name}\nCODE:\n{code}\nINVARIANT: {inv}\nReturn only JSON {{patch_id}}. Use a stable descriptive identifier for the minimal behavioral fix.",{"patch_id":ans}))
    reviews=[
        ("cache.py", "if key in cache: return cache[key]", "cache[key] = value", ["R1-missing-invalidation"]),
        ("auth.py", "if token: grant_admin()", "audit.log(user, token)", ["R2-token-presence-escalates"]),
        ("queue.py", "items.sort()", "return items.pop(0)", ["R3-mutates-shared-order"]),
        ("parser.py", "value = int(raw)", "except ValueError: value = 0", ["R4-invalid-input-collapses"]),
        ("writer.py", "open(path, 'w')", "write(payload)", ["R5-nonatomic-overwrite"]),
    ]
    for i,(file,old,new,ans) in enumerate(reviews):
        rows.append(_f("CODE_REVIEW",i,f"Review this diff and identify only real defects.\nFILE {file}\n- {old}\n+ {new}\nClean distractor: add type annotation to helper().\nReturn only JSON {{defects: [defect_id]}}.",{"defects":ans}))
    structs=[
        ({"type":"object","required":["ticket","severity"],"properties":{"ticket":{"type":"string"},"severity":{"enum":["low","high"]}}},{"ticket":"T-ALPHA","severity":"high"}),
        ({"type":"object","required":["owner","enabled"],"properties":{"owner":{"type":"string"},"enabled":{"type":"boolean"}}},{"owner":"team-cobalt","enabled":False}),
        ({"type":"object","required":["region","replicas"],"properties":{"region":{"type":"string"},"replicas":{"type":"integer"}}},{"region":"eu-west","replicas":3}),
        ({"type":"object","required":["mode","threshold"],"properties":{"mode":{"enum":["safe","fast"]},"threshold":{"type":"number"}}},{"mode":"safe","threshold":0.75}),
        ({"type":"object","required":["name","labels"],"properties":{"name":{"type":"string"},"labels":{"type":"array"}}},{"name":"spark-worker","labels":["mci","radar"]}),
    ]
    for i,(schema,ans) in enumerate(structs):
        source = "; ".join(f"{k}={json.dumps(v)}" for k,v in ans.items())
        rows.append(_f("STRUCTURED_OUTPUT",i,f"Transform the source record ({source}) into one JSON object satisfying this schema exactly: {json.dumps(schema,sort_keys=True)}. Do not add keys.",ans))
    tools=[
        ([{"name":"lookup","args":{"key":"amber-17"}},{"name":"summarize","args":{"style":"short"}}],[{"tool":"lookup","args":{"key":"amber-17"}},{"tool":"summarize","args":{"style":"short"}}]),
        ([{"name":"read","args":{"path":"/sandbox/input.csv"}},{"name":"count","args":{"field":"id"}}],[{"tool":"read","args":{"path":"/sandbox/input.csv"}},{"tool":"count","args":{"field":"id"}}]),
        ([{"name":"get","args":{"id":"job-204"}},{"name":"approve","args":{"id":"job-204"}}],[{"tool":"get","args":{"id":"job-204"}},{"tool":"approve","args":{"id":"job-204"}}]),
        ([{"name":"list","args":{"prefix":"mci/"}},{"name":"sort","args":{"by":"name"}}],[{"tool":"list","args":{"prefix":"mci/"}},{"tool":"sort","args":{"by":"name"}}]),
        ([{"name":"fetch","args":{"url":"https://sandbox.invalid/a"}},{"name":"hash","args":{"algorithm":"sha256"}}],[{"tool":"fetch","args":{"url":"https://sandbox.invalid/a"}},{"tool":"hash","args":{"algorithm":"sha256"}}]),
    ]
    for i,(defs,ans) in enumerate(tools): rows.append(_f("TOOL_USE",i,f"Use only these fake sandbox tools. Definitions: {json.dumps(defs,sort_keys=True)}. Inspect the request context and produce the minimal ordered calls needed to complete it. Return only JSON {{calls}}.",{"calls":ans}))
    instr=[(["keep order","allow keys: id,value","forbid: debug"],[{"id":"a7","value":4}],{"count":1,"mode":"strict"}),(["reverse order","allow keys: name","forbid: null"],[{"name":"north"},{"name":"south"}],{"count":2,"mode":"strict"}),(["sort ascending","allow keys: code","forbid: explanation"],[{"code":"A1"},{"code":"B2"}],{"count":2,"mode":"strict"}),(["deduplicate","allow keys: tag","forbid: metadata"],[{"tag":"core"},{"tag":"edge"}],{"count":2,"mode":"strict"}),(["emit one item","allow keys: state","forbid: extra"],[{"state":"ready"}],{"count":1,"mode":"strict"})]
    for i,(rules,items,meta) in enumerate(instr): rows.append(_f("INSTRUCTION_ADHERENCE",i,f"Apply every rule: {rules}. Return only JSON with items and the exact metadata fields count/mode.",{"items":items,**meta}))
    plans=[(["A before C","B before D"],["A","B","C","D"]),(["inspect before patch","patch before test"],["inspect","patch","test"]),(["load before parse","parse before emit"],["load","parse","emit"]),(["backup before migrate","migrate before verify"],["backup","migrate","verify"]),(["design before build","build before deploy"],["design","build","deploy"])]
    for i,(constraints,ans) in enumerate(plans): rows.append(_f("REASONING_PLANNING",i,f"Find a valid schedule satisfying all precedence constraints {constraints}. Return only JSON {{schedule}}.",{"schedule":ans}))
    extracts=[("name=quartz; version=4.2; active=no; stale_version=1.0",{"name":"quartz","version":"4.2","active":False}),("owner=blue; revision=17; owner_old=red",{"owner":"blue","revision":17}),("service=relay; tier=gold; tier_old=silver",{"service":"relay","tier":"gold"}),("id=K9; enabled=true; enabled_old=false",{"id":"K9","enabled":True}),("project=atlas; region=us-central; region_old=eu",{"project":"atlas","region":"us-central"})]
    for i,(src,ans) in enumerate(extracts): rows.append(_f("EXTRACTION_TRANSFORMATION",i,f"Extract the current fields from this record and ignore explicitly stale fields: {src}. Return only the target JSON shape.",ans))
    contexts=[("owner=team-iris revision=8; superseded owner=team-ash revision=7",{"current_owner":"team-iris","current_revision":8}),("status=green at 2026-09-10; superseded status=yellow at 2026-09-01",{"current_status":"green","current_date":"2026-09-10"}),("quota=120; old_quota=80",{"current_quota":120}),("schema=v3; prior_schema=v2",{"current_schema":"v3"}),("region=ap-south; prior_region=eu-north",{"current_region":"ap-south"})]
    for i,(ctx,ans) in enumerate(contexts): rows.append(_f("LONG_CONTEXT_RETRIEVAL",i,f"Retrieve only the current fact from this context; superseded values are distractors: {ctx}. Return only JSON.",ans))
    assert len(rows)==40
    return tuple(rows)

def _sha_bytes(v): return hashlib.sha256(v).hexdigest()
def materialize(out):
    out=Path(out); out.mkdir(parents=True,exist_ok=True); fs=build_fixtures()
    public={"battery_version":BATTERY_VERSION,"fixtures":[{"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"subject_prompt":f.prompt} for f in fs]}
    private={"battery_version":BATTERY_VERSION,"fixtures":[{**x,"scorer_kind":"MECHANICAL"} for x in [{"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"subject_prompt":f.prompt,"oracle":f.oracle} for f in fs]]}
    scorer={"scorer_version":SCORER_VERSION,"raw_range":[0,1],"normalized_range":[0,5],"unknown_is_not_zero":True,"transport_failures":"UNKNOWN","tool_turns":MAX_TOOL_TURNS}
    for name,obj in [("RADAR_V1_1_SUBJECT_FIXTURES.json",public),("RADAR_V1_1_FIXTURE_PACK.json",private),("RADAR_V1_1_SCORER_CONTRACT.json",scorer)]: (out/name).write_text(json.dumps(obj,sort_keys=True,indent=2)+"\n")
    man={"battery_version":BATTERY_VERSION,"scorer_version":SCORER_VERSION,"fixture_count":40,"dimensions":list(DIMENSIONS),"cheap_fixture_slots":[0,1],"fixture_pack_sha256":_sha_bytes((out/'RADAR_V1_1_FIXTURE_PACK.json').read_bytes()),"subject_pack_sha256":_sha_bytes((out/'RADAR_V1_1_SUBJECT_FIXTURES.json').read_bytes()),"scorer_contract_sha256":_sha_bytes((out/'RADAR_V1_1_SCORER_CONTRACT.json').read_bytes()),"oracle_exposure":"PRIVATE_SCORER_ONLY"}
    shared=out.parent/'RADAR_SHARED_RELEASE_MANIFEST.json'
    if shared.exists():
        man["assignments"]=json.loads(shared.read_text())["assignments"]
        man["execution_contract"]=json.loads(shared.read_text())["execution_contract"]
    man["manifest_payload_sha256"]=_sha_bytes(json.dumps(man,sort_keys=True,separators=(",",":")).encode()); (out/'RADAR_V1_1_BATTERY_MANIFEST.json').write_text(json.dumps(man,sort_keys=True,indent=2)+"\n"); return man

def score(f, text, status="SCORED"):
    if status!="SCORED": return {"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":status,"raw_score":None,"normalized_score":None}
    try: value=json.loads(text)
    except Exception: value=None
    ok=value==f.oracle
    return {"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":"SCORED","raw_score":1.0 if ok else 0.0,"normalized_score":5.0 if ok else 0.0,"reason":"exact_match" if ok else "oracle_mismatch"}

def validate(out):
    fs=build_fixtures(); assert len(fs)==40 and len({f.fixture_id for f in fs})==40
    assert {f.dimension for f in fs}==set(DIMENSIONS)
    assert all(json.dumps(f.oracle,sort_keys=True) not in f.prompt for f in fs)
    assert all((not f.oracle.get("patch_id")) or f.oracle["patch_id"] not in f.prompt for f in fs)
    assert all(not re.search(r"(?:task|fixture|record)\s+[1-5]",f.prompt,re.I) for f in fs)
    man=materialize(out); return {"fixtures":40,"dimensions":8,"guessability_check":"PASS","manifest":man}

def run_lane(manifest, lane, out):
    """Run exact assigned models; imports only the existing adapter seam."""
    sys.path.insert(0,"/home/chromikey/SWARM/worktrees/compute-qualification-toolkit-v1")
    from swarm.mci_v2.local_ai.remote_providers import OpenRouterInferenceProvider
    from swarm.mci_v2.local_ai.linux_secret_service import LinuxSecretServiceCredentialStore
    from swarm.mci_v2.local_ai.secure_access import CredentialBroker
    from swarm.mci_v2.local_ai.contracts import InferenceRequest
    from decimal import Decimal
    d=json.loads(Path(manifest).read_text()); rows=[x for x in d["assignments"] if x["lane"]==lane]
    root=Path(out); root.mkdir(parents=True,exist_ok=True); ledger=SHARED_LEDGER; lock=ledger.with_suffix(".lock")
    ledger.parent.mkdir(parents=True,exist_ok=True)
    if not ledger.exists():
        ledger.write_text(json.dumps({"allocations":{"A":4.0,"B":4.0,"C":4.0,"ASSESSMENT":3.0},"reserved":{"A":0.0,"B":0.0,"C":0.0,"ASSESSMENT":0.0},"actual":{"A":0.0,"B":0.0,"C":0.0,"ASSESSMENT":0.0},"events":[]},indent=2)+"\n")
    manifest_dir=Path(manifest).parent
    pack_dir=manifest_dir if manifest_dir.name=='v1_1_package' else manifest_dir/'v1_1_package'
    pack_path=pack_dir/'RADAR_V1_1_FIXTURE_PACK.json'
    if not pack_path.exists(): raise RuntimeError(f"private scorer pack missing: {pack_path}")
    fs=tuple(Fixture(x["fixture_id"],x["dimension"],x["slot"],x["subject_prompt"],x["oracle"]) for x in json.loads(pack_path.read_text())["fixtures"])
    creds=CredentialBroker(secure_store=LinuxSecretServiceCredentialStore()); creds.credential("remote.openrouter")
    all_results=root/"results.jsonl"
    for model in rows:
        provider=OpenRouterInferenceProvider(model["model_id"],creds,"session.remote.openrouter",route_provider_slug=model["route"],timeout_seconds=60)
        for f in fs:
            for rep in range(3):
                key=f"{lane}|{model['model_id']}|{model['route']}|{f.fixture_id}|{rep+1}"
                if all_results.exists() and any(key in line for line in all_results.read_text().splitlines()): continue
                unsupported = f.dimension in (set(model.get("unsupported_dimensions", [])) | RUNNER_UNSUPPORTED_DIMENSIONS)
                if unsupported:
                    reason = "runner_native_tool_contract_unavailable" if f.dimension in RUNNER_UNSUPPORTED_DIMENSIONS else "declared_route_capability_absent"
                    record={"key":key,"lane":lane,"model_id":model["model_id"],"provider_route":model["route"],"fixture_id":f.fixture_id,"rep":rep+1,"started_at":time.time(),"ended_at":time.time(),"attempts":[],"result":{"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":"UNSUPPORTED","raw_score":None,"normalized_score":None,"reason":reason}}
                    with all_results.open("a") as h: h.write(json.dumps(record,sort_keys=True)+"\n")
                    continue
                started=time.time(); attempts=[]; final=None
                for attempt in range(MAX_RETRIES+1):
                    response=None
                    # Reserve the worst-case physical attempt atomically in the
                    # mission ledger. A retry is a new reservation and cannot
                    # borrow another lane's allocation.
                    bound=240.0*(3500.0*float(model.get("prompt_price",0.0))+1024.0*float(model.get("completion_price",0.0)))
                    with lock.open("a+") as lh:
                        fcntl.flock(lh, fcntl.LOCK_EX)
                        state=json.loads(ledger.read_text())
                        if state["reserved"][lane]+state["actual"][lane]+bound > state["allocations"][lane]+1e-9:
                            attempts.append({"status":"budget_exhausted","bound_usd":bound})
                            final={"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":"UNKNOWN","raw_score":None,"normalized_score":None,"reason":"budget_exhausted_before_call"}
                            fcntl.flock(lh, fcntl.LOCK_UN); break
                        state["reserved"][lane]+=bound
                        state["events"].append({"kind":"reserve","lane":lane,"key":key,"attempt":attempt+1,"bound_usd":bound,"at":time.time()})
                        ledger.write_text(json.dumps(state,sort_keys=True,indent=2)+"\n")
                        fcntl.flock(lh, fcntl.LOCK_UN)
                    try:
                        response=provider.infer(InferenceRequest(f.prompt,timeout_seconds=60,request_id=key+f"|a{attempt+1}",correlation_id=key,maximum_output_tokens=MAX_OUTPUT_TOKENS))
                        identity_ok = bool(response.returned_model_ref) and response.returned_model_ref == model["model_id"]
                        route_ok = bool(response.returned_route_id) and response.returned_route_id == model["route"]
                        complete = response.completion_state not in ("UNKNOWN", "TRUNCATED") and response.finish_reason != "length"
                        final=score(f,response.text) if identity_ok and route_ok and complete else {"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":"UNKNOWN","raw_score":None,"normalized_score":None,"reason":"identity_or_completion_unverified"}
                        final.update({"finish_reason":response.finish_reason,"completion_state":response.completion_state,"usage":{"input_tokens":response.input_tokens,"output_tokens":response.output_tokens},"returned_model":response.returned_model_ref,"returned_route":response.returned_route_id,"attempt":attempt+1,"raw_response_text":response.text,"identity_verified":identity_ok and route_ok}); attempts.append({"status":"success" if identity_ok and route_ok else "identity_mismatch"}); break
                    except Exception as e: attempts.append({"status":"transport_error","type":type(e).__name__,"detail":str(e)[:500]}); final={"fixture_id":f.fixture_id,"dimension":f.dimension,"slot":f.slot,"status":"UNKNOWN","raw_score":None,"normalized_score":None,"reason":"transport_failure","error":type(e).__name__}
                    finally:
                        with lock.open("a+") as lh:
                            fcntl.flock(lh, fcntl.LOCK_EX); state=json.loads(ledger.read_text()); state["reserved"][lane]-=bound
                            cost=float(getattr(locals().get("response",None),"actual_cost","") or getattr(locals().get("response",None),"estimated_cost","") or bound)
                            if not isinstance(cost,(int,float)) or cost<0: cost=bound
                            state["actual"][lane]+=min(cost,bound); state["events"].append({"kind":"settle","lane":lane,"key":key,"attempt":attempt+1,"cost_or_bound_usd":min(cost,bound),"at":time.time()}); ledger.write_text(json.dumps(state,sort_keys=True,indent=2)+"\n"); fcntl.flock(lh, fcntl.LOCK_UN)
                record={"key":key,"lane":lane,"model_id":model["model_id"],"provider_route":model["route"],"fixture_id":f.fixture_id,"rep":rep+1,"started_at":started,"ended_at":time.time(),"attempts":attempts,"result":final}
                with all_results.open("a") as h: h.write(json.dumps(record,sort_keys=True)+"\n")
    return {"lane":lane,"results":str(all_results)}

def main():
    p=argparse.ArgumentParser(); sub=p.add_subparsers(dest="cmd",required=True); v=sub.add_parser("validate"); v.add_argument("--out",required=True); m=sub.add_parser("materialize"); m.add_argument("--out",required=True); r=sub.add_parser("run-lane"); r.add_argument("--manifest",required=True); r.add_argument("--lane",required=True); r.add_argument("--out",required=True); a=p.parse_args()
    if a.cmd=="validate": print(json.dumps(validate(a.out),sort_keys=True))
    elif a.cmd=="materialize": print(json.dumps(materialize(a.out),sort_keys=True))
    else: print(json.dumps(run_lane(a.manifest,a.lane,a.out),sort_keys=True))
if __name__=="__main__": main()
