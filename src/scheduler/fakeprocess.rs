use std::fmt;

#[derive(Clone)]
pub struct FakeProcess {
    pub id: u32,
    pub quantum_left: u32,
    pub quantum_ran_for: u32,
    pub total_time: u32,
    pub quantum_to_run_for: u32,
    pub time_spent_waiting: u32,
}

fn subtract_until_zero(a: u32, b: u32) -> u32 {
    if a < b {
        0
    } else {
        a - b
    }
}

impl FakeProcess {
    pub fn new(id: u32, quantum_run_time: u32) -> FakeProcess {
        FakeProcess {
            id,
            quantum_left: quantum_run_time,
            quantum_ran_for: 0,
            total_time: 0,
            quantum_to_run_for: quantum_run_time,
            time_spent_waiting: 0,
        }
    }
    pub fn execute(&self, quantum_run: u32) -> FakeProcess {
        FakeProcess {
            id: self.id,
            quantum_left: subtract_until_zero(self.quantum_left, quantum_run),
            quantum_ran_for: self.quantum_ran_for + quantum_run,
            total_time: self.total_time + quantum_run,
            quantum_to_run_for: self.quantum_to_run_for,
            time_spent_waiting: self.time_spent_waiting,
        }
    }
    pub fn wait(&self, quantum_wait: u32) -> FakeProcess {
        FakeProcess {
            id: self.id,
            quantum_left: self.quantum_left,
            quantum_ran_for: self.quantum_ran_for,
            total_time: self.total_time + quantum_wait,
            quantum_to_run_for: self.quantum_to_run_for,
            time_spent_waiting: self.time_spent_waiting + quantum_wait,
        }
    }
    pub fn is_done(&self) -> bool {
        self.quantum_left <= 0
    }
}

impl fmt::Display for FakeProcess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| id: {} | q-left:{} | q-ran-for: {} | total: {} |",
            self.id, self.quantum_left, self.quantum_ran_for, self.total_time
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_execute_quantum_left_lower_than_quantum_used_should_not_buffer_overflow() {
        let process = FakeProcess::new(0, 5).execute(10);
        assert_eq!(process.quantum_left, 0);
    }
}
