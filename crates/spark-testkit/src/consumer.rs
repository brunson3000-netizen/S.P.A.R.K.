//! A reference bounded FIFO consumer implementing the Revision-2 consumer
//! contract (§5.2–§5.5). Test harness only — Phase 2 authorizes no production
//! mailbox or transport implementation; the engine owns safety, this is the
//! liveness side used by the AT-I49 consumer-level tests.

use spark_engine::engine::Engine;
use spark_engine::request::{Outcome, ProcessResult, Request};
use std::collections::VecDeque;

/// Why the consumer stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsumerStop {
    /// The mailbox is empty.
    Drained,
    /// `CONSUMER_HALTED_ACTIVE_REQUEST_NOT_AT_HEAD` (§5.3): the engine's
    /// `ActiveRequest` is not the head; nothing presented, nothing removed.
    HaltedActiveRequestNotAtHead,
    /// The engine fail-stopped.
    FailStopped,
    /// The step bound was reached.
    StepBound,
}

#[derive(Debug, Clone, Default)]
pub struct Mailbox {
    capacity: usize,
    queue: VecDeque<Request>,
}

impl Mailbox {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: VecDeque::new(),
        }
    }

    /// Backpressure is refusal at the producer; nothing reaches the engine.
    pub fn offer(&mut self, request: Request) -> bool {
        if self.queue.len() >= self.capacity {
            return false;
        }
        self.queue.push_back(request);
        true
    }

    pub fn head(&self) -> Option<&Request> {
        self.queue.front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn contents(&self) -> Vec<Request> {
        self.queue.iter().cloned().collect()
    }

    /// Removes the head **only** if `result` terminates exactly that head
    /// (request-bound dequeue eligibility, §5.2). Returns whether it removed.
    pub fn complete_if_terminated(&mut self, result: &ProcessResult) -> bool {
        let terminates = match self.queue.front() {
            Some(head) => result.terminates(&head.discriminator()),
            None => false,
        };
        if terminates {
            self.queue.pop_front();
        }
        terminates
    }
}

/// Drives the engine from the mailbox: presents only the head, repeats while
/// paused, removes the head only on a result terminating it, and halts
/// fail-closed if the engine's `ActiveRequest` is not the head.
pub fn drive(
    engine: &mut Engine,
    mailbox: &mut Mailbox,
    max_steps: usize,
) -> (Vec<ProcessResult>, ConsumerStop) {
    let mut results = Vec::new();
    for _ in 0..max_steps {
        let Some(head) = mailbox.head().cloned() else {
            return (results, ConsumerStop::Drained);
        };
        if let Some(active) = engine.active_request() {
            if active != &head.discriminator() {
                return (results, ConsumerStop::HaltedActiveRequestNotAtHead);
            }
        }
        let result = engine.process(&head);
        let outcome = result.outcome().clone();
        mailbox.complete_if_terminated(&result);
        results.push(result);
        match outcome {
            Outcome::RefusedActiveRequestMismatch { .. } => {
                return (results, ConsumerStop::HaltedActiveRequestNotAtHead)
            }
            Outcome::FinalizationEntailmentViolated => return (results, ConsumerStop::FailStopped),
            _ => {}
        }
    }
    (results, ConsumerStop::StepBound)
}
