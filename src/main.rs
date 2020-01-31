use scheduler::*;
use std::{thread, time};
use termion::clear;

// Quantum used each loop 4
const CPU_CYCLE_QUANTUM_USED: u32 = 4;
// How much time it takes to swap a process
const CONTEXT_SWITCH_TIME: u32 = 0;

const LOOP_PAUSE: time::Duration = time::Duration::from_millis(1000);

fn main() {
    let mut scheduler = FIFOScheduler::new();
    let mut cycle_count = 0;

    loop {
        // Add to scheduler
        if let Some(p) = get_process(cycle_count) {
            scheduler = scheduler.add_process(p);
        }

        scheduler = scheduler.execute(CPU_CYCLE_QUANTUM_USED);
        scheduler.print_process_table();
        scheduler = scheduler.schedule_next(CONTEXT_SWITCH_TIME);
        cycle_count = cycle_count + 1;
        thread::sleep(LOOP_PAUSE);
        println!();
    }
}

fn get_process(cpu_cycle: u32) -> Option<Process> {
    let choose_process = cpu_cycle % 20;
    match choose_process {
        0 => Some(Process::new(0, 60)),
        3 => Some(Process::new(1, 20)),
        5 => Some(Process::new(2, 10)),
        9 => Some(Process::new(3, 70)),
        10 => Some(Process::new(4, 50)),
        12 => Some(Process::new(5, 30)),
        14 => Some(Process::new(6, 40)),
        16 => Some(Process::new(7, 50)),
        17 => Some(Process::new(8, 70)),
        19 => Some(Process::new(9, 20)),
        _ => None,
    }
}
