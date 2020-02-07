extern crate scheduler;
use scheduler::{algorithms, *};
use std::{cmp, thread};
const NUM_OF_PROCESSES: u32 = 10;

fn main() {
    println!("The Scheduler Report - By: Levi Butcher\n");
    let report_runs = vec![(4, 0), (4, 1), (8, 4)];
    let num_of_reports = report_runs.len();
    let result = report_runs
        .into_iter()
        .map(|(quantum, switch)| thread::spawn(move || run_report(quantum, switch)))
        .map(|thread| thread.join())
        .collect::<Vec<Result<_, _>>>();
    assert_eq!(num_of_reports, result.len());
}

fn run_report(quantum_used: u32, context_switch: u32) {
    let schedule_algorithms: Vec<fn(&QueuedProcess, &QueuedProcess) -> cmp::Ordering> = vec![
        algorithms::first_come,
        algorithms::shortest_next,
        algorithms::shortest_remain,
    ];
    let schedulers = vec![Scheduler::new(), Scheduler::new(), Scheduler::new()];

    let schedulers: Vec<Scheduler> = schedule_algorithms
        .into_iter()
        .zip(schedulers.into_iter())
        .map(|(algorithm, scheduler)| {
            thread::spawn(move || {
                run_process_on_scheduler(scheduler, &algorithm, quantum_used, context_switch)
            })
        })
        .map(|handle| handle.join().unwrap())
        .collect();

    let average_turnarounds: Vec<u32> = schedulers
        .iter()
        .map(|x| {
            x.finished_processes
                .iter()
                .fold(0, |acc, curr| acc + curr.process.get_turnaround_time())
                / NUM_OF_PROCESSES
        })
        .collect();

    let average_wait_times: Vec<u32> = schedulers
        .iter()
        .map(|x| {
            x.finished_processes
                .iter()
                .fold(0, |acc, curr| acc + curr.process.time_spent_waiting)
                / NUM_OF_PROCESSES
        })
        .collect();

    println!(
        "CPU_QUANTUM_TIME: {}\tCONTEXT_SWITCH:{}",
        quantum_used, context_switch
    );
    println!("|         Name         | Avg Turnaround Time | Waiting Time |");
    println!(
        "|  FiFo                |          {}         |       {}     |",
        average_turnarounds.get(0).unwrap(),
        average_wait_times.get(0).unwrap()
    );
    println!(
        "|  Shortest Next       |          {}         |       {}     |",
        average_turnarounds.get(1).unwrap(),
        average_wait_times.get(1).unwrap()
    );
    println!(
        "|  Shortest Remaining  |          {}         |       {}     |",
        average_turnarounds.get(2).unwrap(),
        average_wait_times.get(2).unwrap()
    );
    println!();
}

fn get_process(cpu_cycle: u32) -> Option<FakeProcess> {
    let choose_process = cpu_cycle;
    match choose_process {
        0 => Some(FakeProcess::new(1, 60)),
        3 => Some(FakeProcess::new(2, 20)),
        5 => Some(FakeProcess::new(3, 10)),
        9 => Some(FakeProcess::new(4, 70)),
        10 => Some(FakeProcess::new(5, 50)),
        12 => Some(FakeProcess::new(6, 30)),
        14 => Some(FakeProcess::new(7, 40)),
        16 => Some(FakeProcess::new(8, 50)),
        17 => Some(FakeProcess::new(9, 70)),
        19 => Some(FakeProcess::new(10, 20)),
        _ => None,
    }
}

fn run_process_on_scheduler<F>(
    scheduler: Scheduler,
    sort: &F,
    quantum_time: u32,
    context_switch: u32,
) -> Scheduler
where
    F: Fn(&QueuedProcess, &QueuedProcess) -> cmp::Ordering,
{
    let mut cycle_count = 0;
    let mut scheduler = scheduler;
    loop {
        if let Some(p) = get_process(cycle_count) {
            scheduler = scheduler.add_process(p);
        }

        scheduler = scheduler
            .execute(quantum_time)
            .schedule_next(sort, context_switch);

        cycle_count = cycle_count + 1;
        if scheduler.is_queue_empty() {
            break;
        }
    }
    scheduler
}
