use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use futures::task::Task;

#[derive(Clone)]
pub struct Interval {
    counter: Arc<AtomicUsize>,
    still_running: Arc<AtomicBool>,
    task: Arc<Mutex<Option<Task>>>,
}

impl Drop for Interval {
    fn drop(&mut self) {
        println!("Interval thread shutting down");
        self.still_running.store(false, Ordering::SeqCst);
    }
}

impl Interval {
    pub fn from_millis(millis: u64) -> Interval {
        let duration = Duration::from_millis(millis);

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let still_running = Arc::new(AtomicBool::new(true));
        let still_running_clone = still_running.clone();

        let task: Arc<Mutex<Option<Task>>> = Arc::new(Mutex::new(None));
        let task_clone = task.clone();

        spawn(move || {
            println!("Interval thread launched");
            while still_running_clone.load(Ordering::SeqCst) {
                sleep(duration);
                let old = counter_clone.fetch_add(1, Ordering::SeqCst);
                println!("Interval thread still alive, value was: {}", old);

                let task = task_clone.lock().unwrap();
                match *task {
                    None => (),
                    Some(ref task) => task.notify(),
                };
            }
        });

        Interval {
            counter,
            still_running,
            task,
        }
    }

    pub fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    pub fn set_task(&mut self, task: Task) {
        let mut guard = self.task.lock().unwrap();
        *guard = Some(task);
    }
}
