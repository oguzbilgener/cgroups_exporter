use std::{collections::HashMap, sync::LazyLock};

use serde_prom::{MetricDescriptor, MetricType};

pub static METADATA: phf::Map<&'static str, MetricDescriptor<'static>> = phf::phf_map! {
    // PROCESS METRICS
    "rss" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Resident Set Size in bytes",
        labels: vec![],
        rename: None,
    },
    "utime" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "User CPU time in seconds",
        labels: vec![],
        rename: Some("utime_seconds_total"),
    },
    "stime" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "System CPU time in seconds",
        labels: vec![],
        rename: Some("stime_seconds_total"),
    },
    "cpu_seconds_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total CPU time in seconds",
        labels: vec![],
        rename: None,
    },
    "memory_usage_bytes" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Memory usage in bytes",
        labels: vec![],
        rename: None,
    },
    "num_fds" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Number of file descriptors",
        labels: vec![],
        rename: None,
    },
    "num_procs" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Number of processes",
        labels: vec![],
        rename: None,
    },
    "num_threads" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Number of threads",
        labels: vec![],
        rename: None,
    },
    "io_read_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of bytes read",
        labels: vec![],
        rename: None,
    },
    "io_write_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of bytes written",
        labels: vec![],
        rename: None,
    },
    "major_page_faults_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of major page faults",
        labels: vec![],
        rename: None,
    },
    "minor_page_faults_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of minor page faults",
        labels: vec![],
        rename: None,
    },
    "start_time" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Start time in seconds since epoch",
        labels: vec![],
        rename: None,
    },
    // CGROUP CPU METRICS
    "cpu_usage_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "CPU usage in microseconds",
        labels: vec![],
        rename: Some("cpu_usage_usec_total"),
    },
    "cpu_user_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "User CPU time in microseconds",
        labels: vec![],
        rename: Some("cpu_user_usec_total"),
    },
    "cpu_system_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "System CPU time in microseconds",
        labels: vec![],
        rename: Some("cpu_system_usec_total"),
    },
    "cpu_nice_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_nice_usec_total"),
    },
    "cpu_nr_periods" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_nr_periods_total"),
    },
    "cpu_nr_throttled" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_nr_throttled_total"),
    },
    "cpu_throttled_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_throttled_usec_total"),
    },
    "cpu_nr_bursts" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_nr_bursts_total"),
    },
    "cpu_burst_usec" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "",
        labels: vec![],
        rename: Some("cpu_burst_usec_total"),
    },
    // CGROUP MEMORY METRICS (cgroups_rs::memory::Memory)
    // Memory struct fields
    "memory_fail_cnt" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many times the limit has been hit.",
        labels: vec![],
        rename: Some("memory_fail_cnt_total"),
    },
    "memory_limit_in_bytes" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The limit in bytes of the memory usage of the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_usage_in_bytes" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The current usage of memory by the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_max_usage_in_bytes" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The maximum observed usage of memory by the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_move_charge_at_immigrate" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Whether moving charges at immigrate is allowed.",
        labels: vec![],
        rename: None,
    },
    // Nested: NumaStat fields (prefixed with "memory_numa_stat_")
    "memory_numa_stat_total_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of pages used by the control group.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_total_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of pages used by the control group, broken down by NUMA node.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_file_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of file pages used by the control group.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_file_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of file pages used by the control group, broken down by NUMA node.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_anon_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of anonymous pages used by the control group.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_anon_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of anonymous pages used by the control group, broken down by NUMA node.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_unevictable_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of unevictable pages used by the control group.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_unevictable_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total amount of unevictable pages used by the control group, broken down by NUMA node.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_total_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as total_pages, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_total_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as total_pages_per_node, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_file_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as file_pages, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_file_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as file_pages_per_node, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_anon_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as anon_pages, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_anon_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as anon_pages_per_node, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_unevictable_pages" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as unevictable, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    "memory_numa_stat_hierarchical_unevictable_pages_per_node" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Same as unevictable_per_node, but includes the descendant control groups' number as well.",
        labels: vec![],
        rename: None,
    },
    // Nested: OomControl fields (prefixed with "memory_oom_control_")
    "memory_oom_control_oom_kill_disable" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "If true, the OOM killer has been disabled for the tasks in this control group.",
        labels: vec![],
        rename: None,
    },
    "memory_oom_control_under_oom" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Is the OOM killer currently running for the tasks in the control group?",
        labels: vec![],
        rename: None,
    },
    "memory_oom_control_oom_kill" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many tasks were killed by the OOM killer so far.",
        labels: vec![],
        rename: Some("memory_oom_control_oom_kill_total"),
    },
    // Back to Memory struct fields
    "memory_soft_limit_in_bytes" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Allows setting a limit to memory usage which is enforced when the system detects memory pressure.",
        labels: vec![],
        rename: None,
    },
    // Nested: MemoryStat fields (prefixed with "memory_stat_")
    "memory_stat_cache" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Cache memory usage by the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_rss" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Resident set size memory usage.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_rss_huge" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Huge pages of resident set size memory usage.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_shmem" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Shared memory usage.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_mapped_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Mapped file memory usage.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_dirty" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Dirty pages count.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_writeback" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Pages in writeback.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_swap" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Swap usage.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_pgpgin" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of pages paged in.",
        labels: vec![],
        rename: Some("memory_stat_pgpgin_total"),
    },
    "memory_stat_pgpgout" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of pages paged out.",
        labels: vec![],
        rename: Some("memory_stat_pgpgout_total"),
    },
    "memory_stat_pgfault" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of page faults.",
        labels: vec![],
        rename: Some("memory_stat_pgfault_total"),
    },
    "memory_stat_pgmajfault" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of major page faults.",
        labels: vec![],
        rename: Some("memory_stat_pgmajfault_total"),
    },
    "memory_stat_inactive_anon" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Inactive anonymous pages.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_active_anon" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Active anonymous pages.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_inactive_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Inactive file pages.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_active_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Active file pages.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_unevictable" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Unevictable pages.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_hierarchical_memory_limit" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Hierarchical memory limit for the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_hierarchical_memsw_limit" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Hierarchical memory+swap limit for the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_cache" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total cache memory usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_rss" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total resident set size memory usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_rss_huge" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total huge pages of RSS memory usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_shmem" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total shared memory usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_mapped_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total mapped file memory usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_dirty" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total dirty pages count including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_writeback" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total writeback pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_swap" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total swap usage including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_pgpgin" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of pages paged in including descendant control groups.",
        labels: vec![],
        rename: Some("memory_stat_total_pgpgin_total"),
    },
    "memory_stat_total_pgpgout" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of pages paged out including descendant control groups.",
        labels: vec![],
        rename: Some("memory_stat_total_pgpgout_total"),
    },
    "memory_stat_total_pgfault" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of page faults including descendant control groups.",
        labels: vec![],
        rename: Some("memory_stat_total_pgfault_total"),
    },
    "memory_stat_total_pgmajfault" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of major page faults including descendant control groups.",
        labels: vec![],
        rename: Some("memory_stat_total_pgmajfault_total"),
    },
    "memory_stat_total_inactive_anon" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total inactive anonymous pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_active_anon" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total active anonymous pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_inactive_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total inactive file pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_active_file" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total active file pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    "memory_stat_total_unevictable" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Total unevictable pages including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    // Final Memory struct fields
    "memory_swappiness" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "Set the tendency of the kernel to swap out parts of the address space consumed by the control group's tasks.",
        labels: vec![],
        rename: None,
    },
    "memory_use_hierarchy" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "If set, under OOM conditions the kernel will try to reclaim memory from the children of the offending process.",
        labels: vec![],
        rename: None,
    },
    // CGROUP BLKIO METRICS
    // BlkIo struct fields

    // Field: io_merged: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_merged_"
    "blkio_io_merged_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_merged_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_merged_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_merged_read_total"),
    },
    "blkio_io_merged_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_merged_write_total"),
    },
    "blkio_io_merged_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_sync_total"),
    },
    "blkio_io_merged_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_async_total"),
    },
    "blkio_io_merged_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_merged_discard_total"),
    },
    "blkio_io_merged_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_total"),
    },
    // Field: io_merged_total: u64
    "blkio_io_merged_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_merged`, but only reports the total number.",
        labels: vec![],
        rename: Some("blkio_io_merged_total"),
    },
    // Field: io_merged_recursive: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_merged_recursive_"
    "blkio_io_merged_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_merged_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_merged_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_read_total"),
    },
    "blkio_io_merged_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_write_total"),
    },
    "blkio_io_merged_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_sync_total"),
    },
    "blkio_io_merged_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_async_total"),
    },
    "blkio_io_merged_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_discard_total"),
    },
    "blkio_io_merged_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_total"),
    },
// Field: io_merged_recursive_total: u64

    "blkio_io_merged_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_merged_recursive`, but only reports the total number.",
        labels: vec![],
        rename: Some("blkio_io_merged_recursive_total"),
    },
    // Field: io_queued: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_queued_"
    "blkio_io_queued_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_queued_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_queued_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_queued_read_total"),
    },
    "blkio_io_queued_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_queued_write_total"),
    },
    "blkio_io_queued_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_sync_total"),
    },
    "blkio_io_queued_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_async_total"),
    },
    "blkio_io_queued_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_queued_discard_total"),
    },
    "blkio_io_queued_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_total"),
    },
    // Field: io_queued_total: u64
    "blkio_io_queued_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_queued`, but only reports the total number.",
        labels: vec![],
        rename: Some("blkio_io_queued_total"),
    },
    // Field: io_queued_recursive: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_queued_recursive_"
    "blkio_io_queued_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_queued_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_queued_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_read_total"),
    },
    "blkio_io_queued_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_write_total"),
    },
    "blkio_io_queued_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_sync_total"),
    },
    "blkio_io_queued_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_async_total"),
    },
    "blkio_io_queued_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_discard_total"),
    },
    "blkio_io_queued_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_total"),
    },
    // Field: io_queued_recursive_total: u64
    "blkio_io_queued_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_queued_recursive`, but only reports the total number.",
        labels: vec![],
        rename: Some("blkio_io_queued_recursive_total"),
    },
    // Field: io_service_bytes: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_service_bytes_"
    "blkio_io_service_bytes_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_bytes_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_bytes_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_read_total"),
    },
    "blkio_io_service_bytes_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_write_total"),
    },
    "blkio_io_service_bytes_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_sync_total"),
    },
    "blkio_io_service_bytes_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_async_total"),
    },
    "blkio_io_service_bytes_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_discard_total"),
    },
    "blkio_io_service_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_total"),
    },
    // Field: io_service_bytes_total: u64
    "blkio_io_service_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_service_bytes`, but only reports the total number.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_total"),
    },
    // Field: io_service_bytes_recursive: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_service_bytes_recursive_"
    "blkio_io_service_bytes_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_bytes_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_bytes_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_read_total"),
    },
    "blkio_io_service_bytes_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_write_total"),
    },
    "blkio_io_service_bytes_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_sync_total"),
    },
    "blkio_io_service_bytes_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_async_total"),
    },
    "blkio_io_service_bytes_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_discard_total"),
    },
    "blkio_io_service_bytes_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_total"),
    },
    // Field: io_service_bytes_recursive_total: u64
    "blkio_io_service_bytes_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total amount of bytes transferred between the tasks and block devices, including descendant control groups.",
        labels: vec![],
        rename: Some("blkio_io_service_bytes_recursive_total"),
    },
    // Field: io_serviced: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_serviced_"
    "blkio_io_serviced_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_serviced_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_serviced_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_serviced_read_total"),
    },
    "blkio_io_serviced_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_serviced_write_total"),
    },
    "blkio_io_serviced_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_serviced_sync_total"),
    },
    "blkio_io_serviced_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_serviced_async_total"),
    },
    "blkio_io_serviced_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_serviced_discard_total"),
    },
    "blkio_io_serviced_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_serviced_total"),
    },
    // Field: io_serviced_total: u64
    "blkio_io_serviced_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "The total number of I/O operations performed on the devices as seen by the throttling policy.",
        labels: vec![],
        rename: Some("blkio_io_serviced_total"),
    },
    // Field: io_serviced_recursive: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_serviced_recursive_"
    "blkio_io_serviced_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_serviced_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_serviced_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_read_total"),
    },
    "blkio_io_serviced_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_write_total"),
    },
    "blkio_io_serviced_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_sync_total"),
    },
    "blkio_io_serviced_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_async_total"),
    },
    "blkio_io_serviced_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_discard_total"),
    },
    "blkio_io_serviced_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_serviced`, but contains all descendant control groups and only the total amount.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_total"),
    },
    // Field: io_serviced_recursive_total: u64
    "blkio_io_serviced_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_serviced_recursive`, but contains all descendant control groups and only the total amount.",
        labels: vec![],
        rename: Some("blkio_io_serviced_recursive_total"),
    },
    // Field: io_service_time: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_service_time_"
    "blkio_io_service_time_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_time_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_time_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_service_time_read_total"),
    },
    "blkio_io_service_time_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_service_time_write_total"),
    },
    "blkio_io_service_time_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_time_sync_total"),
    },
    "blkio_io_service_time_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_time_async_total"),
    },
    "blkio_io_service_time_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_service_time_discard_total"),
    },
    "blkio_io_service_time_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total time spent between dispatch and completion for I/O requests.",
        labels: vec![],
        rename: Some("blkio_io_service_time_total"),
    },
    // Field: io_service_time_total: u64
    "blkio_io_service_time_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_service_time`, but reports only the total amount.",
        labels: vec![],
        rename: Some("blkio_io_service_time_total"),
    },
    // Field: io_service_time_recursive: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_service_time_recursive_"
    "blkio_io_service_time_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_time_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_service_time_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_read_total"),
    },
    "blkio_io_service_time_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_write_total"),
    },
    "blkio_io_service_time_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_sync_total"),
    },
    "blkio_io_service_time_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_async_total"),
    },
    "blkio_io_service_time_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_discard_total"),
    },
    "blkio_io_service_time_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_service_time_recursive`, but reports only the total amount.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_total"),
    },
    // Field: io_service_time_recursive_total: u64
    "blkio_io_service_time_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_service_time_recursive`, but reports only the total amount.",
        labels: vec![],
        rename: Some("blkio_io_service_time_recursive_total"),
    },
    // Field: io_wait_time: Vec<IoService>
    // Nested IoService fields prefixed with "blkio_io_wait_time_"
    "blkio_io_wait_time_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_wait_time_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_wait_time_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_read_total"),
    },
    "blkio_io_wait_time_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_write_total"),
    },
    "blkio_io_wait_time_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_sync_total"),
    },
    "blkio_io_wait_time_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_async_total"),
    },
    "blkio_io_wait_time_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_discard_total"),
    },
    "blkio_io_wait_time_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of items transferred.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_total"),
    },
    // Field: io_wait_time_total: u64
    "blkio_io_wait_time_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_wait_time`, but only reports the total amount.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_total"),
    },
    // Field: io_wait_time_recursive: Vec<IoService)
    // Nested IoService fields prefixed with "blkio_io_wait_time_recursive_"
    "blkio_io_wait_time_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_wait_time_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_io_wait_time_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_read_total"),
    },
    "blkio_io_wait_time_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_write_total"),
    },
    "blkio_io_wait_time_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were synchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_sync_total"),
    },
    "blkio_io_wait_time_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were asynchronously transferred.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_async_total"),
    },
    "blkio_io_wait_time_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many items were discarded.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_discard_total"),
    },
    "blkio_io_wait_time_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_wait_time_recursive`, but only reports the total amount.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_total"),
    },
    // Field: io_wait_time_recursive_total: u64
    "blkio_io_wait_time_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Same as `io_wait_time_recursive`, but only reports the total amount.",
        labels: vec![],
        rename: Some("blkio_io_wait_time_recursive_total"),
    },
    // Field: leaf_weight: u64
    "blkio_leaf_weight" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "How much weight the control group's tasks have when competing against descendant control groups.",
        labels: vec![],
        rename: None,
    },
    // Field: leaf_weight_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_leaf_weight_device_"
    "blkio_leaf_weight_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_leaf_weight_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_leaf_weight_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The data associated with the device.",
        labels: vec![],
        rename: None,
    },
    // Field: sectors: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_sectors_"
    "blkio_sectors_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_sectors_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_sectors_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The sector count transferred.",
        labels: vec![],
        rename: None,
    },
    // Field: sectors_recursive: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_sectors_recursive_"
    "blkio_sectors_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_sectors_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device.",
        labels: vec![],
        rename: None,
    },
    "blkio_sectors_recursive_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The sector count transferred, including descendant control groups.",
        labels: vec![],
        rename: None,
    },
    // Field: throttle: BlkIoThrottle
    // Nested BlkIoThrottle fields prefixed with "blkio_throttle_"
    // Field: throttle.io_service_bytes: Vec<IoService>
    "blkio_throttle_io_service_bytes_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (throttle io_service_bytes).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_service_bytes_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (throttle io_service_bytes).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_service_bytes_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Bytes transferred (read) as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_read_total"),
    },
    "blkio_throttle_io_service_bytes_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Bytes transferred (write) as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_write_total"),
    },
    "blkio_throttle_io_service_bytes_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Synchronous bytes transferred as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_sync_total"),
    },
    "blkio_throttle_io_service_bytes_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Asynchronous bytes transferred as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_async_total"),
    },
    "blkio_throttle_io_service_bytes_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Discarded bytes as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_discard_total"),
    },
    "blkio_throttle_io_service_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total bytes transferred as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_total"),
    },
    // Field: throttle.io_service_bytes_total: u64
    "blkio_throttle_io_service_bytes_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total amount of bytes transferred as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_total"),
    },
    // Field: throttle.io_service_bytes_recursive: Vec<IoService>
    "blkio_throttle_io_service_bytes_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (throttle io_service_bytes_recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_service_bytes_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (throttle io_service_bytes_recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_service_bytes_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Bytes transferred (read) recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_read_total"),
    },
    "blkio_throttle_io_service_bytes_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Bytes transferred (write) recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_write_total"),
    },
    "blkio_throttle_io_service_bytes_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Synchronous bytes transferred recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_sync_total"),
    },
    "blkio_throttle_io_service_bytes_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Asynchronous bytes transferred recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_async_total"),
    },
    "blkio_throttle_io_service_bytes_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Discarded bytes recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_discard_total"),
    },
    "blkio_throttle_io_service_bytes_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total recursive bytes transferred as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_total"),
    },
    // Field: throttle.io_service_bytes_recursive_total: u64
    "blkio_throttle_io_service_bytes_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total amount of bytes transferred recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_service_bytes_recursive_total"),
    },
    // Field: throttle.io_serviced: Vec<IoService>
    "blkio_throttle_io_serviced_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (throttle io_serviced).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_serviced_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (throttle io_serviced).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_serviced_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of I/O operations read as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_read_total"),
    },
    "blkio_throttle_io_serviced_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of I/O operations written as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_write_total"),
    },
    "blkio_throttle_io_serviced_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Synchronous I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_sync_total"),
    },
    "blkio_throttle_io_serviced_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Asynchronous I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_async_total"),
    },
    "blkio_throttle_io_serviced_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Discarded I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_discard_total"),
    },
    "blkio_throttle_io_serviced_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_total"),
    },
    // Field: throttle.io_serviced_total: u64
    "blkio_throttle_io_serviced_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "The total number of I/O operations performed as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_total"),
    },
    // Field: throttle.io_serviced_recursive: Vec<IoService>
    "blkio_throttle_io_serviced_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (throttle io_serviced_recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_serviced_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (throttle io_serviced_recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_io_serviced_recursive_read" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of I/O operations read recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_read_total"),
    },
    "blkio_throttle_io_serviced_recursive_write" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Number of I/O operations written recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_write_total"),
    },
    "blkio_throttle_io_serviced_recursive_sync" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Synchronous I/O operations recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_sync_total"),
    },
    "blkio_throttle_io_serviced_recursive_async" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Asynchronous I/O operations recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_async_total"),
    },
    "blkio_throttle_io_serviced_recursive_discard" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Discarded I/O operations recursively as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_discard_total"),
    },
    "blkio_throttle_io_serviced_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total recursive I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_total"),
    },
    // Field: throttle.io_serviced_recursive_total: u64
    "blkio_throttle_io_serviced_recursive_total" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "Total number of recursive I/O operations as seen by the throttle policy.",
        labels: vec![],
        rename: Some("blkio_throttle_io_serviced_recursive_total"),
    },
    // Field: throttle.read_bps_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_throttle_read_bps_device_"
    "blkio_throttle_read_bps_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (read BPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_read_bps_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (read BPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_read_bps_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The upper limit of bytes per second rate of read operations.",
        labels: vec![],
        rename: None,
    },
    // Field: throttle.read_iops_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_throttle_read_iops_device_"
    "blkio_throttle_read_iops_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (read IOPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_read_iops_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (read IOPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_read_iops_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The upper limit of I/O operations per second for read operations.",
        labels: vec![],
        rename: None,
    },
    // Field: throttle.write_bps_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_throttle_write_bps_device_"
    "blkio_throttle_write_bps_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (write BPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_write_bps_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (write BPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_write_bps_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The upper limit of bytes per second rate of write operations.",
        labels: vec![],
        rename: None,
    },
    // Field: throttle.write_iops_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_throttle_write_iops_device_"
    "blkio_throttle_write_iops_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (write IOPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_write_iops_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (write IOPS).",
        labels: vec![],
        rename: None,
    },
    "blkio_throttle_write_iops_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The upper limit of I/O operations per second for write operations.",
        labels: vec![],
        rename: None,
    },
    // End of throttle fields
    // Field: time: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_time_"
    "blkio_time_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (time).",
        labels: vec![],
        rename: None,
    },
    "blkio_time_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (time).",
        labels: vec![],
        rename: None,
    },
    "blkio_time_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The total time the control group had access to the I/O devices.",
        labels: vec![],
        rename: None,
    },
    // Field: time_recursive: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_time_recursive_"
    "blkio_time_recursive_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (time recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_time_recursive_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (time recursive).",
        labels: vec![],
        rename: None,
    },
    "blkio_time_recursive_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The total time the control group had access to the I/O devices, including descendants.",
        labels: vec![],
        rename: None,
    },
    // Field: weight: u64
    "blkio_weight" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The weight of this control group.",
        labels: vec![],
        rename: None,
    },
    // Field: weight_device: Vec<BlkIoData>
    // Nested BlkIoData fields prefixed with "blkio_weight_device_"
    "blkio_weight_device_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (weight).",
        labels: vec![],
        rename: None,
    },
    "blkio_weight_device_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (weight).",
        labels: vec![],
        rename: None,
    },
    "blkio_weight_device_data" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The weight data associated with the device.",
        labels: vec![],
        rename: None,
    },
    // Field: io_stat: Vec<IoStat>
    // Nested IoStat fields prefixed with "blkio_io_stat_"
    "blkio_io_stat_major" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The major number of the device (io_stat).",
        labels: vec![],
        rename: None,
    },
    "blkio_io_stat_minor" => MetricDescriptor {
        metric_type: MetricType::Gauge,
        help: "The minor number of the device (io_stat).",
        labels: vec![],
        rename: None,
    },
    "blkio_io_stat_rbytes" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many bytes were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_rbytes_total"),
    },
    "blkio_io_stat_wbytes" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many bytes were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_wbytes_total"),
    },
    "blkio_io_stat_rios" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many IOPS were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_rios_total"),
    },
    "blkio_io_stat_wios" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many IOPS were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_wios_total"),
    },
    "blkio_io_stat_dbytes" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many discard bytes were read from the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_dbytes_total"),
    },
    "blkio_io_stat_dios" => MetricDescriptor {
        metric_type: MetricType::Counter,
        help: "How many discard IOPS were written to the device.",
        labels: vec![],
        rename: Some("blkio_io_stat_dios_total"),
    },
};
