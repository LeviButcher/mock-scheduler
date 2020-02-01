extern crate scheduler;
use scheduler::{algorithms, *};

fn main() {
    println!("The Scheduler Report - By: Levi Butcher");

    run_report(4, 0);
    run_report(4, 1);
    run_report(8, 4);
}

fn run_report(quantum_used: u32, context_switch: u32) {
    let mut fifo_scheduler = Scheduler::new();
    let mut shortest_next_scheduler = Scheduler::new();
    let mut shortest_remain_scheduler = Scheduler::new();

    fifo_scheduler = get_processes()
        .into_iter()
        .fold(fifo_scheduler, |acc, curr| {
            acc.add_process(curr)
                .execute(quantum_used)
                .schedule_next(algorithms::first_come, context_switch)
        });

    shortest_next_scheduler =
        get_processes()
            .into_iter()
            .fold(shortest_next_scheduler, |acc, curr| {
                acc.add_process(curr)
                    .execute(quantum_used)
                    .schedule_next(algorithms::shortest_next, context_switch)
            });

    shortest_remain_scheduler =
        get_processes()
            .into_iter()
            .fold(shortest_remain_scheduler, |acc, curr| {
                acc.add_process(curr)
                    .execute(quantum_used)
                    .schedule_next(algorithms::shortest_remain, context_switch)
            });

    // Could clean this up by storing sort algorithm
    // when constructing scheduler
    loop {
        if fifo_scheduler.is_queue_empty() {
            break;
        }
        fifo_scheduler = fifo_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::first_come, context_switch);
    }

    loop {
        if shortest_next_scheduler.is_queue_empty() {
            break;
        }
        shortest_next_scheduler = shortest_next_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::shortest_next, context_switch)
    }

    loop {
        if shortest_remain_scheduler.is_queue_empty() {
            break;
        }

        shortest_remain_scheduler = shortest_remain_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::shortest_remain, context_switch)
    }

    let schedulers = vec![
        fifo_scheduler,
        shortest_next_scheduler,
        shortest_remain_scheduler,
    ];

    let average_turnarounds: Vec<u32> = schedulers
        .iter()
        .map(|x| {
            x.finished_processes
                .iter()
                .fold(0, |acc, curr| acc + curr.total_time)
                / get_processes().len() as u32
        })
        .collect();

    let average_wait_times: Vec<u32> = schedulers
        .iter()
        .map(|x| {
            x.finished_processes
                .iter()
                .fold(0, |acc, curr| acc + curr.time_spent_waiting)
                / get_processes().len() as u32
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

fn get_processes() -> Vec<FakeProcess> {
    vec![
        FakeProcess::new(0, 60),
        FakeProcess::new(1, 20),
        FakeProcess::new(2, 10),
        FakeProcess::new(3, 70),
        FakeProcess::new(4, 50),
        FakeProcess::new(5, 30),
        FakeProcess::new(6, 40),
        FakeProcess::new(7, 50),
        FakeProcess::new(8, 70),
        FakeProcess::new(9, 20),
    ]
}
