extern crate scheduler;
use scheduler::{algorithms, *};
use std::{thread, time};
use termion::clear;

static mut NEXT_ID: u32 = 1;

// Quantum used each loop 4
const CPU_CYCLE_QUANTUM_USED: u32 = 4;
// How much time it takes to swap a process
const CONTEXT_SWITCH_TIME: u32 = 0;

const LOOP_PAUSE: time::Duration = time::Duration::from_millis(1000);

fn main() {
    let mut scheduler = Scheduler::new();
    let mut cycle_count = 0;
    let chosen_algorithm = algorithms::shortest_remain;

    loop {
        println!(
            "\nQuantum_Time: {}\tContext_Switch: {}\n",
            CPU_CYCLE_QUANTUM_USED, CONTEXT_SWITCH_TIME
        );
        if let Some(p) = get_process(cycle_count) {
            unsafe {
                NEXT_ID = NEXT_ID + 1;
            }
            scheduler = scheduler.add_process(p);
        }
        if scheduler.is_queue_empty() {
            println!("IDLE, no more work");
            continue;
        }

        scheduler = scheduler
            .execute(CPU_CYCLE_QUANTUM_USED)
            .schedule_next(chosen_algorithm, CONTEXT_SWITCH_TIME);
        scheduler.print_process_table();
        cycle_count = cycle_count + 1;
        thread::sleep(LOOP_PAUSE);
        println!("{}", clear::All);
    }
}

fn get_process(cpu_cycle: u32) -> Option<FakeProcess> {
    unsafe {
        match cpu_cycle {
            0 => Some(FakeProcess::new(NEXT_ID, 60, 1)),
            3 => Some(FakeProcess::new(NEXT_ID, 20, 2)),
            5 => Some(FakeProcess::new(NEXT_ID, 10, 3)),
            9 => Some(FakeProcess::new(NEXT_ID, 70, 4)),
            10 => Some(FakeProcess::new(NEXT_ID, 50, 1)),
            12 => Some(FakeProcess::new(NEXT_ID, 30, 2)),
            14 => Some(FakeProcess::new(NEXT_ID, 40, 3)),
            16 => Some(FakeProcess::new(NEXT_ID, 50, 4)),
            17 => Some(FakeProcess::new(NEXT_ID, 70, 1)),
            19 => Some(FakeProcess::new(NEXT_ID, 20, 2)),
            _ => None,
        }
    }
}
