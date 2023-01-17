use crate::command::daemon::backgroun_task::BackgroundTask;

pub struct UnixBackgroundTask;

impl BackgroundTask for UnixBackgroundTask {
    fn execute(&self) -> () {
        #[cfg(unix)]
        {
            use crate::util;
            use daemonize::Daemonize;
            use log::error;
            use log::info;
            use std::env;
            use std::fs::OpenOptions;
            use util::{PID_FILE, STDERR_LOGFILE, STDOUT_LOGFILE};

            let temp_dir = env::temp_dir();
            let stdout_logfile = temp_dir.join(STDOUT_LOGFILE);
            let stderr_logfile = temp_dir.join(STDERR_LOGFILE);
            let pid_file = temp_dir.join(PID_FILE);

            let stdout = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(stdout_logfile)
                .unwrap();
            let stderr = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(stderr_logfile)
                .unwrap();

            let daemonize = Daemonize::new()
                .pid_file(pid_file)
                .chown_pid_file(true)
                .working_directory(temp_dir)
                .stdout(stdout)
                .stderr(stderr);

            // Start the daemon.
            if let Err(e) = daemonize.start() {
                error!("Error daemonzing process:, {}", e);
            } else {
                info!("Daemonized successfully");
            }
        }
    }
}
