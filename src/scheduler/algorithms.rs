use super::FakeProcess;
use std::cmp;

pub fn first_come(a: &FakeProcess, b: &FakeProcess) -> cmp::Ordering {
    a.id.cmp(&b.id)
}

pub fn shortest_next(a: &FakeProcess, b: &FakeProcess) -> cmp::Ordering {
    a.quantum_to_run_for.cmp(&b.quantum_to_run_for)
}

pub fn shortest_remain(a: &FakeProcess, b: &FakeProcess) -> cmp::Ordering {
    a.quantum_left.cmp(&b.quantum_left)
}
