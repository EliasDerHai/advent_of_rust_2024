// might not necessarily be used in "prod" / only for debugging
#![allow(unused)]

use std::{collections::HashMap, time::Instant};

pub struct TimingLogger(HashMap<String, std::time::Instant>);

impl TimingLogger {
    pub fn new() -> Self {
        TimingLogger(HashMap::new())
    }

    pub fn new_running(label: &str) -> Self {
        let mut map: HashMap<String, Instant> = HashMap::new();
        map.insert(label.to_string(), Instant::now());
        TimingLogger(map)
    }

    pub fn start(&mut self, label: &str) {
        self.0.insert(label.to_string(), Instant::now());
    }

    pub fn stop(&mut self, label: &str) {
        match self.0.get(label) {
            Some(inst) => {
                println!(
                    "{label} took {elapsed}ms",
                    elapsed = inst.elapsed().as_millis()
                );
                self.0.remove(label);
            }
            None => eprintln!("'{label}' does not match any tracked timers"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn new_running() {
        let mut t = TimingLogger::new_running("example");
        thread::sleep(Duration::from_millis(10));
        t.stop("example");
        assert_eq!(0, t.0.len());
    }

    #[test]
    fn start_stop() {
        let mut t = TimingLogger::new();
        t.start("example");
        thread::sleep(Duration::from_millis(10));
        t.stop("example");
        assert_eq!(0, t.0.len());
    }
}
