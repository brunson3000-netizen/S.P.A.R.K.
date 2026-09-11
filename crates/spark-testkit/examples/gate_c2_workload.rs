//! Gate C2 long-history workload check (acceptance pin 2).
//!
//! Not a production benchmark and not part of the test suite: a representative,
//! single-machine measurement run in `--release`, whose output is recorded as
//! evidence. It builds finalized histories of increasing length through the
//! public engine (`process`), then measures:
//!
//! 1. **Preflight only** — commands refused at row P-9 (a source-sequence
//!    regression). P-1 … P-9 are all evaluated, including the PX-1 indexed
//!    lookups of P-7 … P-9, and no `stage` or `submit_fence` runs.
//! 2. **The forbidden alternative** — the same three predicates evaluated by
//!    scanning `finalized_commands()` (what the pin prohibits in production),
//!    as a reference only.
//! 3. **Successful finalization** — the whole operation. This includes Phase-1
//!    `submit_fence`, whose `FinalizationResult` recomputes
//!    `canonical_history_digest()` over the complete finalized history: an
//!    inherited `O(history)` cost that Gate C2 does not change (Phase-1 is
//!    closed) and that is reported separately rather than attributed to the
//!    preflight.
//!
//! Run: `cargo run --release --offline -p spark-testkit --example gate_c2_workload`

use spark_core::id::SourceId;
use spark_engine::engine::Engine;
use spark_engine::request::{CommandRequest, FinalizationRefusal, Outcome, Request};
use spark_testkit::phase2::*;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn command(id: String, sequence: u64) -> CommandRequest {
    command_request(&id, 1, sequence, "cmd.noop", actor("bron"), vec![], vec![])
}

/// The prohibited production alternative: P-7 … P-9 by scanning history.
fn scan_rows(e: &Engine, c: &CommandRequest) -> bool {
    let history = e.finalized_commands();
    let id_claimed = history
        .iter()
        .any(|f| f.envelope.command_id == c.command_id);
    let pair_claimed = history.iter().any(|f| {
        f.envelope.source_id == c.source_id && f.envelope.source_sequence == c.source_sequence
    });
    let last = history
        .iter()
        .rev()
        .find(|f| f.envelope.source_id == c.source_id)
        .map(|f| f.envelope.source_sequence);
    !id_claimed && !pair_claimed && last.is_none_or(|l| c.source_sequence > l)
}

fn per_op(d: Duration, n: u32) -> f64 {
    d.as_secs_f64() * 1e9 / f64::from(n)
}

fn main() {
    println!(
        "history_len,build_s,preflight_ns_per_call,scan_reference_ns_per_call,finalize_ns_per_call"
    );
    for n in [1_000u64, 4_000, 16_000] {
        let mut f = standard();
        let mut e = f.engine(budgets(4), vec![], vec![]);
        let started = Instant::now();
        for i in 0..n {
            let r = e.process(&Request::Command(command(format!("h{i}"), 10 + i)));
            assert_eq!(r.outcome(), &Outcome::Completed);
        }
        let build = started.elapsed();
        assert_eq!(e.finalized_commands().len() as u64, n);

        const REFUSALS: u32 = 2_000;
        let refused: Vec<Request> = (0..REFUSALS)
            .map(|j| Request::Command(command(format!("r{j}"), 5)))
            .collect();
        let started = Instant::now();
        for r in &refused {
            let result = e.process(r);
            assert!(matches!(
                result.outcome(),
                Outcome::CompletedCommandNotFinalized(
                    FinalizationRefusal::SourceSequenceNotIncreasing { .. }
                )
            ));
        }
        let preflight = started.elapsed();

        const SCANS: u32 = 200;
        let probe = command("probe".to_string(), 5);
        let started = Instant::now();
        for _ in 0..SCANS {
            black_box(scan_rows(black_box(&e), black_box(&probe)));
        }
        let scan = started.elapsed();

        const SUCCESSES: u32 = 200;
        let next = 10 + n;
        let started = Instant::now();
        for j in 0..SUCCESSES {
            let r = e.process(&Request::Command(command(
                format!("s{j}"),
                next + u64::from(j),
            )));
            assert_eq!(r.outcome(), &Outcome::Completed);
        }
        let finalize = started.elapsed();
        let _ = SourceId::new(SOURCE);
        println!(
            "{n},{:.3},{:.0},{:.0},{:.0}",
            build.as_secs_f64(),
            per_op(preflight, REFUSALS),
            per_op(scan, SCANS),
            per_op(finalize, SUCCESSES)
        );
    }
}
