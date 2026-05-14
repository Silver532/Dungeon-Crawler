use std::fs::File;
use std::io::Write;

use generator::timing::take;

pub fn write_report(run_count: u16) {
    let timings = take();
    let mut file = File::create("timings.txt").expect("failed to create timings.txt");

    let mut entries: Vec<(&'static str, Vec<std::time::Duration>)> = timings.into_iter().collect();
    entries.sort_by_key(|(name, _)| *name);

    for (name, durations) in entries {
        let total: std::time::Duration = durations.iter().sum();
        let count = durations.len();
        let avg_per_call = total / count as u32;
        let avg_per_run = total / run_count as u32;
        let min = durations.iter().min().unwrap();
        let max = durations.iter().max().unwrap();

        writeln!(file, "{}", name).unwrap();
        writeln!(file, "\ttotal:        {:.3}ms", total.as_secs_f64() * 1000.0).unwrap();
        writeln!(file, "\tcalls:        {}", count).unwrap();
        writeln!(file, "\tcalls/run:    {:.1}", count as f64 / run_count as f64).unwrap();
        writeln!(file, "\tavg/call:     {:.3}ms", avg_per_call.as_secs_f64() * 1000.0).unwrap();
        writeln!(file, "\tavg/run:      {:.3}ms", avg_per_run.as_secs_f64() * 1000.0).unwrap();
        writeln!(file, "\tmin:          {:.3}ms", min.as_secs_f64() * 1000.0).unwrap();
        writeln!(file, "\tmax:          {:.3}ms", max.as_secs_f64() * 1000.0).unwrap();
        writeln!(file, "").unwrap();
    }
}