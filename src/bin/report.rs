extern crate scheduler;
use scheduler::{algorithms, *};
const NUM_OF_PROCESSES: u32 = 10;

fn main() {
    println!("The Scheduler Report - By: Levi Butcher");

    run_report(4, 0);
    run_report(4, 1);
    run_report(8, 4);
}

fn run_report(quantum_used: u32, context_switch: u32) {
    let mut cycle_count = 0;
    let mut fifo_scheduler = Scheduler::new();
    let mut shortest_next_scheduler = Scheduler::new();
    let mut shortest_remain_scheduler = Scheduler::new();

    loop {
        if let Some(p) = get_process(cycle_count) {
            fifo_scheduler = fifo_scheduler.add_process(p);
            shortest_next_scheduler = shortest_next_scheduler.add_process(p);
            shortest_remain_scheduler = shortest_remain_scheduler.add_process(p);
        }

        fifo_scheduler = fifo_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::first_come, context_switch);

        shortest_next_scheduler = shortest_next_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::shortest_next, context_switch);

        shortest_remain_scheduler = shortest_remain_scheduler
            .execute(quantum_used)
            .schedule_next(algorithms::shortest_remain, context_switch);

        if fifo_scheduler.is_queue_empty()
            && shortest_remain_scheduler.is_queue_empty()
            && shortest_remain_scheduler.is_queue_empty()
        {
            break;
        }

        cycle_count = cycle_count + 1;
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
                / NUM_OF_PROCESSES
        })
        .collect();

    let average_wait_times: Vec<u32> = schedulers
        .iter()
        .map(|x| {
            x.finished_processes
                .iter()
                .fold(0, |acc, curr| acc + curr.time_spent_waiting)
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
