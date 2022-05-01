use instant::Instant;
use std::{ops::Deref, time::Duration};

//

#[derive(Debug)]
pub struct Reporter {
    count: u32,
    elapsed: Duration,
    report_timer: Instant,
    report_interval: Duration,

    last_interval: Option<Duration>,
    last_per_second: Option<f64>,
}

#[derive(Debug)]
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

    pub fn report_all<const COUNT: usize>(
        label: &str,
        reporters: [(&'static str, &mut Self); COUNT],
    ) -> String {
        #[cfg(debug_assertions)]
        const DEBUG: &str = "debug build";
        #[cfg(not(debug_assertions))]
        const DEBUG: &str = "release build";

        let max_label_width = reporters
            .iter()
            .map(|(label, reporter)| label.len() + reporter.last_string().1.len())
            .max()
            .unwrap_or(0)
            .max(7);
        let padding = " ".repeat(max_label_width - 7);
        let first = format!("Report {label} ({DEBUG})\n{padding}per second @ time per\n");

        let result = Some(first)
            .into_iter()
            .chain(reporters.iter().map(|(label, reporter)| {
                let (int, per_sec) = reporter.last_string();
                let padding = " ".repeat(max_label_width - label.len() - per_sec.len() + 1);
                format!("{label}: {padding}{per_sec} @ {int}\n")
            }))
            .collect();

        // reset all
        for (_, reporter) in reporters.into_iter() {
            reporter.reset();
        }

        result
    }

    pub fn reset(&mut self) {
        let avg = self.elapsed / self.count;
        let fps = self.count as f64 / self.report_interval.as_secs_f64();

        self.count = 0;
        self.elapsed = Duration::default();
        self.report_timer = Instant::now();
        self.last_interval = Some(avg);
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

impl Deref for Timer {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.begin
    }
}
