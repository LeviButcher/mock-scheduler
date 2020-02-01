mod scheduler;
pub use crate::scheduler::*;
use std::cmp;

pub struct Scheduler {
    pub process_queue: Vec<FakeProcess>,
    pub finished_processes: Vec<FakeProcess>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let process_queue: Vec<FakeProcess> = Vec::new();
        Scheduler {
            process_queue,
            finished_processes: Vec::new(),
        }
    }
    pub fn add_process(mut self, proc: FakeProcess) -> Scheduler {
        // Crying Functional Tears
        self.process_queue.push(proc);
        Scheduler {
            process_queue: self.process_queue,
            finished_processes: self.finished_processes,
        }
    }
    pub fn execute(self, cpu_cycle_quantum_usage: u32) -> Scheduler {
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

                Scheduler {
                    process_queue: [&updated_proc[..], &queue[..]].concat(),
                    finished_processes: self.finished_processes,
                }
            }
            None => self,
        }
    }

    /// Only re-sort the queue, won't remove processes
    /// finished process should be sorted last
    /// When you schedule next, should you only charge wait times only if scheduled process changes
    pub fn schedule_next<F>(self, compare: F, context_switch_cost: u32) -> Self
    where
        F: Fn(&FakeProcess, &FakeProcess) -> cmp::Ordering,
    {
        let mut queue = self.process_queue;
        queue.sort_by(compare);

        let finished_processes = queue.clone().into_iter().filter(|x| x.is_done()).collect();

        let queue = queue
            .into_iter()
            .filter(|x| !x.is_done())
            .map(|x| x.wait(context_switch_cost))
            .collect();
        Scheduler {
            process_queue: queue,
            finished_processes,
        }
    }

    pub fn scheduled_process(&self) -> Option<&FakeProcess> {
        self.process_queue.first()
    }
    pub fn print_process_table(&self) {
        for p in &self.process_queue[..] {
            println!("{}", p);
        }
    }

    pub fn is_queue_empty(&self) -> bool {
        self.process_queue.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scheduler_add_process_should_add_process() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.process_queue.len(), 0);
        let scheduler = scheduler.add_process(FakeProcess::new(0, 32));
        assert_eq!(scheduler.process_queue.len(), 1);
    }

    #[test]
    fn scheduler_add_process_should_add_process_to_back() {
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 32))
            .add_process(FakeProcess::new(1, 42));
        let last_process = scheduler.process_queue.last().unwrap();
        assert_eq!(last_process.id, 1);
    }

    #[test]
    fn scheduler_execute_should_reduce_first_process_by_quantum() {
        let start_quantum = 30;
        let run_time = 5;
        let expected_time = start_quantum - run_time;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, start_quantum))
            .execute(run_time);
        let ran_proc = scheduler.scheduled_process().unwrap();
        assert_eq!(ran_proc.quantum_left, expected_time);
    }

    #[test]
    fn scheduler_execute_should_increase_total_time_on_all_processes() {
        let expected_time = vec![5, 5, 5];
        let run_time = 5;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 25))
            .add_process(FakeProcess::new(1, 30))
            .add_process(FakeProcess::new(2, 35))
            .execute(run_time);
        let total_times: Vec<u32> = scheduler
            .process_queue
            .into_iter()
            .map(|x| x.total_time)
            .collect();
        assert_eq!(expected_time, total_times);
    }

    #[test]
    fn scheduler_schedule_next_by_id_should_schedule_next_process_after_execute() {
        let run_time = 10;
        let switch_cost = 5;
        let expected_process_id = 1;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 10))
            .add_process(FakeProcess::new(expected_process_id, 15))
            .execute(run_time)
            .schedule_next(|a, b| a.id.cmp(&b.id), switch_cost);
        let curr_process = scheduler.scheduled_process().unwrap();
        assert_eq!(expected_process_id, curr_process.id);
        assert_eq!(1, scheduler.process_queue.len());
    }

    #[test]
    fn scheduler_schedule_next_by_id_should_increment_all_process_total_time_by_switch_cost() {
        let switch_cost = 5;
        let expected_total_times = vec![switch_cost, switch_cost];
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 10))
            .add_process(FakeProcess::new(1, 15))
            .schedule_next(|a, b| a.id.cmp(&b.id), switch_cost);

        assert_eq!(scheduler.scheduled_process().unwrap().id, 0);

        let total_times: Vec<u32> = scheduler
            .process_queue
            .into_iter()
            .map(|x| x.total_time)
            .collect();

        assert_eq!(expected_total_times, total_times);
    }

    #[test]
    fn scheduler_schedule_next_by_quantum_left_should_by_in_order() {
        let switch_cost = 5;
        let expected_order = vec![2, 3, 1];
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 20))
            .add_process(FakeProcess::new(2, 5))
            .add_process(FakeProcess::new(3, 15))
            .schedule_next(|a, b| a.quantum_left.cmp(&b.quantum_left), switch_cost);
        let process_order: Vec<u32> = scheduler.process_queue.into_iter().map(|x| x.id).collect();
        assert_eq!(expected_order, process_order);
    }

    #[test]
    fn scheduler_schedule_next_should_put_finished_process_in_finished() {
        let switch_cost = 5;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 0))
            .schedule_next(|a, b| a.quantum_left.cmp(&b.quantum_left), switch_cost);
        assert_eq!(true, scheduler.process_queue.is_empty());
        assert_eq!(1, scheduler.finished_processes.len());
    }
}
