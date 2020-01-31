// Algorithm THOUGHTS
// Every cpu cycle (aka while loop)
// call execute on scheduler -> decrement selected process by CPU_CYCLE_QUANTUM_USED

use std::fmt;

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub quantum_left: u32,
    pub quantum_ran_for: u32,
    pub total_time: u32,
}

fn subtract_until_zero(a: u32, b: u32) -> u32 {
    if a < b {
        0
    } else {
        a - b
    }
}

impl Process {
    pub fn new(id: u32, quantum_run_time: u32) -> Process {
        Process {
            id,
            quantum_left: quantum_run_time,
            quantum_ran_for: 0,
            total_time: 0,
        }
    }
    pub fn execute(&self, quantum_run: u32) -> Process {
        Process {
            id: self.id,
            quantum_left: subtract_until_zero(self.quantum_left, quantum_run),
            quantum_ran_for: self.quantum_ran_for + quantum_run,
            total_time: self.total_time + quantum_run,
        }
    }
    pub fn wait(&self, quantum_wait: u32) -> Process {
        Process {
            id: self.id,
            quantum_left: self.quantum_left,
            quantum_ran_for: self.quantum_ran_for,
            total_time: self.total_time + quantum_wait,
        }
    }
    pub fn is_done(&self) -> bool {
        self.quantum_left <= 0
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| id: {} | q-left:{} | q-total: {} | total: {}",
            self.id, self.quantum_left, self.quantum_ran_for, self.total_time
        )
    }
}

pub trait Scheduler {
    fn new() -> Self;
    fn add_process(self, proc: Process) -> Self;
    fn execute(self, cpu_cycle_quantum_usage: u32) -> Self;
    fn schedule_next(self, context_switch_cost: u32) -> Self;
    fn scheduled_process(&self) -> Option<&Process>;
    fn print_process_table(&self);
}

pub struct FIFOScheduler {
    pub process_queue: Vec<Process>,
}

impl Scheduler for FIFOScheduler {
    fn new() -> FIFOScheduler {
        let process_queue: Vec<Process> = Vec::new();
        FIFOScheduler { process_queue }
    }
    fn add_process(mut self, proc: Process) -> FIFOScheduler {
        // Crying Functional Tears
        self.process_queue.push(proc);
        FIFOScheduler {
            process_queue: self.process_queue,
        }
    }
    fn execute(self, cpu_cycle_quantum_usage: u32) -> FIFOScheduler {
        match self.scheduled_process() {
            Some(p) => {
                let updated_proc = vec![p.execute(cpu_cycle_quantum_usage)];
                let queue = match self.process_queue.as_slice().split_first() {
                    Some((_, rest)) => rest,
                    None => &[],
                };
                let queue: Vec<Process> = queue
                    .to_vec()
                    .into_iter()
                    .map(|x| x.wait(cpu_cycle_quantum_usage))
                    .collect();

                FIFOScheduler {
                    process_queue: [&updated_proc[..], &queue[..]].concat(),
                }
            }
            None => self,
        }
    }
    fn schedule_next(self, context_switch_cost: u32) -> Self {
        let queue: Vec<Process> = self
            .process_queue
            .into_iter()
            .map(|x| x.wait(context_switch_cost))
            .collect();

        let (scheduled, rest) = match queue.as_slice().split_first() {
            Some((scheduled, queue)) => (scheduled, queue),
            None => {
                return FIFOScheduler {
                    process_queue: queue,
                }
            }
        };

        if scheduled.is_done() {
            FIFOScheduler {
                process_queue: rest.to_vec(),
            }
        } else {
            FIFOScheduler {
                process_queue: queue,
            }
        }
    }
    fn scheduled_process(&self) -> Option<&Process> {
        self.process_queue.first()
    }
    fn print_process_table(&self) {
        for p in &self.process_queue[..] {
            println!("{}", p);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fifo_scheduler_add_process_should_add_process() {
        let fifo_scheduler = FIFOScheduler::new();
        assert_eq!(fifo_scheduler.process_queue.len(), 0);
        let fifo_scheduler = fifo_scheduler.add_process(Process::new(0, 32));
        assert_eq!(fifo_scheduler.process_queue.len(), 1);
    }

    #[test]
    fn fifo_scheduler_add_process_should_add_process_to_back() {
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(Process::new(0, 32))
            .add_process(Process::new(1, 42));
        let last_process = fifo_scheduler.process_queue.last().unwrap();
        assert_eq!(last_process.id, 1);
    }

    #[test]
    fn fifo_scheduler_execute_should_reduce_first_process_by_quantum() {
        let start_quantum = 30;
        let run_time = 5;
        let expected_time = start_quantum - run_time;
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(Process::new(0, start_quantum))
            .execute(run_time);
        let ran_proc = fifo_scheduler.scheduled_process().unwrap();
        assert_eq!(ran_proc.quantum_left, expected_time);
    }

    #[test]
    fn fifo_scheduler_execute_should_increase_total_time_on_all_processes() {
        let expected_time = vec![5, 5, 5];
        let run_time = 5;
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(Process::new(0, 25))
            .add_process(Process::new(1, 30))
            .add_process(Process::new(2, 35))
            .execute(run_time);
        let total_times: Vec<u32> = fifo_scheduler
            .process_queue
            .into_iter()
            .map(|x| x.total_time)
            .collect();
        assert_eq!(expected_time, total_times);
    }

    #[test]
    fn fifo_scheduler_schedule_next_should_schedule_next_process_after_execute() {
        let run_time = 10;
        let switch_cost = 5;
        let expected_process_id = 1;
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(Process::new(0, 10))
            .add_process(Process::new(expected_process_id, 15))
            .execute(run_time)
            .schedule_next(switch_cost);
        let curr_process = fifo_scheduler.scheduled_process().unwrap();
        assert_eq!(expected_process_id, curr_process.id);
        assert_eq!(1, fifo_scheduler.process_queue.len());
    }

    #[test]
    fn fifo_scheduler_schedule_next_should_increment_all_process_total_time_by_switch_cost() {
        let switch_cost = 5;
        let expected_total_times = vec![switch_cost, switch_cost];
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(Process::new(0, 10))
            .add_process(Process::new(1, 15))
            .schedule_next(switch_cost);

        assert_eq!(fifo_scheduler.scheduled_process().unwrap().id, 0);

        let total_times: Vec<u32> = fifo_scheduler
            .process_queue
            .into_iter()
            .map(|x| x.total_time)
            .collect();

        assert_eq!(expected_total_times, total_times);
    }

    #[test]
    fn process_execute_quantum_left_lower_than_quantum_used_should_not_buffer_overflow() {
        let process = Process::new(0, 5).execute(10);
        assert_eq!(process.quantum_left, 0);
    }
}
