extern crate scheduler;
use scheduler::{algorithms, *};
use std::{thread, time};
use termion::clear;

static mut NEXT_ID: u32 = 1;

// Quantum used each loop 4
const CPU_CYCLE_QUANTUM_USED: u32 = 4;
// How much time it takes to swap a process
const CONTEXT_SWITCH_TIME: u32 = 0;

const LOOP_PAUSE: time::Duration = time::Duration::from_millis(2000);

fn main() {
    let mut scheduler = Scheduler::new();
    let mut cycle_count = 0;
    let chosen_algorithm = algorithms::shortest_remain;

    loop {
        if let Some(p) = get_process(cycle_count) {
            unsafe {
                NEXT_ID = NEXT_ID + 1;
            }
            scheduler = scheduler.add_process(p);
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
            0 => Some(FakeProcess::new(NEXT_ID, 60)),
            3 => Some(FakeProcess::new(NEXT_ID, 20)),
            5 => Some(FakeProcess::new(NEXT_ID, 10)),
            9 => Some(FakeProcess::new(NEXT_ID, 70)),
            10 => Some(FakeProcess::new(NEXT_ID, 50)),
            12 => Some(FakeProcess::new(NEXT_ID, 30)),
            14 => Some(FakeProcess::new(NEXT_ID, 40)),
            16 => Some(FakeProcess::new(NEXT_ID, 50)),
            17 => Some(FakeProcess::new(NEXT_ID, 70)),
            19 => Some(FakeProcess::new(NEXT_ID, 20)),
            _ => None,
        }
    }
}
