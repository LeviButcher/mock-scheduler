use crate::FakeProcess;

#[derive(Copy, Clone)]
pub struct QueuedProcess {
    pub entered_number: u32,
    pub process: FakeProcess,
}

impl QueuedProcess {
    pub fn wait(self, quantum_wait: u32) -> QueuedProcess {
        QueuedProcess {
            entered_number: self.entered_number,
            process: self.process.wait(quantum_wait),
        }
    }
    pub fn execute(&self, quantum_run: u32) -> (QueuedProcess, u32) {
        let (process, ran_for) = self.process.execute(quantum_run);
        (
            QueuedProcess {
                entered_number: self.entered_number,
                process,
            },
            ran_for,
        )
    }
}
