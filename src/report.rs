use instant::Instant;
use std::{
    hash::{Hash, Hasher},
    ops::Deref,
    time::Duration,
};

//

#[derive(Debug, Clone, Copy)]
pub struct Reporter {
    count: u32,
    elapsed: Duration,
    report_timer: Instant,
    report_interval: Duration,

    last_interval: Option<Duration>,
    last_per_second: Option<f64>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Timer {
    begin: Instant,
}

//

impl Reporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_interval(report_interval: Duration) -> Self {
        Self {
            count: 0_u32,
            elapsed: Duration::default(),
            report_timer: Instant::now(),
            report_interval,

            last_interval: None,
            last_per_second: None,
        }
    }

    pub fn begin(&self) -> Timer {
        Timer {
            begin: Instant::now(),
        }
    }

    pub fn end(&mut self, timer: Timer) {
        self.elapsed += timer.begin.elapsed();
        self.count += 1;
    }

    pub fn time<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let timer = self.begin();
        let result = f();
        self.end(timer);
        result
    }

    pub fn should_report(&mut self) -> bool {
        self.report_timer.elapsed() >= self.report_interval
    }

    pub fn report_interval(&self) -> Duration {
        self.report_interval
    }

    pub fn report_all<'a, I>(label: &str, reporters: I) -> String
    where
        I: IntoIterator<Item = (&'static str, &'a mut Self)>,
    {
        #[cfg(debug_assertions)]
        const DEBUG: &str = "debug build";
        #[cfg(not(debug_assertions))]
        const DEBUG: &str = "release build";

        let reporters: Vec<_> = reporters
            .into_iter()
            .map(|(label, reporter)| {
                reporter.reset();
                (label, reporter.last_string())
            })
            .collect();

        let max_label_width = reporters
            .iter()
            .map(|(label, (_, time_per))| label.len() + time_per.len())
            .max()
            .unwrap_or(0)
            .max(7);
        let padding = " ".repeat(max_label_width - 7);
        let first = format!("Report {label} ({DEBUG})\n{padding}per second @ time per\n");

        Some(first)
            .into_iter()
            .chain(reporters.iter().map(|(label, (int, per_sec))| {
                let padding = " ".repeat(max_label_width - label.len() - per_sec.len() + 1);
                format!("{label}: {padding}{per_sec} @ {int}\n")
            }))
            .collect()
    }

    pub fn reset(&mut self) {
        let avg = self.elapsed.checked_div(self.count);
        let fps = self.count as f64 / self.report_interval.as_secs_f64();

        self.count = 0;
        self.elapsed = Duration::default();
        self.report_timer = Instant::now();
        self.last_interval = avg;
        self.last_per_second = Some(fps);
    }

    pub fn last(&self) -> Option<(Duration, f64)> {
        Some((self.last_interval?, self.last_per_second?))
    }

    pub fn last_string(&self) -> (String, String) {
        self.last_string_prec(4, 1)
    }

    pub fn last_string_prec(&self, int_prec: usize, per_sec_prec: usize) -> (String, String) {
        (
            self.last_interval
                .map(|ft| format!("{ft:.int_prec$?}"))
                .unwrap_or_else(|| "...".into()),
            self.last_per_second
                .map(|ps| format!("{ps:.per_sec_prec$}"))
                .unwrap_or_else(|| "...".into()),
        )
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new_with_interval(Duration::from_secs(3))
    }
}

impl PartialEq for Reporter {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
            && self.elapsed == other.elapsed
            && self.report_timer == other.report_timer
            && self.report_interval == other.report_interval
    }
}

impl Eq for Reporter {}

impl Hash for Reporter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.count.hash(state);
        self.elapsed.hash(state);
        self.report_timer.hash(state);
        self.report_interval.hash(state);
    }
}

impl Deref for Timer {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.begin
    }
}
