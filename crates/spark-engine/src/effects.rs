//! The corrected effect model (freeze v2 §3–§5, v3 §4): stable causal emission
//! identity, candidate canonicalization, the typed same-target reducer, batch
//! identity, bounded provenance, and the frozen numeric model (v1 Q1: `i64`
//! and `FixedPoint` with `i128` intermediates, one floor-rounding rule).

use crate::report::{
    CoverageStatus, Provenance, WaveRejection, WritePath, PROVENANCE_PRUNING_POLICY,
};
use crate::rules::{ResultFamily, TransformFamily};
use crate::state::MAX_SOURCE_REFS;
use spark_core::clock::LogicalTime;
use spark_core::hash::{CanonicalEncoder, Digest};
use spark_core::id::{CanonicalTag, DefinitionId, ProfileId};
use spark_core::scheduler::WorkKey;
use spark_core::scope::ScopeId;
use spark_core::value::FIXED_SCALE;
use std::collections::{BTreeMap, BTreeSet};

// ---------------------------------------------------------------- identities

/// `H("scheduled_cohort_v1" ‖ profile ‖ due_time ‖ count ‖ ascending WorkKey
/// identity digests, each length-prefixed)` over the **executable** keys (v3
/// §4.1 as settled by FINAL §12.1).
pub fn scheduled_cohort_identity(
    profile: &ProfileId,
    due_time: LogicalTime,
    executable: &[WorkKey],
) -> Digest {
    let mut ids: Vec<Digest> = executable.iter().map(WorkKey::identity_digest).collect();
    ids.sort();
    let mut enc = CanonicalEncoder::new();
    enc.push_str("scheduled_cohort_v1");
    profile.canonicalize(&mut enc);
    due_time.canonicalize(&mut enc);
    enc.push_u64(ids.len() as u64);
    for id in &ids {
        enc.push_bytes(id.as_bytes());
    }
    enc.finish()
}

/// The finalized command barrier identity (ADR-0003 §14; v3 §4.2).
pub fn command_cohort_identity(
    timeline_epoch: u64,
    input_ordinal: u64,
    semantic_hash: &Digest,
    fence_hash: &Digest,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("command_cohort_v1");
    enc.push_u64(timeline_epoch);
    enc.push_u64(input_ordinal);
    enc.push_digest(semantic_hash);
    enc.push_digest(fence_hash);
    enc.finish()
}

/// Domain-separated parent contexts (v3 §4.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParentContext {
    WorkKey(Digest),
    Command(Digest),
    Emissions(BTreeSet<Digest>),
}

impl ParentContext {
    pub(crate) fn digest(&self) -> Digest {
        let mut enc = CanonicalEncoder::new();
        match self {
            ParentContext::WorkKey(id) => {
                enc.push_str("parents_workkey_v1");
                enc.push_digest(id);
            }
            ParentContext::Command(id) => {
                enc.push_str("parents_command_v1");
                enc.push_digest(id);
            }
            ParentContext::Emissions(parents) => {
                enc.push_str("parents_emission_v1");
                enc.push_u64(parents.len() as u64);
                for p in parents {
                    enc.push_bytes(p.as_bytes());
                }
            }
        }
        enc.finish()
    }
}

/// The inputs of one emission identity (v3 §4.4).
pub(crate) struct EmissionContext<'a> {
    pub cohort_identity: &'a Digest,
    pub wave_index: u32,
    pub parent_context: &'a Digest,
    pub rule_fingerprint: &'a Digest,
    pub producer_scope: &'a ScopeId,
    pub behavior_artifact_hash: &'a Digest,
}

/// `H("emission" ‖ cohort ‖ wave ‖ parent context ‖ rule fingerprint ‖
/// operation sub-ID(s) ‖ producer scope ‖ target ‖ behavior artifact)`.
/// Identity never includes the numeric payload.
pub(crate) fn emission_identity(
    ctx: &EmissionContext<'_>,
    sub_ids: &[&CanonicalTag],
    target_definition: &DefinitionId,
    target_scope: &ScopeId,
) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("emission");
    enc.push_digest(ctx.cohort_identity);
    enc.push_u32(ctx.wave_index);
    enc.push_digest(ctx.parent_context);
    enc.push_digest(ctx.rule_fingerprint);
    enc.push_u64(sub_ids.len() as u64);
    for s in sub_ids {
        s.canonicalize(&mut enc);
    }
    ctx.producer_scope.canonicalize(&mut enc);
    target_definition.canonicalize(&mut enc);
    target_scope.canonicalize(&mut enc);
    enc.push_digest(ctx.behavior_artifact_hash);
    enc.finish()
}

// ---------------------------------------------------------------- candidates

/// A candidate's update intent (v2 §3.1). Payload is never in identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Intent {
    AddDelta(i64),
    Result {
        value: i64,
        family: ResultFamily,
    },
    Transform {
        family: TransformFamily,
        op_fingerprint: Digest,
        /// The resolved value, committed at the cohort's canonical time like
        /// every effect (FINAL §4). Decay/recovery keeps its cadence
        /// remainder on the operation's parameter-segment grid, not in a
        /// backdated commit time (C2R-01).
        resolved: i64,
    },
}

impl Intent {
    fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        match self {
            Intent::AddDelta(d) => {
                enc.push_str("intent.add_delta");
                enc.push_i64(*d);
            }
            Intent::Result { value, family } => {
                enc.push_str("intent.result");
                enc.push_str(family.tag());
                enc.push_i64(*value);
            }
            Intent::Transform {
                family,
                op_fingerprint,
                resolved,
            } => {
                enc.push_str("intent.transform");
                enc.push_str(family.tag());
                enc.push_digest(op_fingerprint);
                enc.push_i64(*resolved);
            }
        }
    }
}

/// One candidate cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub definition: DefinitionId,
    pub scope: ScopeId,
    pub write_path: WritePath,
    pub identity: Digest,
    pub intent: Intent,
}

/// Emission-identity canonicalization (v2 §3.2): exact duplicates fold; one
/// identity with distinct payloads is contested and rejects the wave. The
/// reported identity is the least contested one, so evidence is independent
/// of enumeration order.
pub(crate) fn canonicalize_candidates(
    candidates: Vec<Candidate>,
) -> Result<BTreeMap<Digest, Candidate>, WaveRejection> {
    let mut canonical: BTreeMap<Digest, Candidate> = BTreeMap::new();
    let mut contested: BTreeSet<Digest> = BTreeSet::new();
    for c in candidates {
        match canonical.get(&c.identity) {
            Some(existing) if existing != &c => {
                contested.insert(c.identity.clone());
            }
            Some(_) => {}
            None => {
                canonical.insert(c.identity.clone(), c);
            }
        }
    }
    match contested.into_iter().next() {
        Some(identity) => Err(WaveRejection::ContestedEmission { identity }),
        None => Ok(canonical),
    }
}

/// `H("candidate_set_v1")` over the canonical multiset in ascending identity.
pub(crate) fn candidate_set_digest(canonical: &BTreeMap<Digest, Candidate>) -> Digest {
    let mut enc = CanonicalEncoder::new();
    enc.push_str("candidate_set_v1");
    enc.push_u64(canonical.len() as u64);
    for c in canonical.values() {
        let mut inner = CanonicalEncoder::new();
        inner.push_digest(&c.identity);
        c.definition.canonicalize(&mut inner);
        c.scope.canonicalize(&mut inner);
        inner.push_str(c.write_path.tag());
        c.intent.canonicalize(&mut inner);
        enc.push_block(&inner);
    }
    enc.finish()
}

/// One reduced target: the single resulting raw value plus every contributing
/// emission identity (ascending).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Reduced {
    pub definition: DefinitionId,
    pub scope: ScopeId,
    pub write_path: WritePath,
    pub raw: i64,
    pub contributing: Vec<Digest>,
}

/// The typed target-local reducer (v2 §4.2). Groups are reduced in canonical
/// `(definition, scope, write-path)` order; the first failing group in that
/// order is the reported rejection, so evidence cannot depend on enumeration.
/// `pre` returns the committed pre-wave numeric value (`None` if absent).
pub(crate) fn reduce(
    canonical: &BTreeMap<Digest, Candidate>,
    pre: &dyn Fn(&DefinitionId, &ScopeId) -> Option<i64>,
) -> Result<Vec<Reduced>, WaveRejection> {
    let mut groups: BTreeMap<(DefinitionId, ScopeId, WritePath), Vec<&Candidate>> = BTreeMap::new();
    for c in canonical.values() {
        groups
            .entry((c.definition.clone(), c.scope.clone(), c.write_path))
            .or_default()
            .push(c);
    }
    let mut out = Vec::new();
    for ((definition, scope, write_path), members) in groups {
        let contributing: Vec<Digest> = members.iter().map(|c| c.identity.clone()).collect();
        let additive: Vec<i64> = members
            .iter()
            .filter_map(|c| match c.intent {
                Intent::AddDelta(d) => Some(d),
                _ => None,
            })
            .collect();
        let results: Vec<(i64, ResultFamily)> = members
            .iter()
            .filter_map(|c| match c.intent {
                Intent::Result { value, family } => Some((value, family)),
                _ => None,
            })
            .collect();
        let transforms: Vec<(TransformFamily, i64)> = members
            .iter()
            .filter_map(|c| match c.intent {
                Intent::Transform {
                    family, resolved, ..
                } => Some((family, resolved)),
                _ => None,
            })
            .collect();
        let raw = if additive.len() == members.len() {
            // ADDITIVE: checked i128 fold, applied once to the pre-wave value.
            let mut total: i128 = 0;
            for d in &additive {
                total = total
                    .checked_add(i128::from(*d))
                    .ok_or(WaveRejection::Arithmetic {
                        what: "additive_fold",
                    })?;
            }
            let base = i128::from(pre(&definition, &scope).unwrap_or(0));
            let sum = base.checked_add(total).ok_or(WaveRejection::Arithmetic {
                what: "additive_apply",
            })?;
            i64::try_from(sum).map_err(|_| WaveRejection::Arithmetic {
                what: "additive_domain",
            })?
        } else if results.len() == members.len() {
            let families: BTreeSet<ResultFamily> = results.iter().map(|(_, f)| *f).collect();
            let values: BTreeSet<i64> = results.iter().map(|(v, _)| *v).collect();
            match (families.iter().next(), values.iter().next()) {
                (Some(family), Some(value)) if families.len() == 1 => {
                    if values.len() == 1 {
                        *value
                    } else {
                        return Err(WaveRejection::ResultConflict {
                            definition,
                            scope,
                            family: *family,
                            values,
                        });
                    }
                }
                _ => return Err(WaveRejection::FamilyMixture { definition, scope }),
            }
        } else if transforms.len() == members.len() {
            match transforms.as_slice() {
                [(_, resolved)] => *resolved,
                _ => {
                    return Err(WaveRejection::MultipleTransforms {
                        definition,
                        scope,
                        families: transforms.iter().map(|(f, _)| *f).collect(),
                    })
                }
            }
        } else {
            return Err(WaveRejection::FamilyMixture { definition, scope });
        };
        out.push(Reduced {
            definition,
            scope,
            write_path,
            raw,
            contributing,
        });
    }
    Ok(out)
}

/// Bounded provenance under `provenance_prune.v1` (v2 §10).
pub(crate) fn provenance_of(contributing: &[Digest]) -> Provenance {
    let mut sorted: Vec<Digest> = contributing.to_vec();
    sorted.sort();
    sorted.dedup();
    let total = sorted.len();
    let retained: Vec<Digest> = sorted.into_iter().take(MAX_SOURCE_REFS).collect();
    let omitted = total.saturating_sub(retained.len()) as u64;
    Provenance {
        retained,
        omitted_source_count: omitted,
        coverage_status: if omitted == 0 {
            CoverageStatus::Complete
        } else {
            CoverageStatus::Truncated
        },
        pruning_policy: PROVENANCE_PRUNING_POLICY,
    }
}

// ---------------------------------------------------------------- numeric model

/// Floor division (toward negative infinity) with a positive divisor.
pub(crate) fn floor_div(n: i128, d: i128) -> Option<i128> {
    if d <= 0 {
        return None;
    }
    n.checked_div_euclid(d)
}

pub(crate) fn to_i64(v: i128, what: &'static str) -> Result<i64, WaveRejection> {
    i64::try_from(v).map_err(|_| WaveRejection::Arithmetic { what })
}

/// `floor(v · factor_raw / FIXED_SCALE)`.
pub(crate) fn scale(v: i64, factor_raw: i64) -> Result<i64, WaveRejection> {
    to_i64(scale_wide(i128::from(v), factor_raw)?, "scale")
}

/// `floor(v · factor_raw / FIXED_SCALE)` over a widened intermediate: the
/// result stays `i128` for a later stage (v1 Q1).
pub(crate) fn scale_wide(v: i128, factor_raw: i64) -> Result<i128, WaveRejection> {
    let p = v
        .checked_mul(i128::from(factor_raw))
        .ok_or(WaveRejection::Arithmetic { what: "scale" })?;
    floor_div(p, i128::from(FIXED_SCALE)).ok_or(WaveRejection::Arithmetic { what: "scale" })
}

/// Aggregation (v1 Q1/§7, AT-I24): `floor(Σ values · weight_raw /
/// FIXED_SCALE)` with the accumulation **and** the weighting in checked `i128`
/// and exactly one floor; the caller converts the result once.
pub(crate) fn weighted_aggregate(
    values: impl Iterator<Item = i64>,
    weight_raw: i64,
) -> Result<i128, WaveRejection> {
    let mut acc: i128 = 0;
    for v in values {
        acc = acc
            .checked_add(i128::from(v))
            .ok_or(WaveRejection::Arithmetic { what: "aggregate" })?;
    }
    let weighted = acc
        .checked_mul(i128::from(weight_raw))
        .ok_or(WaveRejection::Arithmetic { what: "aggregate" })?;
    floor_div(weighted, i128::from(FIXED_SCALE))
        .ok_or(WaveRejection::Arithmetic { what: "aggregate" })
}

/// `floor(Σ wᵢ·xᵢ / FIXED_SCALE)` with one floor at the end.
pub(crate) fn weighted_sum(terms: &[(i64, i64)]) -> Result<i64, WaveRejection> {
    let mut acc: i128 = 0;
    for (w, x) in terms {
        let p = i128::from(*w)
            .checked_mul(i128::from(*x))
            .ok_or(WaveRejection::Arithmetic {
                what: "weighted_sum",
            })?;
        acc = acc.checked_add(p).ok_or(WaveRejection::Arithmetic {
            what: "weighted_sum",
        })?;
    }
    let q = floor_div(acc, i128::from(FIXED_SCALE)).ok_or(WaveRejection::Arithmetic {
        what: "weighted_sum",
    })?;
    to_i64(q, "weighted_sum")
}

/// Piecewise-linear interpolation over strictly increasing x (validated at
/// activation), clamped to the end segments, floor rounding.
pub(crate) fn curve(x: i64, points: &[(i64, i64)]) -> Result<i64, WaveRejection> {
    let (first, last) = match (points.first(), points.last()) {
        (Some(f), Some(l)) => (*f, *l),
        _ => return Err(WaveRejection::Arithmetic { what: "curve" }),
    };
    if x <= first.0 {
        return Ok(first.1);
    }
    if x >= last.0 {
        return Ok(last.1);
    }
    for w in points.windows(2) {
        if let [(x0, y0), (x1, y1)] = w {
            if x >= *x0 && x < *x1 {
                let dx = i128::from(x)
                    .checked_sub(i128::from(*x0))
                    .ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                let dy = i128::from(*y1)
                    .checked_sub(i128::from(*y0))
                    .ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                let span = i128::from(*x1)
                    .checked_sub(i128::from(*x0))
                    .ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                let num = dx
                    .checked_mul(dy)
                    .ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                let step =
                    floor_div(num, span).ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                let y = i128::from(*y0)
                    .checked_add(step)
                    .ok_or(WaveRejection::Arithmetic { what: "curve" })?;
                return to_i64(y, "curve");
            }
        }
    }
    Err(WaveRejection::Arithmetic { what: "curve" })
}

/// One closed-form decay/recovery segment (v1 Q7): move `value` toward
/// `baseline` by `rate` per whole step, `steps` whole steps, linear and never
/// overshooting the baseline, in checked `i128`. Composing segments is exact:
/// `move(move(v, a), b) == move(v, a + b)` for one baseline, which is the
/// chunk invariance the frozen formula promises.
pub(crate) fn move_toward(
    value: i128,
    baseline: i128,
    steps: u64,
    rate: i64,
) -> Result<i128, WaveRejection> {
    let err = || WaveRejection::Arithmetic { what: "decay" };
    let movement = i128::from(steps)
        .checked_mul(i128::from(rate.max(0)))
        .ok_or_else(err)?;
    let distance = value
        .checked_sub(baseline)
        .and_then(i128::checked_abs)
        .ok_or_else(err)?;
    let moved = distance.min(movement);
    if value > baseline {
        value.checked_sub(moved)
    } else {
        value.checked_add(moved)
    }
    .ok_or_else(err)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use spark_core::hash::hash_bytes;
    use spark_core::scope::ScopeKind;

    fn cand(id: &str, intent: Intent) -> Candidate {
        Candidate {
            definition: DefinitionId::new("state.stress").unwrap(),
            scope: ScopeId::new(ScopeKind::Actor, "bron").unwrap(),
            write_path: WritePath::SparkEffect,
            identity: hash_bytes(id.as_bytes()),
            intent,
        }
    }

    fn run(cands: Vec<Candidate>) -> Result<Vec<Reduced>, WaveRejection> {
        let canonical = canonicalize_candidates(cands)?;
        reduce(&canonical, &|_, _| Some(20))
    }

    /// v2 §4.4 mandated examples, every permutation of each pair.
    #[test]
    fn mandated_examples_resolve_as_frozen_in_every_permutation() {
        let add = |id, d| cand(id, Intent::AddDelta(d));
        let res = |id, v| {
            cand(
                id,
                Intent::Result {
                    value: v,
                    family: ResultFamily::Assignment,
                },
            )
        };
        let scale = |id| {
            cand(
                id,
                Intent::Transform {
                    family: TransformFamily::Scale,
                    op_fingerprint: hash_bytes(b"scale"),
                    resolved: 30,
                },
            )
        };
        for (pair, expected) in [
            (vec![add("a", 10), add("b", 15)], Some(45)),
            (vec![add("a", 10), add("b", 10)], Some(40)),
            (vec![add("a", 10), add("a", 10)], Some(30)),
            (vec![res("a", 30), res("b", 30)], Some(30)),
            (vec![res("a", 30), res("b", 35)], None),
            (vec![res("a", 30), add("b", 10)], None),
            (vec![scale("a"), scale("b")], None),
        ] {
            let forward = run(pair.clone());
            let mut rev = pair.clone();
            rev.reverse();
            let reversed = run(rev);
            assert_eq!(forward, reversed);
            assert_eq!(forward.ok().map(|r| r[0].raw), expected);
        }
        // One identity, two payloads: contested.
        assert!(matches!(
            run(vec![add("a", 10), add("a", 15)]),
            Err(WaveRejection::ContestedEmission { .. })
        ));
    }

    #[test]
    fn numeric_model_floors_toward_negative_infinity() {
        assert_eq!(scale(3, 1_500_000).unwrap(), 4);
        assert_eq!(scale(-3, 1_500_000).unwrap(), -5);
        assert_eq!(weighted_sum(&[(500_000, 3), (500_000, 4)]).unwrap(), 3);
        assert_eq!(weighted_sum(&[(500_000, -3)]).unwrap(), -2);
        assert_eq!(curve(5, &[(0, 0), (10, 3)]).unwrap(), 1);
        assert_eq!(curve(-5, &[(0, 0), (10, 3)]).unwrap(), 0);
        assert_eq!(curve(50, &[(0, 0), (10, 3)]).unwrap(), 3);
        assert_eq!(move_toward(100, 20, 3, 10).unwrap(), 70);
        assert_eq!(move_toward(25, 20, 3, 10).unwrap(), 20, "never overshoots");
        assert_eq!(
            move_toward(10, 20, 2, 3).unwrap(),
            16,
            "recovery from below"
        );
        assert_eq!(move_toward(100, 20, 0, 10).unwrap(), 100);
        // Segment composition is exact (chunk invariance of the frozen formula).
        let once = move_toward(100, 20, 7, 10).unwrap();
        let split = move_toward(move_toward(100, 20, 3, 10).unwrap(), 20, 4, 10).unwrap();
        assert_eq!(once, split);
        assert!(scale(i64::MAX, 2_000_000).is_err());
        // Widened aggregate: floor((MAX + MAX) · 0.5) = MAX, no premature narrowing.
        assert_eq!(
            weighted_aggregate([i64::MAX, i64::MAX].into_iter(), 500_000).unwrap(),
            i128::from(i64::MAX)
        );
        assert_eq!(
            weighted_aggregate([-3].into_iter(), 500_000).unwrap(),
            -2,
            "one floor toward negative infinity"
        );
    }
}
