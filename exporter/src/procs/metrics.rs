use super::Proc;
use crate::render::Named;
use procfs::WithCurrentSystemInfo as _;
use saturating_cast::SaturatingCast as _;
use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
pub struct ProcessMetrics {
    #[serde(skip)]
    pub name: String,

    pub rss: u64,
    pub utime: f64,
    pub stime: f64,
    pub cpu_seconds_total: f64,
    pub memory_usage_bytes: u64,
    pub num_fds: u64,
    pub num_procs: u64,
    pub num_threads: u64,
    pub io_read_bytes_total: u64,
    pub io_write_bytes_total: u64,
    pub major_page_faults_total: u64,
    pub minor_page_faults_total: u64,
    pub start_time: Option<i64>,
}

impl ProcessMetrics {
    #[allow(clippy::similar_names)]
    pub fn from_processes(proc_iter: impl Iterator<Item = Proc>, name: &str) -> Self {
        let name = name.to_string();
        let mut metrics = ProcessMetrics {
            name,
            ..Default::default()
        };

        let mut sum_rss_of_procs = 0;
        let mut sum_utime = 0f64;
        let mut sum_stime = 0f64;
        let mut sum_io_read_bytes = 0;
        let mut sum_io_write_bytes = 0;
        let mut sum_fds = 0;
        let mut sum_procs = 0;
        let mut sum_threads = 0;
        let mut sum_majflt = 0;
        let mut sum_minflt = 0;
        let mut min_start_time = None;
        let page_size = procfs::page_size();
        let clock_tick = procfs::ticks_per_second() as f64;

        for process in proc_iter {
            let stat = process.stat();
            sum_utime += stat.utime as f64 / clock_tick;
            sum_stime += stat.stime as f64 / clock_tick;
            sum_rss_of_procs += stat.rss * page_size;
            sum_threads += stat.num_threads.saturating_cast::<u64>();
            sum_majflt += stat.majflt;
            sum_minflt += stat.minflt;
            if let Ok(start_time) = stat.starttime().get().map(|t| t.timestamp()) {
                min_start_time = Some(min_start_time.unwrap_or(i64::MAX).min(start_time));
            }

            if let Some(io_stat) = process.io() {
                sum_io_read_bytes += io_stat.read_bytes;
                sum_io_write_bytes += io_stat.write_bytes;
            }
            if let Some(fd_count) = process.fd_count() {
                sum_fds += *fd_count as u64;
            }
            sum_procs += 1;
        }
        metrics.rss = sum_rss_of_procs;
        metrics.memory_usage_bytes = sum_rss_of_procs;
        metrics.utime = sum_utime;
        metrics.stime = sum_stime;
        metrics.cpu_seconds_total = sum_utime + sum_stime;
        metrics.num_fds = sum_fds;
        metrics.num_procs = sum_procs;
        metrics.num_threads = sum_threads;
        metrics.io_read_bytes_total = sum_io_read_bytes;
        metrics.io_write_bytes_total = sum_io_write_bytes;
        metrics.major_page_faults_total = sum_majflt;
        metrics.minor_page_faults_total = sum_minflt;
        metrics.start_time = min_start_time;
        metrics
    }
}

impl Named for ProcessMetrics {
    fn name(&self) -> &str {
        &self.name
    }
}
