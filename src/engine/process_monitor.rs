//
// TurtKey - Process & Active Window Monitor
// Enumerates running processes for search/selection
// and detects currently focused foreground applications.
// Clean, professional architecture (Zero emojis)
//

use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<String>,
}

pub struct ProcessMonitor {
    sys: System,
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self {
            sys: System::new(),
        }
    }
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Refresh and enumerate all currently running processes
    pub fn list_running_processes(&mut self) -> Vec<ProcessInfo> {
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut list: Vec<ProcessInfo> = Vec::new();
        let mut seen_names = std::collections::HashSet::new();

        for (pid, proc_data) in self.sys.processes() {
            let raw_name = proc_data.name().to_string_lossy().to_string();
            let name = raw_name.trim().to_string();
            if name.is_empty() || seen_names.contains(&name) {
                continue;
            }

            seen_names.insert(name.clone());
            let exe_path = proc_data.exe().map(|p| p.to_string_lossy().to_string());

            list.push(ProcessInfo {
                pid: pid.as_u32(),
                name,
                exe_path,
            });
        }

        list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        list
    }

    /// Detect currently active/foreground window process name cross-platform
    #[allow(unreachable_code)]
    pub fn get_active_process_name() -> Option<String> {
        #[cfg(target_os = "windows")]
        {
            // Windows implementation via user32 / kernel32 can be queried here
            return None;
        }

        #[cfg(target_os = "linux")]
        {
            // Try reading X11 active window if xdotool or X11 environment is present
            if let Ok(output) = std::process::Command::new("xdotool")
                .args(["getactivewindow", "getwindowpid"])
                .output()
            {
                if output.status.success() {
                    let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        let comm_path = format!("/proc/{}/comm", pid);
                        if let Ok(comm) = std::fs::read_to_string(comm_path) {
                            return Some(comm.trim().to_string());
                        }
                    }
                }
            }
            return None;
        }

        None
    }
}
