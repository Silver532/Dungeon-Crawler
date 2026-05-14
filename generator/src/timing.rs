use std::cell::RefCell;
use indexmap::IndexMap;
use std::time::Duration;

thread_local! {
    static TIMINGS: RefCell<IndexMap<&'static str, IndexMap<&'static str, Vec<Duration>>>> = RefCell::new(IndexMap::new());
}

pub fn record(stage: &'static str, name: &'static str, duration: Duration) {
    TIMINGS.with(|t| {
        t.borrow_mut()
            .entry(stage)
            .or_default()
            .entry(name)
            .or_default()
            .push(duration);
    });
}

pub fn take() -> IndexMap<&'static str, IndexMap<&'static str, Vec<Duration>>> {
    TIMINGS.with(|t| {
        let mut map = t.borrow_mut();
        std::mem::take(&mut *map)
    })
}