//! S.P.A.R.K.–G.A.M.E. safe handoff: targeted discriminating diagnostics.
//!
//! Target tree: crates/ at `7907f4d729104fd5dbfd4adad46e66cf09aa13dd`, the exact
//! crate tree of pinned candidate `67b877192cc78b75c6fbe60c69b5594dc10befe8`.
//!
//! Each test discriminates between two competing explanations of an observation
//! recorded by the LUNA or SONNET adversarial campaign, or closes a coverage gap
//! those campaigns declared. No product file is modified by this harness.
#![allow(clippy::all)]

use spark_core::clock::LogicalTime;
use spark_core::hash::Digest;
use spark_core::scope::ScopeId;
use spark_engine::engine::{Engine, RestoreError};
use spark_engine::fixture::*;
use spark_engine::report::{CohortOutcome, WaveRejection};
use spark_engine::request::{Outcome, Request};
use spark_engine::profile::manifest::ProfileManifest;
use spark_engine::profile::text::BoundedText;
use spark_engine::rules::{RuleSetError, Update};
use spark_testkit::phase2::*;

fn a() -> ScopeId {
    actor("a")
}

/// Both digests of the engine, for byte-identity assertions across a refusal.
fn digests(e: &Engine) -> (Digest, Digest) {
    digest_pair(e)
}

/// The canonical digest of the state store alone.
fn cells_digest(e: &Engine) -> Digest {
    e.state().canonical_state_digest()
}

// ===================================================================== D-1
// LUNA-001: are extreme assignments "unspecified", or declared-bounds refusals?
//
// Competing explanations:
//   (a) the campaign's: expected behavior for i64::MAX/i64::MIN is unspecified;
//   (b) the discriminating one: the value is refused because it violates the
//       *declared* ValueConstraint of the target definition in the activated
//       profile manifest, exactly as any other out-of-range value is.
//
// (b) is proven by showing the refusal boundary tracks the declared bound, not
// the i64 range: the declared maximum commits, maximum+1 refuses identically to
// i64::MAX, and each refusal is reported as a cohort-level
// InvalidEffect{OutOfBounds} naming the same definition.

const WIDE: i64 = 1_000_000_000_000; // the declared bound of state.pressure

fn pressure_engine() -> Engine {
    let mut f = standard();
    let rules = vec![on_command(
        "rule.set",
        "cmd.set",
        vec![emit("s", "state.pressure", Update::Assign(param(0)))],
    )];
    let g = f.genesis(budgets(50), rules, vec![]);
    Engine::genesis(g).unwrap()
}

fn set_pressure(e: &mut Engine, id: &str, t: u64, seq: u64, v: i64) -> CohortOutcome {
    let c = command_request(id, t, seq, "cmd.set", a(), vec![], vec![v]);
    let rs = drive(e, &Request::Command(c));
    let last = rs.last().unwrap();
    assert_eq!(
        last.outcome(),
        &Outcome::Completed,
        "D-1: the request boundary itself must complete for value {v}"
    );
    let reports = reports(&rs);
    let cohort = reports
        .iter()
        .find(|r| r.canonical_time == LogicalTime(t))
        .expect("D-1: a cohort report at the command time");
    cohort.outcome.clone()
}

fn is_out_of_bounds(o: &CohortOutcome) -> bool {
    matches!(
        o,
        CohortOutcome::Rejected {
            rejection: WaveRejection::InvalidEffect { .. },
            ..
        }
    )
}

#[test]
fn d1_luna001_refusal_boundary_tracks_the_declared_constraint() {
    let mut e = pressure_engine();

    // The declared maximum is accepted and committed.
    let at_max = set_pressure(&mut e, "cmd.1", 5, 1, WIDE);
    assert_eq!(
        at_max,
        CohortOutcome::Committed,
        "D-1 check 1: the declared maximum must commit"
    );
    assert_eq!(value(&e, "state.pressure", &a()), Some(WIDE));

    let before = digests(&e);
    let before_cells = cells_digest(&e);

    // One past the declared maximum refuses.
    let past_max = set_pressure(&mut e, "cmd.2", 6, 2, WIDE + 1);
    assert!(
        is_out_of_bounds(&past_max),
        "D-1 check 2: declared maximum + 1 must be an InvalidEffect refusal, got {past_max:?}"
    );

    // i64::MAX refuses in exactly the same way: it is not a distinct,
    // unspecified case, it is the same declared-bounds refusal.
    let extreme_hi = set_pressure(&mut e, "cmd.3", 7, 3, i64::MAX);
    assert_eq!(
        extreme_hi, past_max,
        "D-1 check 3: i64::MAX must produce the identical refusal to declared-max+1"
    );

    let extreme_lo = set_pressure(&mut e, "cmd.4", 8, 4, i64::MIN);
    assert!(
        is_out_of_bounds(&extreme_lo),
        "D-1 check 4: i64::MIN must be the same declared-bounds refusal, got {extreme_lo:?}"
    );
    let below_min = set_pressure(&mut e, "cmd.5", 9, 5, -WIDE - 1);
    assert_eq!(
        extreme_lo, below_min,
        "D-1 check 5: i64::MIN must produce the identical refusal to declared-min-1"
    );

    // Every refusal left the previously committed cell exactly as it was.
    assert_eq!(
        value(&e, "state.pressure", &a()),
        Some(WIDE),
        "D-1 check 6: refused assignments must not change the committed value"
    );
    assert_eq!(
        cells_digest(&e),
        before_cells,
        "D-1 check 7: the canonical state store must be unchanged by four refusals"
    );
    assert_ne!(
        before.0,
        digests(&e).0,
        "D-1 check 8: the engine state digest DOES move, because the refused commands \
         still finalize on the timeline; only the state store is unchanged"
    );
    println!(
        "D-1 PASS: refusal boundary is the declared constraint, not the i64 range; \
         state store unchanged across four refusals, while the timeline advances"
    );
}

#[test]
fn d1b_luna001_absent_cell_is_the_refusal_consequence_not_a_separate_behavior() {
    // LUNA observed "left the cell absent". Discriminate: absence is simply the
    // never-written initial condition, because the *first* write was the refused
    // one. A prior successful write followed by a refused extreme leaves the
    // prior value, proven above in D-1f. Here: refusal first => still absent.
    let mut e = pressure_engine();
    assert_eq!(value(&e, "state.pressure", &a()), None);
    let o = set_pressure(&mut e, "cmd.x", 5, 1, i64::MAX);
    assert!(is_out_of_bounds(&o));
    assert_eq!(
        value(&e, "state.pressure", &a()),
        None,
        "D-1b check 1: a refused first write creates no cell"
    );
    println!("D-1b PASS: cell absence after refusal is the unchanged initial condition");
}

// ===================================================================== D-2
// SONNET T-1: is CommandIdentityConflict a defect (non-idempotent resubmission)
// or the declared P-7 finalization refusal?
//
// Discriminating property: the refusal is keyed to the *command identity*, not
// to the request shape. A second command carrying the same payload under a new
// command id and source sequence finalizes normally. So the engine is not
// refusing "a repeat of this work"; it is refusing "a reuse of this identity",
// which is exactly what D-2/P-7 requires.

#[test]
fn d2_t1_identity_conflict_is_identity_scoped_not_payload_scoped() {
    let mut f = standard();
    let rules = vec![on_command(
        "rule.set",
        "cmd.set",
        vec![emit("s", "state.stress", Update::Assign(param(0)))],
    )];
    let g = f.genesis(budgets(50), rules, vec![]);
    let mut e = Engine::genesis(g).unwrap();

    let c = command_request("cmd.same", 5, 1, "cmd.set", a(), vec![], vec![42]);
    let r1 = drive(&mut e, &Request::Command(c.clone()));
    assert_eq!(r1.last().unwrap().outcome(), &Outcome::Completed);
    let before = digests(&e);

    // Exact resubmission: the typed P-7 refusal, and nothing moves.
    let r2 = drive(&mut e, &Request::Command(c.clone()));
    let out2 = r2.last().unwrap().outcome().clone();
    assert!(
        matches!(out2, Outcome::CompletedCommandNotFinalized(_)),
        "D-2a: exact resubmission must be a typed finalization refusal, got {out2:?}"
    );
    assert_eq!(
        before,
        digests(&e),
        "D-2b: a refused resubmission must leave both digests byte-identical"
    );

    // Same payload, fresh identity and sequence: finalizes.
    let c2 = command_request("cmd.fresh", 6, 2, "cmd.set", a(), vec![], vec![42]);
    let r3 = drive(&mut e, &Request::Command(c2));
    assert_eq!(
        r3.last().unwrap().outcome(),
        &Outcome::Completed,
        "D-2c: identical work under a fresh identity must finalize"
    );
    assert_ne!(
        before,
        digests(&e),
        "D-2d: the fresh-identity command must actually advance the engine"
    );
    println!("D-2 PASS: T-1 is an identity-scoped P-7 refusal, not a payload-level rejection");
}

// ===================================================================== D-3
// SONNET T-2 / T-3: are ZeroDelay and ZeroPacingBudget runtime hangs avoided by
// luck, or structurally unreachable static admission refusals?
//
// Discriminating property: the rule set never activates, so no Engine can exist
// that carries the hazard. Assert the typed construction error directly.

#[test]
fn d3_t2_t3_zero_delay_and_zero_budget_are_static_admission_refusals() {
    let mut f = standard();

    // T-2: a zero-delay self-reschedule cannot be activated at all.
    let r = with_schedule(
        work_rule(
            "rule.w",
            "work.w",
            vec![emit("s", "state.stress", Update::Add(lit(1)))],
        ),
        reevaluate_after("sch", 0, "work.w", "rule.w"),
    );
    let errs = f
        .rule_set(budgets(50), vec![r])
        .expect_err("D-3a: a zero-delay reschedule must fail rule-set activation");
    assert!(
        errs.iter().any(|e| matches!(e, RuleSetError::ZeroDelay { .. })),
        "D-3b: expected a typed ZeroDelay error, got {errs:?}"
    );

    // T-3: a zero pacing budget cannot be activated at all.
    let ok_rule = work_rule(
        "rule.w",
        "work.w",
        vec![emit("s", "state.stress", Update::Add(lit(1)))],
    );
    let errs = f
        .rule_set(budgets(0), vec![ok_rule])
        .expect_err("D-3c: a zero pacing budget must fail rule-set activation");
    assert!(
        errs.iter().any(|e| matches!(e, RuleSetError::ZeroPacingBudget)),
        "D-3d: expected the typed ZeroPacingBudget error specifically, got {errs:?}"
    );
    println!("D-3 PASS: T-2/T-3 hazards are refused at activation; no Engine can carry them");
}

// ===================================================================== D-4
// SONNET T-4: does an oversized first slice mean the pacing budget is ignored,
// or that the first slice is always admitted to guarantee progress?
//
// Discriminating property: with work at two distinct due times and budget 1,
// the first slice is admitted whole (oversize and all) but the engine then
// pauses rather than draining the second time's slice in the same call. A
// budget that was simply ignored would complete both in one call.

#[test]
fn d4_t4_oversize_first_slice_is_admitted_but_the_budget_still_paces() {
    let mut f = standard();
    let rules = vec![work_rule(
        "rule.w",
        "work.w",
        vec![emit("s", "state.stress", Update::Add(lit(1)))],
    )];
    // 20 items due at t=5 (one oversized slice) and 20 more due at t=6.
    let mut work: Vec<_> = (0..20).map(|_| initial("rule.w", &a(), 5, "work.w")).collect();
    work.extend((0..20).map(|_| initial("rule.w", &a(), 6, "work.w")));
    let g = f.genesis(budgets(1), rules, work);
    let mut e = Engine::genesis(g).unwrap();

    let req = advance(6);
    let first = e.process(&req);
    assert_eq!(
        first.outcome(),
        &Outcome::Paused,
        "D-4a: with two due times and budget 1 the first call must pause"
    );
    let d = first.diagnostics().expect("D-4b: pacing diagnostics");
    assert_eq!(
        d.declared_max_due_per_cycle, 1,
        "D-4c: the declared budget must be reported as 1"
    );
    assert!(
        d.pacing_overrun,
        "D-4d: admitting an oversized first slice must be reported as an overrun, \
         not silently treated as within budget"
    );
    assert!(
        d.deferred_cohort_count >= 1,
        "D-4e: the later due time must be deferred, proving the budget still paces"
    );

    // The same request then completes on resume.
    let rest = drive(&mut e, &req);
    assert_eq!(rest.last().unwrap().outcome(), &Outcome::Completed);
    println!(
        "D-4 PASS: T-4 is a declared progress guarantee with a reported overrun, \
         not an ignored budget (deferred cohorts > 0)"
    );
}

// ===================================================================== D-5
// SONNET T-5: request-level Completed with a cohort-level rejection. Is the
// refusal reported at all? Discriminating property: the two levels carry
// different, both-necessary information, and the cohort level names the exact
// definition and error. Covered by D-1 for the rejection content; here assert
// the two levels explicitly disagree by design on the same call.

#[test]
fn d5_t5_request_completion_and_cohort_rejection_are_distinct_levels() {
    let mut e = pressure_engine();
    let c = command_request("cmd.oob", 5, 1, "cmd.set", a(), vec![], vec![i64::MAX]);
    let rs = drive(&mut e, &Request::Command(c));
    let last = rs.last().unwrap();
    assert_eq!(
        last.outcome(),
        &Outcome::Completed,
        "D-5a: the request boundary completed"
    );
    let cohorts = reports(&rs);
    let rejected: Vec<_> = cohorts
        .iter()
        .filter(|r| is_out_of_bounds(&r.outcome))
        .collect();
    assert_eq!(
        rejected.len(),
        1,
        "D-5b: exactly one cohort must carry the rejection, got {}",
        rejected.len()
    );
    if let CohortOutcome::Rejected {
        rejection: WaveRejection::InvalidEffect { definition, .. },
        ..
    } = &rejected[0].outcome
    {
        assert_eq!(
            definition.as_str(),
            "state.pressure",
            "D-5c: the rejection must name the exact definition"
        );
    } else {
        panic!("D-5c: expected InvalidEffect");
    }
    println!("D-5 PASS: T-5 is two-level reporting; the cohort level names the refused definition");
}

// ===================================================================== D-6
// SONNET coverage gap: a *genuinely* distinct second profile for the
// mismatched-profile restore case. T-6 could not build one; content addressing
// made the "different" profile identical. Build one that differs in content.

#[test]
fn d6_gap_restore_against_a_genuinely_distinct_profile_is_refused() {
    // Engine A on the standard manifest.
    let mut fa = standard();
    let rules = vec![on_command(
        "rule.set",
        "cmd.set",
        vec![emit("s", "state.stress", Update::Assign(param(0)))],
    )];
    let ga = fa.genesis(budgets(50), rules.clone(), vec![]);
    let mut ea = Engine::genesis(ga).unwrap();
    drive(&mut ea, &Request::Command(command_request(
        "cmd.1", 5, 1, "cmd.set", a(), vec![], vec![7],
    )));
    let snap = ea.snapshot().expect("D-6a: a stable-boundary snapshot");

    // Engine B on a manifest with genuinely different *content*: one extra
    // definition. Its activation hash therefore differs from A's.
    let mut m = standard_manifest();
    m = {
        let mut defs: Vec<_> = m.definitions().to_vec();
        defs.push(int_spec("state.extra", spark_core::authority::Authority::SparkOwned, -10, 10));
        ProfileManifest::new(
            profile_id(),
            BoundedText::new("phase2.fixture.distinct").unwrap(),
            defs,
        )
    };
    let fb = activate(&m);
    assert_ne!(
        fa.profile.activation_hash(),
        fb.profile.activation_hash(),
        "D-6b: the second profile must genuinely differ (this is what T-6 could not build)"
    );

    let err = Engine::restore(snap, &fb.profile)
        .expect_err("D-6c: restoring A's snapshot into B must be refused");
    assert_eq!(
        err,
        RestoreError::ArtifactBindingMismatch,
        "D-6d: the refusal must be exactly ArtifactBindingMismatch — step 1 of restore \
         validation, reached before the epoch chain or the digest is consulted"
    );
    println!("D-6 PASS: coverage gap closed — a genuinely distinct profile refuses restore ({err:?})");
}

// ===================================================================== D-7
// SONNET coverage gap: the bidirectional-invariant obligation-drop mutation,
// which needed an internal WorkKey. The key is reachable from the public
// ObligationStore::keys() iterator.

#[test]
fn d7_gap_dropping_an_obligation_inside_a_snapshot_breaks_restore() {
    let mut f = standard();
    let rules = vec![with_schedule(
        work_rule(
            "rule.w",
            "work.w",
            vec![emit("s", "state.stress", Update::Add(lit(1)))],
        ),
        reevaluate_after("sch", 5, "work.w", "rule.w"),
    )];
    let g = f.genesis(budgets(50), rules, vec![initial("rule.w", &a(), 5, "work.w")]);
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &advance(5));

    let key = e
        .obligations()
        .keys()
        .next()
        .cloned()
        .expect("D-7a: a scheduled obligation exists after the first evaluation");

    // Baseline: an untouched snapshot restores.
    let clean = e.snapshot().expect("D-7b: snapshot");
    assert!(
        Engine::restore(clean, &f.profile).is_ok(),
        "D-7c: the unmutated snapshot must restore"
    );

    // Mutation: drop that one claim set, then reseal so the digest step cannot
    // be what catches it. The bidirectional invariant must catch it instead.
    let mut mutated = e.snapshot().expect("D-7d: snapshot");
    snapshot_drop_obligation(&mut mutated, &key);
    snapshot_reseal(&mut mutated);
    let err = Engine::restore(mutated, &f.profile)
        .expect_err("D-7e: a resealed snapshot missing an obligation must be refused");
    assert_eq!(
        err,
        RestoreError::BidirectionalInvariantBroken,
        "D-7f: expected BidirectionalInvariantBroken, got {err:?}"
    );
    println!("D-7 PASS: coverage gap closed — obligation-drop is caught by the bidirectional invariant, not by the digest");
}

// ===================================================================== D-8
// SONNET coverage gap: a command staged directly into a snapshot via
// snapshot_stage_raw. A stable-boundary snapshot must never carry staging.

#[test]
fn d8_gap_staged_command_inside_a_snapshot_is_refused_on_restore() {
    let mut f = standard();
    let rules = vec![on_command(
        "rule.set",
        "cmd.set",
        vec![emit("s", "state.stress", Update::Assign(param(0)))],
    )];
    let g = f.genesis(budgets(50), rules, vec![]);
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &Request::Command(command_request(
        "cmd.1", 5, 1, "cmd.set", a(), vec![], vec![3],
    )));

    let mut snap = e.snapshot().expect("D-8a: snapshot");
    let at = e.timeline_frontier_ordinal();
    let envelope = command_request("cmd.staged", 9, 2, "cmd.set", a(), vec![], vec![4])
        .envelope_at(at);
    let staged = snapshot_stage_raw(&mut snap, envelope);
    println!("D-8: staging at ordinal {at:?} -> {staged}");
    assert!(staged, "D-8b: the raw staging seam must have staged the envelope");
    snapshot_reseal(&mut snap);

    let err = Engine::restore(snap, &f.profile)
        .expect_err("D-8c: a snapshot carrying staging must be refused");
    assert_eq!(
        err,
        RestoreError::TimelineStagingPresent,
        "D-8d: expected TimelineStagingPresent, got {err:?}"
    );
    println!("D-8 PASS: coverage gap closed — staged-command snapshots are refused (Revision 2 §7 step 2b)");
}

// ===================================================================== D-9
// LUNA-002: zero-rate evaluation committing canonical time. Discriminate a
// no-op-that-still-commits from a silent state change: the cell value must be
// unchanged while canonical time is committed.

#[test]
fn d9_luna002_zero_rate_evaluation_commits_time_without_changing_value() {
    let mut f = standard();
    let rules = vec![
        on_command(
            "rule.seed",
            "cmd.set",
            vec![emit("s", "state.stress", Update::Assign(param(0)))],
        ),
        with_schedule(
            work_rule(
                "rule.d",
                "work.d",
                vec![emit("d", "state.stress", decay(0, 3))],
            ),
            reevaluate_after("sch", 3, "work.d", "rule.d"),
        ),
    ];
    let g = f.genesis_with(
        budgets(50),
        rules,
        vec![baseline("state.stress", 0)],
        vec![initial("rule.d", &a(), 3, "work.d")],
    );
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &Request::Command(command_request(
        "cmd.1", 1, 1, "cmd.set", a(), vec![], vec![50],
    )));
    let v0 = value(&e, "state.stress", &a());
    drive(&mut e, &advance(12));
    let v1 = value(&e, "state.stress", &a());
    assert_eq!(
        v0, v1,
        "D-9a: a zero-rate decay must not change the value (got {v0:?} -> {v1:?})"
    );
    assert_eq!(e.frontier(), LogicalTime(12), "D-9b: the frontier must still advance");
    println!("D-9 PASS: LUNA-002 is a value-preserving evaluation that still commits canonical time");
}

// ==================================================================== D-10
// C2W-RN02 (recorded, not optimized): composed additive settlement resolves the
// settled baseline more than once per written target per wave.
//
// Competing explanations:
//   (a) the repeated walk has a canonical effect — it bills decay debt twice,
//       so a body that takes the settled path commits a different value than
//       the same arithmetic taken on the additive-only path;
//   (b) the repeat is read-only, so both paths commit the same value and that
//       value is the one the declared semantics predict.
//
// This test discriminates by exercising TWO DIFFERENT CODE PATHS on identical
// state and comparing them against a hand-computed oracle:
//
//   * body A = [Add(10)]                        -> all-additive AddDelta path
//   * body B = [Add(10), Clamp(-WIDE, +WIDE)]   -> mixed body, settled path,
//                                                  with a clamp that binds
//                                                  nothing and so changes no
//                                                  value of its own
//
// A previous revision of this diagnostic ran the same history through two
// freshly built engines and asserted they agreed. An independent reviewer
// correctly recorded that as NOT discriminating: a walk that perturbed the
// result would perturb both runs identically. That test proved reproducibility;
// it is retained below as exactly that, under an honest name, and the
// discriminating work is done by the two-path comparison and the oracle.

const DECAY_RATE: i64 = 7;
const DECAY_CADENCE: i64 = 3;

/// `None` = all-additive body. `Some(max)` = mixed body whose clamp has that
/// upper bound.
fn rn02_engine(clamp_max: Option<i64>) -> Engine {
    let mut f = standard();
    let mut emits = vec![emit("add", "state.stress", Update::Add(lit(10)))];
    if let Some(max) = clamp_max {
        emits.push(emit(
            "clamp",
            "state.stress",
            Update::Clamp { min: -WIDE, max },
        ));
    }
    let rules = vec![
        on_command(
            "rule.seed",
            "cmd.seed",
            vec![emit("s", "state.stress", Update::Assign(param(0)))],
        ),
        on_command("rule.body", "cmd.body", emits),
        with_schedule(
            work_rule(
                "rule.d",
                "work.d",
                vec![emit("d", "state.stress", decay(DECAY_RATE, DECAY_CADENCE))],
            ),
            reevaluate_after("sch", DECAY_CADENCE as u64, "work.d", "rule.d"),
        ),
    ];
    let g = f.genesis_with(
        budgets(50),
        rules,
        vec![baseline("state.stress", 0)],
        vec![initial("rule.d", &a(), DECAY_CADENCE as u64, "work.d")],
    );
    Engine::genesis(g).unwrap()
}

/// Seed 100 at t=1, accrue decay debt to t=19, then write the body at t=20.
/// Returns (value at t=19, value after the body).
fn rn02_run(clamp_max: Option<i64>) -> (Option<i64>, Option<i64>, Digest) {
    let mut e = rn02_engine(clamp_max);
    let r = drive(
        &mut e,
        &Request::Command(command_request(
            "c.seed", 1, 1, "cmd.seed", a(), vec![], vec![100],
        )),
    );
    assert_eq!(r.last().unwrap().outcome(), &Outcome::Completed);
    drive(&mut e, &advance(19));
    let mid = value(&e, "state.stress", &a());
    let r = drive(
        &mut e,
        &Request::Command(command_request(
            "c.body", 20, 2, "cmd.body", a(), vec![], vec![],
        )),
    );
    assert_eq!(r.last().unwrap().outcome(), &Outcome::Completed);
    (mid, value(&e, "state.stress", &a()), cells_digest(&e))
}

#[test]
fn d10_rn02_the_repeated_settlement_walk_has_no_canonical_effect() {
    // The oracle, computed from the declared semantics and not from the engine:
    // seeded 100 at t=1 with baseline 0; by t=19 that is 18 logical units, i.e.
    // 6 whole cadences of 3, so 6 * 7 = 42 of decay debt: 100 - 42 = 58.
    const EXPECTED_AT_19: i64 = 100 - (18 / DECAY_CADENCE) * DECAY_RATE; // 58
    // From t=19 to t=20 is less than one cadence, so no further debt is billed
    // and the body adds 10: 68.
    const EXPECTED_AFTER_BODY: i64 = EXPECTED_AT_19 + 10; // 68

    let (mid_add, add_only, _) = rn02_run(None);
    // A deliberately NON-BINDING clamp: it changes no value, but it makes the
    // body "mixed", which is what selects the settled path.
    let (mid_mixed, mixed, _) = rn02_run(Some(WIDE));

    assert_eq!(
        mid_add,
        Some(EXPECTED_AT_19),
        "D-10 check 1: the additive-path engine must hold the hand-computed \
         decayed value at t=19"
    );
    assert_eq!(
        mid_mixed,
        Some(EXPECTED_AT_19),
        "D-10 check 2: so must the mixed-path engine — the two runs start equal"
    );
    assert_eq!(
        add_only,
        Some(EXPECTED_AFTER_BODY),
        "D-10 check 3: the all-additive AddDelta path must commit the oracle value"
    );
    assert_eq!(
        mixed,
        Some(EXPECTED_AFTER_BODY),
        "D-10 check 4: THE DISCRIMINATOR — the mixed body takes the settled path, \
         which resolves the settled baseline twice. If that repeat billed decay \
         debt a second time the committed value would be below the oracle. It is \
         not: both paths commit {EXPECTED_AFTER_BODY}"
    );
    assert_eq!(
        add_only, mixed,
        "D-10 check 5: the two code paths agree exactly on identical state"
    );

    // POSITIVE CONTROL. Checks 1-5 would also pass if the clamp stage were
    // simply dropped — in which case body B would be the same all-additive body
    // as A and the comparison would prove nothing. A clamp that BINDS must
    // therefore change the result, and by exactly the amount the declared
    // semantics predict.
    const BINDING_MAX: i64 = 60;
    let (_, clamped, _) = rn02_run(Some(BINDING_MAX));
    assert_eq!(
        clamped,
        Some(BINDING_MAX),
        "D-10 check 6 (positive control): a binding clamp must take effect, proving \
         the transform stage is really evaluated and body B is genuinely the mixed \
         path rather than a silently-dropped no-op"
    );
    assert_ne!(
        clamped, add_only,
        "D-10 check 7 (positive control): and it must differ from the additive-only \
         result, or the clamp stage is not reaching the value at all"
    );
    println!(
        "D-10 PASS: additive path and settled path both commit {EXPECTED_AFTER_BODY} \
         against a hand-computed oracle, with {} of decay debt already billed once; \
         a binding clamp at {BINDING_MAX} does change the result, so the transform \
         stage is genuinely evaluated. RN02's repeated walk is read-only; it is \
         recorded as a disclosed cost limitation and is not optimized.",
        100 - EXPECTED_AT_19
    );
}

#[test]
fn d10b_rn02_the_composed_result_is_also_reproducible_run_to_run() {
    // This is a DETERMINISM check, not the RN02 discriminator. It is kept
    // because reproducibility is worth asserting, and named for what it proves.
    let (_, first, d1) = rn02_run(Some(WIDE));
    let (_, second, d2) = rn02_run(Some(WIDE));
    assert_eq!(first, second, "D-10b check 1: same value across runs");
    assert_eq!(d1, d2, "D-10b check 2: same canonical state digest across runs");
    println!("D-10b PASS: run-to-run reproducibility only — this does not discriminate RN02");
}

// ==================================================================== D-11
// H-OBS-01 (new observation, test-seam only): `snapshot_stage_raw` and
// `stage_raw` in `spark_engine::fixture` report success as `Result::is_ok()`.
// `TimelineIngress::stage` returns `Ok(StageDisposition::NotInAdmissionWindow)`
// for an ordinal outside the current admission window — a correct, retryable
// refusal that consumes no capacity. The seam therefore returns `true` for an
// envelope it did *not* stage. This is a reporting imprecision in a
// `test-support` seam, not in production code; it is recorded because a probe
// that trusts that boolean can believe it mutated a snapshot when it did not,
// and would then report a false "restore accepted a tampered snapshot".

#[test]
fn d11_obs_test_seam_reports_true_for_an_out_of_window_stage() {
    let mut f = standard();
    let rules = vec![on_command(
        "rule.set",
        "cmd.set",
        vec![emit("s", "state.stress", Update::Assign(param(0)))],
    )];
    let g = f.genesis(budgets(50), rules, vec![]);
    let mut e = Engine::genesis(g).unwrap();
    drive(&mut e, &Request::Command(command_request(
        "cmd.1", 5, 1, "cmd.set", a(), vec![], vec![3],
    )));

    // In window: really stages, and restore refuses (proved in D-8).
    let inw = e.timeline_frontier_ordinal();
    // Far outside the window: the seam still answers `true`.
    let mut snap = e.snapshot().expect("D-11a: snapshot");
    let out = spark_core::timeline::Ordinal(inw.0.saturating_add(1_000));
    let envelope = command_request("cmd.far", 9, 2, "cmd.set", a(), vec![], vec![4])
        .envelope_at(out);
    let reported = snapshot_stage_raw(&mut snap, envelope);
    assert!(
        reported,
        "D-11b: the seam reports true for an out-of-window ordinal"
    );
    snapshot_reseal(&mut snap);
    let restored = Engine::restore(snap, &f.profile);
    assert!(
        restored.is_ok(),
        "D-11c: nothing was actually staged, so restore correctly succeeds"
    );
    println!(
        "D-11 OBSERVATION: fixture::snapshot_stage_raw returned true at ordinal {out:?} \
         (window starts at {inw:?}) while staging nothing — is_ok() covers the retryable \
         NotInAdmissionWindow disposition. Test-seam reporting only; production unaffected."
    );
}
