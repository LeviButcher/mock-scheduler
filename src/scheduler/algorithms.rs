use crate::QueuedProcess;
use std::cmp;

pub fn first_come(a: &QueuedProcess, b: &QueuedProcess) -> cmp::Ordering {
    a.entered_number.cmp(&b.entered_number)
}

pub fn shortest_next(a: &QueuedProcess, b: &QueuedProcess) -> cmp::Ordering {
    a.process
        .quantum_to_run_for
        .cmp(&b.process.quantum_to_run_for)
}

pub fn shortest_remain(a: &QueuedProcess, b: &QueuedProcess) -> cmp::Ordering {
    a.process.quantum_left.cmp(&b.process.quantum_left)
}
