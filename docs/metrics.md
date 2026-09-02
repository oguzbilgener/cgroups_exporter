# Metrics

## Process metrics

The following metrics are generated for captured process groups, as well as for the processes in a cgroup.

In this table, the default namespace, `process` is used. If you specify a different namespace in the configuration, the metrics will be prefixed with that namespace. For cgroups, the namespace is `cgroup`.

| Metric Name                     | Type    | Help                              |
| ------------------------------- | ------- | --------------------------------- |
| process_rss                     | gauge   | Resident Set Size in bytes        |
| process_utime_seconds_total     | counter | User CPU time in seconds          |
| process_stime_seconds_total     | counter | System CPU time in seconds        |
| process_cpu_seconds_total       | counter | Total CPU time in seconds         |
| process_memory_usage_bytes      | gauge   | Memory usage in bytes             |
| process_num_fds                 | gauge   | Number of file descriptors        |
| process_num_procs               | gauge   | Number of processes               |
| process_num_threads             | gauge   | Number of threads                 |
| process_io_read_bytes_total     | counter | Number of bytes read              |
| process_io_write_bytes_total    | counter | Number of bytes written           |
| process_major_page_faults_total | counter | Number of major page faults       |
| process_minor_page_faults_total | counter | Number of minor page faults       |
| process_start_time              | gauge   | Start time in seconds since epoch |

## Cgroup metrics

| Metric Name                                            | Type    | Help                                                                                                           |
| ------------------------------------------------------ | ------- | -------------------------------------------------------------------------------------------------------------- |
| cgroup_cpu_usage_usec_total                            | counter | CPU usage in microseconds                                                                                      |
| cgroup_cpu_user_usec_total                             | counter | User CPU time in microseconds                                                                                  |
| cgroup_cpu_system_usec_total                           | counter | System CPU time in microseconds                                                                                |
| cgroup_cpu_nice_usec_total                             | counter |                                                                                                                |
| cgroup_cpu_nr_periods_total                            | counter |                                                                                                                |
| cgroup_cpu_nr_throttled_total                          | counter |                                                                                                                |
| cgroup_cpu_throttled_usec_total                        | counter |                                                                                                                |
| cgroup_cpu_nr_bursts_total                             | counter |                                                                                                                |
| cgroup_cpu_burst_usec_total                            | counter |                                                                                                                |
| cgroup_memory_fail_cnt_total                           | counter | How many times the limit has been hit.                                                                         |
| cgroup_memory_limit_in_bytes                           | gauge   | The limit in bytes of the memory usage of the control group's tasks.                                           |
| cgroup_memory_usage_in_bytes                           | gauge   | The current usage of memory by the control group's tasks.                                                      |
| cgroup_memory_max_usage_in_bytes                       | gauge   | The maximum observed usage of memory by the control group's tasks.                                             |
| cgroup_memory_move_charge_at_immigrate                 | gauge   | Whether moving charges at immigrate is allowed.                                                                |
| cgroup_memory_numa_stat_total_pages                    | gauge   | Total amount of pages used by the control group.                                                               |
| cgroup_memory_numa_stat_file_pages                     | gauge   | Total amount of file pages used by the control group.                                                          |
| cgroup_memory_numa_stat_anon_pages                     | gauge   | Total amount of anonymous pages used by the control group.                                                     |
| cgroup_memory_numa_stat_unevictable_pages              | gauge   | Total amount of unevictable pages used by the control group.                                                   |
| cgroup_memory_numa_stat_hierarchical_total_pages       | gauge   | Same as total_pages, but includes the descendant control groups' number as well.                               |
| cgroup_memory_numa_stat_hierarchical_file_pages        | gauge   | Same as file_pages, but includes the descendant control groups' number as well.                                |
| cgroup_memory_numa_stat_hierarchical_anon_pages        | gauge   | Same as anon_pages, but includes the descendant control groups' number as well.                                |
| cgroup_memory_numa_stat_hierarchical_unevictable_pages | gauge   | Same as unevictable, but includes the descendant control groups' number as well.                               |
| cgroup_memory_oom_control_oom_kill_disable             | gauge   | If true, the OOM killer has been disabled for the tasks in this control group.                                 |
| cgroup_memory_oom_control_under_oom                    | gauge   | Is the OOM killer currently running for the tasks in the control group?                                        |
| cgroup_memory_oom_control_oom_kill_total               | counter | How many tasks were killed by the OOM killer so far.                                                           |
| cgroup_memory_soft_limit_in_bytes                      | gauge   | Allows setting a limit to memory usage which is enforced when the system detects memory pressure.              |
| cgroup_memory_stat_cache                               | gauge   | Cache memory usage by the control group's tasks.                                                               |
| cgroup_memory_stat_rss                                 | gauge   | Resident set size memory usage.                                                                                |
| cgroup_memory_stat_rss_huge                            | gauge   | Huge pages of resident set size memory usage.                                                                  |
| cgroup_memory_stat_shmem                               | gauge   | Shared memory usage.                                                                                           |
| cgroup_memory_stat_mapped_file                         | gauge   | Mapped file memory usage.                                                                                      |
| cgroup_memory_stat_dirty                               | gauge   | Dirty pages count.                                                                                             |
| cgroup_memory_stat_writeback                           | gauge   | Pages in writeback.                                                                                            |
| cgroup_memory_stat_swap                                | gauge   | Current swap usage in bytes. On cgroup v2, this is sourced from `memory.swap.current` and includes descendants. |
| cgroup_memory_stat_pgpgin_total                        | counter | Number of pages paged in.                                                                                      |
| cgroup_memory_stat_pgpgout_total                       | counter | Number of pages paged out.                                                                                     |
| cgroup_memory_stat_pgfault_total                       | counter | Number of page faults.                                                                                         |
| cgroup_memory_stat_pgmajfault_total                    | counter | Number of major page faults.                                                                                   |
| cgroup_memory_stat_inactive_anon                       | gauge   | Inactive anonymous pages.                                                                                      |
| cgroup_memory_stat_active_anon                         | gauge   | Active anonymous pages.                                                                                        |
| cgroup_memory_stat_inactive_file                       | gauge   | Inactive file pages.                                                                                           |
| cgroup_memory_stat_active_file                         | gauge   | Active file pages.                                                                                             |
| cgroup_memory_stat_unevictable                         | gauge   | Unevictable pages.                                                                                             |
| cgroup_memory_stat_hierarchical_memory_limit           | gauge   | Hierarchical memory limit for the control group's tasks.                                                       |
| cgroup_memory_stat_hierarchical_memsw_limit            | gauge   | Hierarchical memory+swap limit for the control group's tasks.                                                  |
| cgroup_memory_stat_total_cache                         | gauge   | Total cache memory usage including descendant control groups.                                                  |
| cgroup_memory_stat_total_rss                           | gauge   | Total resident set size memory usage including descendant control groups.                                      |
| cgroup_memory_stat_total_rss_huge                      | gauge   | Total huge pages of RSS memory usage including descendant control groups.                                      |
| cgroup_memory_stat_total_shmem                         | gauge   | Total shared memory usage including descendant control groups.                                                 |
| cgroup_memory_stat_total_mapped_file                   | gauge   | Total mapped file memory usage including descendant control groups.                                            |
| cgroup_memory_stat_total_dirty                         | gauge   | Total dirty pages count including descendant control groups.                                                   |
| cgroup_memory_stat_total_writeback                     | gauge   | Total writeback pages including descendant control groups.                                                     |
| cgroup_memory_stat_total_swap                          | gauge   | Total swap usage including descendant control groups.                                                          |
| cgroup_memory_stat_total_pgpgin_total                  | counter | Total number of pages paged in including descendant control groups.                                            |
| cgroup_memory_stat_total_pgpgout_total                 | counter | Total number of pages paged out including descendant control groups.                                           |
| cgroup_memory_stat_total_pgfault_total                 | counter | Total number of page faults including descendant control groups.                                               |
| cgroup_memory_stat_total_pgmajfault_total              | counter | Total number of major page faults including descendant control groups.                                         |
| cgroup_memory_stat_total_inactive_anon                 | gauge   | Total inactive anonymous pages including descendant control groups.                                            |
| cgroup_memory_stat_total_active_anon                   | gauge   | Total active anonymous pages including descendant control groups.                                              |
| cgroup_memory_stat_total_inactive_file                 | gauge   | Total inactive file pages including descendant control groups.                                                 |
| cgroup_memory_stat_total_active_file                   | gauge   | Total active file pages including descendant control groups.                                                   |
| cgroup_memory_stat_total_unevictable                   | gauge   | Total unevictable pages including descendant control groups.                                                   |
| cgroup_memory_swappiness                               | gauge   | Set the tendency of the kernel to swap out memory. Available on cgroup v1 only.                                 |
| cgroup_memory_use_hierarchy                            | gauge   | If set, under OOM conditions the kernel will try to reclaim memory from the children of the offending process. |
| cgroup_memory_memsw_fail_cnt_total                     | counter | How many times the memory+swap limit has been hit.                                                             |
| cgroup_memory_memsw_limit_in_bytes                     | gauge   | The limit in bytes of the memory+swap usage of the control group's tasks.                                      |
| cgroup_memory_memsw_usage_in_bytes                     | gauge   | The current usage of memory+swap by the control group's tasks.                                                 |
| cgroup_memory_memsw_max_usage_in_bytes                 | gauge   | The maximum observed usage of memory+swap by the control group's tasks.                                        |
| cgroup_blkio_io_merged_total                           | counter | Same as `io_merged`, but only reports the total number.                                                        |
| cgroup_blkio_io_merged_recursive_total                 | counter | Same as `io_merged_recursive`, but only reports the total number.                                              |
| cgroup_blkio_io_queued_total                           | counter | Same as `io_queued`, but only reports the total number.                                                        |
| cgroup_blkio_io_queued_recursive_total                 | counter | Same as `io_queued_recursive`, but only reports the total number.                                              |
| cgroup_blkio_io_service_bytes_total                    | counter | Same as `io_service_bytes`, but only reports the total number.                                                 |
| cgroup_blkio_io_service_bytes_recursive_total          | counter | Total amount of bytes transferred between the tasks and block devices, including descendant control groups.    |
| cgroup_blkio_io_serviced_total                         | counter | The total number of I/O operations performed on the devices as seen by the throttling policy.                  |
| cgroup_blkio_io_serviced_recursive_total               | counter | Same as `io_serviced_recursive`, but contains all descendant control groups and only the total amount.         |
| cgroup_blkio_io_service_time_total                     | counter | Same as `io_service_time`, but reports only the total amount.                                                  |
| cgroup_blkio_io_service_time_recursive_total           | counter | Same as `io_service_time_recursive`, but reports only the total amount.                                        |
| cgroup_blkio_io_wait_time_total                        | counter | Same as `io_wait_time`, but only reports the total amount.                                                     |
| cgroup_blkio_io_wait_time_recursive_total              | counter | Same as `io_wait_time_recursive`, but only reports the total amount.                                           |
| cgroup_blkio_leaf_weight                               | gauge   | How much weight the control group's tasks have when competing against descendant control groups.               |
| cgroup_blkio_throttle_io_service_bytes_total           | counter | Total amount of bytes transferred as seen by the throttle policy.                                              |
| cgroup_blkio_throttle_io_service_bytes_recursive_total | counter | Total amount of bytes transferred recursively as seen by the throttle policy.                                  |
| cgroup_blkio_throttle_io_serviced_total                | counter | The total number of I/O operations performed as seen by the throttle policy.                                   |
| cgroup_blkio_throttle_io_serviced_recursive_total      | counter | Total number of recursive I/O operations as seen by the throttle policy.                                       |
| cgroup_blkio_weight                                    | gauge   | The weight of this control group.                                                                              |
| cgroup_rss                                             | gauge   | Resident Set Size in bytes                                                                                     |
| cgroup_utime_seconds_total                             | counter | User CPU time in seconds                                                                                       |
| cgroup_stime_seconds_total                             | counter | System CPU time in seconds                                                                                     |
| cgroup_cpu_seconds_total                               | counter | Total CPU time in seconds                                                                                      |
| cgroup_memory_usage_bytes                              | gauge   | Memory usage in bytes                                                                                          |
| cgroup_num_fds                                         | gauge   | Number of file descriptors                                                                                     |
| cgroup_num_procs                                       | gauge   | Number of processes                                                                                            |
| cgroup_num_threads                                     | gauge   | Number of threads                                                                                              |
| cgroup_io_read_bytes_total                             | counter | Number of bytes read                                                                                           |
| cgroup_io_write_bytes_total                            | counter | Number of bytes written                                                                                        |
| cgroup_major_page_faults_total                         | counter | Number of major page faults                                                                                    |
| cgroup_minor_page_faults_total                         | counter | Number of minor page faults                                                                                    |
