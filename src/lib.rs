mod scheduler;
pub use crate::scheduler::*;

pub trait Scheduler {
    fn new() -> Self;
    fn add_process(self, proc: FakeProcess) -> Self;
    fn execute(self, cpu_cycle_quantum_usage: u32) -> Self;
    fn schedule_next(self, context_switch_cost: u32) -> Self;
    fn scheduled_process(&self) -> Option<&FakeProcess>;
    fn print_process_table(&self);
}

// FIFO -> sort by ID, asc
// shortestJobNext -> sort by quantum-run-for, asc
// shortest remain time -> sort by quantum-left, asc

pub struct FIFOScheduler {
    pub process_queue: Vec<FakeProcess>,
}

impl Scheduler for FIFOScheduler {
    fn new() -> FIFOScheduler {
        let process_queue: Vec<FakeProcess> = Vec::new();
        FIFOScheduler { process_queue }
    }
    fn add_process(mut self, proc: FakeProcess) -> FIFOScheduler {
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
                let queue: Vec<FakeProcess> = queue
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
        let queue: Vec<FakeProcess> = self
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
    fn scheduled_process(&self) -> Option<&FakeProcess> {
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
        let fifo_scheduler = fifo_scheduler.add_process(FakeProcess::new(0, 32));
        assert_eq!(fifo_scheduler.process_queue.len(), 1);
    }

    #[test]
    fn fifo_scheduler_add_process_should_add_process_to_back() {
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(FakeProcess::new(0, 32))
            .add_process(FakeProcess::new(1, 42));
        let last_process = fifo_scheduler.process_queue.last().unwrap();
        assert_eq!(last_process.id, 1);
    }

    #[test]
    fn fifo_scheduler_execute_should_reduce_first_process_by_quantum() {
        let start_quantum = 30;
        let run_time = 5;
        let expected_time = start_quantum - run_time;
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(FakeProcess::new(0, start_quantum))
            .execute(run_time);
        let ran_proc = fifo_scheduler.scheduled_process().unwrap();
        assert_eq!(ran_proc.quantum_left, expected_time);
    }

    #[test]
    fn fifo_scheduler_execute_should_increase_total_time_on_all_processes() {
        let expected_time = vec![5, 5, 5];
        let run_time = 5;
        let fifo_scheduler = FIFOScheduler::new()
            .add_process(FakeProcess::new(0, 25))
            .add_process(FakeProcess::new(1, 30))
            .add_process(FakeProcess::new(2, 35))
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
            .add_process(FakeProcess::new(0, 10))
            .add_process(FakeProcess::new(expected_process_id, 15))
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
            .add_process(FakeProcess::new(0, 10))
            .add_process(FakeProcess::new(1, 15))
            .schedule_next(switch_cost);

        assert_eq!(fifo_scheduler.scheduled_process().unwrap().id, 0);

        let total_times: Vec<u32> = fifo_scheduler
            .process_queue
            .into_iter()
            .map(|x| x.total_time)
            .collect();

        assert_eq!(expected_total_times, total_times);
    }
}
