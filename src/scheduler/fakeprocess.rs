use std::fmt;

#[derive(Copy, Clone)]
pub struct FakeProcess {
    pub id: u32,
    pub quantum_left: u32,
    pub quantum_ran_for: u32,
    pub quantum_to_run_for: u32,
    pub time_spent_waiting: u32,
    pub priority: u32,
}

impl FakeProcess {
    pub fn new(id: u32, quantum_run_time: u32, priority: u32) -> FakeProcess {
        FakeProcess {
            id,
            quantum_left: quantum_run_time,
            quantum_ran_for: 0,
            quantum_to_run_for: quantum_run_time,
            time_spent_waiting: 0,
            priority,
        }
    }
    pub fn execute(&self, quantum_run: u32) -> (FakeProcess, u32) {
        let could_run_for = quantum_run * self.priority;
        let ran_for = if self.quantum_left > could_run_for {
            could_run_for
        } else {
            self.quantum_left
        };

        (
            FakeProcess {
                id: self.id,
                quantum_left: self.quantum_left - ran_for,
                quantum_ran_for: self.quantum_ran_for + ran_for,
                quantum_to_run_for: self.quantum_to_run_for,
                time_spent_waiting: self.time_spent_waiting,
                priority: self.priority,
            },
            ran_for,
        )
    }
    pub fn wait(&self, quantum_wait: u32) -> FakeProcess {
        FakeProcess {
            id: self.id,
            quantum_left: self.quantum_left,
            quantum_ran_for: self.quantum_ran_for,
            quantum_to_run_for: self.quantum_to_run_for,
            time_spent_waiting: self.time_spent_waiting + quantum_wait,
            priority: self.priority,
        }
    }
    pub fn is_done(&self) -> bool {
        self.quantum_left <= 0
    }

    pub fn get_turnaround_time(&self) -> u32 {
        self.time_spent_waiting + self.quantum_ran_for
    }
}

impl fmt::Display for FakeProcess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| id: {} | priority: {} | q-left:{} | q-ran-for: {} | turnaround: {} | wait: {} |",
            self.id,
            self.priority,
            self.quantum_left,
            self.quantum_ran_for,
            self.get_turnaround_time(),
            self.time_spent_waiting
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_execute_quantum_left_lower_than_quantum_used_should_not_buffer_overflow() {
        let process = FakeProcess::new(0, 5, 1).execute(10);
        assert_eq!(process.0.quantum_left, 0);
    }

    #[test]
    fn process_get_turnaround_should_return_expected() {
        let expected_result = 15;
        let proc = FakeProcess::new(1, 20, 1).execute(7).0.wait(8);
        let result = proc.get_turnaround_time();
        assert_eq!(expected_result, result);
    }

    #[test]
    fn process_execute_run_for_quantum_larger_then_left_should_return_new_process_with_exact_ran_value(
    ) {
        let expected_result = 5;
        let process = FakeProcess::new(1, 5, 1);
        let result = process.execute(6);
        assert_eq!(expected_result, result.1);
        assert_eq!(0, result.0.quantum_left);
    }

    #[test]
    fn process_execute_should_run_for_quantum_times_the_priority() {
        let expected_result = 6;
        let process = FakeProcess::new(1, 20, 2);
        let (proc, ran_for) = process.execute(3);
        assert_eq!(expected_result, ran_for);
        assert_eq!(20 - (ran_for), proc.quantum_left);
    }
}
