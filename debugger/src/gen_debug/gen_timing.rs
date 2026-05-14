use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use generator::timing::take;

pub fn write_report(run_count: u16) {
    let timings = take();
    let mut file = File::create("timings.md").expect("failed to create timings.md");

    let mut stages: Vec<(&'static str, HashMap<&'static str, Vec<std::time::Duration>>)> = timings.into_iter().collect();
    stages.sort_by_key(|(name, _)| *name);

    for (stage, functions) in stages {
        writeln!(file, "## {}", stage).unwrap();
        writeln!(file, "| function | total (ms) | calls | calls/run | avg/call (ms) | avg/run (ms) | min (ms) | max (ms) | first (ms) | last (ms) |").unwrap();
        writeln!(file, "|----------|------------|-------|-----------|---------------|--------------|----------|----------|------------|-----------|").unwrap();

        let mut entries: Vec<(&'static str, Vec<std::time::Duration>)> = functions.into_iter().collect();
        entries.sort_by(|(_, a), (_, b)| {
            let a_total: std::time::Duration = a.iter().sum();
            let b_total: std::time::Duration = b.iter().sum();
            b_total.cmp(&a_total)
        });

        for (name, durations) in entries {
            let total: std::time::Duration = durations.iter().sum();
            let count = durations.len();
            let avg_per_call = total / count as u32;
            let avg_per_run = total / run_count as u32;
            let min = durations.iter().min().unwrap();
            let max = durations.iter().max().unwrap();
            let first = durations.first().unwrap();
            let last = durations.last().unwrap();

            writeln!(file, "| {} | {:.3} | {} | {:.1} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |",
                name,
                total.as_secs_f64() * 1000.0,
                count,
                count as f64 / run_count as f64,
                avg_per_call.as_secs_f64() * 1000.0,
                avg_per_run.as_secs_f64() * 1000.0,
                min.as_secs_f64() * 1000.0,
                max.as_secs_f64() * 1000.0,
                last.as_secs_f64() * 1000.0,
                first.as_secs_f64() * 1000.0,
            ).unwrap();
        }
        writeln!(file, "").unwrap();
    }
}