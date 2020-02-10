mod scheduler;
pub use crate::scheduler::*;
use std::cmp;

pub struct Scheduler {
    pub process_queue: Vec<QueuedProcess>,
    pub finished_processes: Vec<QueuedProcess>,
    next_entered_number: u32,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let process_queue: Vec<QueuedProcess> = Vec::new();
        Scheduler {
            process_queue,
            finished_processes: Vec::new(),
            next_entered_number: 1,
        }
    }
    pub fn add_process(mut self, proc: FakeProcess) -> Scheduler {
        self.process_queue.push(QueuedProcess {
            entered_number: self.next_entered_number,
            process: proc,
        });
        Scheduler {
            process_queue: self.process_queue,
            finished_processes: self.finished_processes,
            next_entered_number: self.next_entered_number + 1,
        }
    }
    pub fn execute(self, cpu_cycle_quantum_usage: u32) -> RanScheduler {
        match self.scheduled_process() {
            Some(p) => {
                let (p, ran_for) = p.execute(cpu_cycle_quantum_usage);
                let updated_proc = vec![p];
                let queue = match self.process_queue.as_slice().split_first() {
                    Some((_, rest)) => rest,
                    None => &[],
                };
                let queue: Vec<QueuedProcess> = queue
                    .to_vec()
                    .into_iter()
                    .map(|x| x.wait(ran_for))
                    .collect();
                RanScheduler {
                    scheduler: Scheduler {
                        process_queue: [&updated_proc[..], &queue[..]].concat(),
                        finished_processes: self.finished_processes,
                        next_entered_number: self.next_entered_number,
                    },
                }
            }
            None => RanScheduler { scheduler: self },
        }
    }

    pub fn scheduled_process(&self) -> Option<&QueuedProcess> {
        self.process_queue.first()
    }
    pub fn print_process_table(&self) {
        for p in &self.process_queue[..] {
            println!("| entered_num: {} {}", p.entered_number, p.process);
        }
    }

    pub fn is_queue_empty(&self) -> bool {
        self.process_queue.is_empty()
    }
}

pub struct RanScheduler {
    scheduler: Scheduler,
}

impl RanScheduler {
    pub fn schedule_next<F>(self, compare: F, context_switch_cost: u32) -> Scheduler
    where
        F: Fn(&QueuedProcess, &QueuedProcess) -> cmp::Ordering,
    {
        let mut queue = self.scheduler.process_queue;
        let mut finished_processes = self.scheduler.finished_processes;
        let scheduled = queue.remove(0);

        queue.sort_by(compare);

        let queue: Vec<QueuedProcess> = queue
            .into_iter()
            .map(|x| x.wait(context_switch_cost))
            .collect();

        if scheduled.process.is_done() {
            finished_processes.push(scheduled);
            return Scheduler {
                process_queue: queue,
                finished_processes,
                next_entered_number: self.scheduler.next_entered_number,
            };
        }

        let scheduler = Scheduler {
            process_queue: queue,
            finished_processes,
            next_entered_number: self.scheduler.next_entered_number,
        };
        scheduler.add_process(scheduled.process.wait(context_switch_cost))
    }

    pub fn ran_process(&self) -> Option<&QueuedProcess> {
        self.scheduler.scheduled_process()
    }

    pub fn print_process_table(&self) {
        self.scheduler.print_process_table();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scheduler_add_process_should_add_process() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.process_queue.len(), 0);
        let scheduler = scheduler.add_process(FakeProcess::new(0, 32, 1));
        assert_eq!(scheduler.process_queue.len(), 1);
    }

    #[test]
    fn scheduler_add_process_should_add_process_to_back() {
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 32, 1))
            .add_process(FakeProcess::new(1, 42, 1));
        let last_process = scheduler.process_queue.last().unwrap();
        assert_eq!(last_process.process.id, 1);
    }

    #[test]
    fn scheduler_execute_should_reduce_first_process_by_quantum() {
        let start_quantum = 30;
        let run_time = 5;
        let expected_time = start_quantum - run_time;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, start_quantum, 1))
            .execute(run_time);
        let ran_proc = scheduler.ran_process().unwrap();
        assert_eq!(ran_proc.process.quantum_left, expected_time);
    }

    #[test]
    fn scheduler_execute_should_increase_total_time_on_all_processes() {
        let expected_time = vec![5, 5, 5];
        let run_time = 5;
        let ran_scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 25, 1))
            .add_process(FakeProcess::new(1, 30, 1))
            .add_process(FakeProcess::new(2, 35, 1))
            .execute(run_time);
        let total_times: Vec<u32> = ran_scheduler
            .scheduler
            .process_queue
            .into_iter()
            .map(|x| x.process.get_turnaround_time())
            .collect();
        assert_eq!(expected_time, total_times);
    }

    #[test]
    fn scheduler_schedule_next_by_id_should_schedule_next_process_after_execute() {
        let run_time = 5;
        let switch_cost = 5;
        let expected_process_id = 1;

        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 10, 1))
            .add_process(FakeProcess::new(expected_process_id, 1, 15))
            .execute(run_time)
            .schedule_next(algorithms::first_come, switch_cost);

        let curr_process = scheduler.scheduled_process().unwrap();
        assert_eq!(expected_process_id, curr_process.process.id);
        assert_eq!(2, scheduler.process_queue.len());
    }

    #[test]
    fn ran_scheduler_schedule_next_by_id_should_increment_all_process_total_time_by_switch_cost() {
        let switch_cost = 5;
        let expected_total_times = vec![switch_cost, switch_cost];
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(0, 10, 1))
            .add_process(FakeProcess::new(1, 15, 1))
            .execute(0)
            .schedule_next(algorithms::first_come, switch_cost);

        assert_eq!(1, scheduler.scheduled_process().unwrap().process.id);

        let total_times: Vec<u32> = scheduler
            .process_queue
            .into_iter()
            .map(|x| x.process.get_turnaround_time())
            .collect();

        assert_eq!(expected_total_times, total_times);
    }

    #[test]
    fn scheduler_schedule_next_by_entered_should_be_in_expected_order() {
        let expected = vec![3, 4, 5];
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 10, 1))
            .add_process(FakeProcess::new(2, 10, 1))
            .add_process(FakeProcess::new(3, 10, 1))
            .execute(1)
            .schedule_next(algorithms::first_come, 1)
            .execute(1)
            .schedule_next(algorithms::first_come, 1);

        let order_of_schedule: Vec<u32> = scheduler
            .process_queue
            .into_iter()
            .map(|x| x.entered_number)
            .collect();
        assert_eq!(expected, order_of_schedule);
    }

    #[test]
    fn scheduler_schedule_next_by_quantum_left_should_be_in_order() {
        let switch_cost = 5;
        let expected_order = vec![2, 3, 1];
        let ran_scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 20, 1))
            .add_process(FakeProcess::new(2, 5, 1))
            .add_process(FakeProcess::new(3, 15, 1))
            .execute(1)
            .schedule_next(algorithms::shortest_next, switch_cost);
        let process_order: Vec<u32> = ran_scheduler
            .process_queue
            .into_iter()
            .map(|x| x.process.id)
            .collect();
        assert_eq!(expected_order, process_order);
    }

    #[test]
    fn scheduler_schedule_next_should_not_schedule_previously_ran_process() {
        let switch_cost = 5;
        let expected_order = vec![2, 1];

        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 20, 1))
            .add_process(FakeProcess::new(2, 5, 1))
            .execute(5)
            .schedule_next(algorithms::first_come, switch_cost);

        let process_order: Vec<u32> = scheduler
            .process_queue
            .into_iter()
            .map(|x| x.process.id)
            .collect();
        assert_eq!(expected_order, process_order);
    }

    #[test]
    fn scheduler_schedule_next_should_put_finished_process_in_finished() {
        let switch_cost = 5;
        let scheduler = Scheduler::new()
            .add_process(FakeProcess::new(1, 0, 1))
            .execute(0)
            .schedule_next(algorithms::shortest_remain, switch_cost);
        assert_eq!(true, scheduler.process_queue.is_empty());
        assert_eq!(1, scheduler.finished_processes.len());
    }
}
