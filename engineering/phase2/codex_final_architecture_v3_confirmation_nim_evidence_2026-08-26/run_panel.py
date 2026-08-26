#!/usr/bin/env python3
"""Run the bounded independent NIM panel for the Phase-2 v3 confirmation."""

from __future__ import annotations

import dataclasses
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import sys


REPOSITORY = Path(__file__).resolve().parents[3]
EVIDENCE_DIR = Path(__file__).resolve().parent
NIM_TOOLS = Path(
    "/home/chromikey/SWARM/worktrees/nim-compute-study-v1/"
    "research/nim_compute_study_2026_08_26/tools"
)
sys.path.insert(0, str(NIM_TOOLS))

from nim_gateway import CompletionRequest, NimGateway  # noqa: E402


SOURCES = (
    "engineering/phase2/SPARK_PHASE_2_CODEX_ARCHITECTURE_V2_REREVIEW_2026-08-26.md",
    "engineering/phase2/SPARK_PHASE_2_OPUS_ARCHITECTURE_FREEZE_v3_2026-08-26.md",
    "engineering/phase2/SPARK_PHASE_2_ACCEPTANCE_TEST_MATRIX_v3_2026-08-26.md",
    "engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_v2_2026-08-26.md",
    "engineering/phase2/SPARK_PHASE_2_FABLE_ARCHITECTURE_FREEZE_2026-08-26.md",
    "crates/spark-core/src/scheduler.rs",
)

COMMON_SYSTEM = """You are an independent architecture-review worker. Review only the
committed evidence in the prompt. Do not assume an implementation exists. Do not invent
new Phase-2 scope or reopen a cleared decision merely because another design is possible.
A blocker requires: a concrete counterexample, the violated controlling invariant,
architectural impact, and the smallest safe correction. Distinguish architecture defects
from test-only defects and implementation choices. Return a concise evidence-grounded
report ending with exactly KEEP or REVISE. You are advisory; Codex remains adjudicator."""

TASKS = (
    {
        "role": "architecture_consistency",
        "model": "nvidia/nemotron-3-ultra-550b-a55b",
        "task": """Trace the v2 B01/B02 findings into v3 and its matrix. Attack internal
consistency across inherited scheduler semantics, cohort selection, engine digest,
stable-boundary claims, and the no-hidden-state rule. Confirm or falsify that v3 is a
complete, implementable freeze. Report only concrete blockers.""",
    },
    {
        "role": "determinism_failure_mode",
        "model": "moonshotai/kimi-k3",
        "task": """Model exact state transitions for one-call catch-up versus repeated
cohort/drain execution. Pay special attention to Scheduler::drain_due removing the due
prefix, scheduler participation in engine_state_digest, pre-wave engine digest in
effect_batch_v3, later cohorts claimed to remain untouched, and AT-I39 byte equality.
Also attack oversized cohorts, terminal caps, starvation, and reconstruction. Give a
minimal execution trace for any counterexample.""",
    },
    {
        "role": "adversarial_identity",
        "model": "openai/gpt-oss-20b",
        "task": """Attack B02 identity closure. Exercise multiple WorkKeys, equal numeric
results from different parent sets, wave 0 leaves, wave N+1 parent unions, parent-order
permutations, paced/unbudgeted partitions, occurrence allocation, obligation and batch
digests, exact replay, and reconstruction. Decide whether any canonical identity still
depends on drain partition or hidden state. Report only falsifiable defects.""",
    },
    {
        "role": "test_oracle",
        "model": "nvidia/nemotron-3-nano-30b-a3b",
        "task": """Audit every v3 acceptance requirement as an implementation oracle,
with focus on AT-I6c/d, I20/c/d, I25, I29, and I39-I41. Reject error-only assertions where
state/digest/value equality is necessary. Check whether any demanded equality is
impossible or underdefined under the frozen source/API, and state the smallest test or
architecture correction. Report only concrete blockers.""",
    },
)


def sha256_text(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def source_packet() -> tuple[str, list[dict[str, object]]]:
    blocks: list[str] = []
    metadata: list[dict[str, object]] = []
    for relative in SOURCES:
        path = REPOSITORY / relative
        content = path.read_text(encoding="utf-8")
        blocks.append(f"\n===== BEGIN {relative} =====\n{content}\n===== END {relative} =====\n")
        metadata.append(
            {
                "path": relative,
                "bytes": len(content.encode("utf-8")),
                "sha256": sha256_text(content),
            }
        )
    return "".join(blocks), metadata


def run_one(gateway: NimGateway, packet: str, task: dict[str, str]) -> dict[str, object]:
    prompt = (
        "S.P.A.R.K. Phase-2 final v3 architecture-confirmation evidence follows.\n\n"
        f"ROLE: {task['role']}\nTASK: {task['task']}\n"
        f"\nEVIDENCE PACKET SHA-256: {sha256_text(packet)}\n{packet}"
    )
    started = datetime.now(timezone.utc).isoformat()
    result = gateway.complete(
        CompletionRequest(
            model=task["model"],
            prompt=prompt,
            system=COMMON_SYSTEM,
            max_tokens=6000,
            temperature=0.1,
            timeout_s=900,
            max_attempts=3,
        )
    )
    finished = datetime.now(timezone.utc).isoformat()
    return {
        "role": task["role"],
        "model": task["model"],
        "task": task["task"],
        "system_prompt": COMMON_SYSTEM,
        "prompt_sha256": sha256_text(prompt),
        "evidence_packet_sha256": sha256_text(packet),
        "started_at_utc": started,
        "finished_at_utc": finished,
        "gateway_credential_source": gateway.credential_source,
        "result": dataclasses.asdict(result),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--only-role", choices=tuple(task["role"] for task in TASKS))
    parser.add_argument("--model", help="replacement model for a single failed seat")
    arguments = parser.parse_args()
    packet, sources = source_packet()
    telemetry = EVIDENCE_DIR / "telemetry.jsonl"
    gateway = NimGateway(max_inflight=4, telemetry_path=str(telemetry))
    if arguments.only_role:
        selected = next(task for task in TASKS if task["role"] == arguments.only_role)
        task = dict(selected)
        if arguments.model:
            task["model"] = arguments.model
        record = run_one(gateway, packet, task)
        safe_model = task["model"].replace("/", "_").replace(":", "_")
        output_name = f"{task['role']}__fallback__{safe_model}.json"
        (EVIDENCE_DIR / output_name).write_text(
            json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        fallback = {
            "reason": "bounded replacement for a failed/not-deployed panel seat; successful seats were not rerun",
            "source_files": sources,
            "evidence_packet_sha256": sha256_text(packet),
            "role": task["role"],
            "model": task["model"],
            "raw_response_file": output_name,
            "ok": record["result"]["ok"],
            "http_status": record["result"]["http_status"],
            "finish_reason": record["result"]["finish_reason"],
            "usage": record["result"]["usage"],
            "latency_s": record["result"]["latency_s"],
        }
        (EVIDENCE_DIR / "fallback_manifest.json").write_text(
            json.dumps(fallback, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        print(json.dumps(fallback, indent=2, sort_keys=True))
        return 0 if record["result"]["ok"] else 1

    records: list[dict[str, object]] = []
    with ThreadPoolExecutor(max_workers=4) as pool:
        futures = {pool.submit(run_one, gateway, packet, task): task for task in TASKS}
        for future in as_completed(futures):
            record = future.result()
            records.append(record)
            output = EVIDENCE_DIR / f"{record['role']}.json"
            output.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    records.sort(key=lambda item: str(item["role"]))
    manifest = {
        "purpose": "supplementary independent NIM panel for Phase-2 v3 final architecture confirmation",
        "independence": "workers ran concurrently from the same committed evidence and received no other worker output",
        "repository": str(REPOSITORY),
        "evidence_packet_sha256": sha256_text(packet),
        "source_files": sources,
        "gateway_tools_path": str(NIM_TOOLS),
        "gateway_tools_commit": "nim-compute-study-v1 worktree; benchmark study not rerun",
        "workers": [
            {
                "role": record["role"],
                "model": record["model"],
                "ok": record["result"]["ok"],
                "http_status": record["result"]["http_status"],
                "finish_reason": record["result"]["finish_reason"],
                "usage": record["result"]["usage"],
                "latency_s": record["result"]["latency_s"],
                "raw_response_file": f"{record['role']}.json",
            }
            for record in records
        ],
    }
    (EVIDENCE_DIR / "manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return 0 if all(worker["ok"] for worker in manifest["workers"]) else 1


if __name__ == "__main__":
    raise SystemExit(main())
