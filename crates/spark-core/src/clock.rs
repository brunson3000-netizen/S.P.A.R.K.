//! Monotonic logical clock.
//!
//! Wall-clock time is never canonical (`CONTROLLING_BLUEPRINT_v0.2.md`
//! §28 item 5, §18.1). The host advances a monotonic integer simulation
//! clock and S.P.A.R.K. evaluates only work that is due against that
//! logical time. [`LogicalClock`] enforces monotonicity mechanically so a
//! backward jump - which would make canonical evaluation order
//! ambiguous - is rejected rather than silently accepted.

use crate::hash::CanonicalEncoder;

/// A monotonic integer simulation time, in host-defined logical units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicalTime(pub u64);

impl LogicalTime {
    pub const ZERO: LogicalTime = LogicalTime(0);

    pub fn canonicalize(&self, enc: &mut CanonicalEncoder) {
        enc.push_u64(self.0);
    }
}

/// Rejects an attempt to move the logical clock backward.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackwardClockAdvance {
    pub current: LogicalTime,
    pub attempted: LogicalTime,
}

impl std::fmt::Display for BackwardClockAdvance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "logical clock cannot move backward: current={}, attempted={}",
            self.current.0, self.attempted.0
        )
    }
}

impl std::error::Error for BackwardClockAdvance {}

/// A monotonic logical clock. `advance_to` accepts the current time
/// (idempotent no-op) or any strictly later time; it rejects anything
/// earlier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalClock {
    current: LogicalTime,
}

impl LogicalClock {
    pub fn new(start: LogicalTime) -> Self {
        Self { current: start }
    }

    pub fn now(&self) -> LogicalTime {
        self.current
    }

    pub fn advance_to(&mut self, new_time: LogicalTime) -> Result<(), BackwardClockAdvance> {
        if new_time < self.current {
            return Err(BackwardClockAdvance {
                current: self.current,
                attempted: new_time,
            });
        }
        self.current = new_time;
        Ok(())
    }
}

impl Default for LogicalClock {
    fn default() -> Self {
        Self::new(LogicalTime::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advances_forward() {
        let mut clock = LogicalClock::default();
        clock.advance_to(LogicalTime(10)).unwrap();
        assert_eq!(clock.now(), LogicalTime(10));
    }

    #[test]
    fn same_time_is_idempotent() {
        let mut clock = LogicalClock::new(LogicalTime(5));
        assert!(clock.advance_to(LogicalTime(5)).is_ok());
        assert_eq!(clock.now(), LogicalTime(5));
    }

    #[test]
    fn rejects_backward_advancement() {
        let mut clock = LogicalClock::new(LogicalTime(10));
        let err = clock.advance_to(LogicalTime(9)).unwrap_err();
        assert_eq!(
            err,
            BackwardClockAdvance {
                current: LogicalTime(10),
                attempted: LogicalTime(9)
            }
        );
        assert_eq!(clock.now(), LogicalTime(10));
    }
}
