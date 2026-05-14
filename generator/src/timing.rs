use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

thread_local! {
    static TIMINGS: RefCell<HashMap<&'static str, Vec<Duration>>> = RefCell::new(HashMap::new());
}

pub fn record(name: &'static str, duration: Duration) {
    TIMINGS.with(|t| {
        t.borrow_mut()
            .entry(name)
            .or_default()
            .push(duration);
    });
}

pub fn take() -> HashMap<&'static str, Vec<Duration>> {
    TIMINGS.with(|t| {
        let mut map = t.borrow_mut();
        std::mem::take(&mut *map)
    })
}