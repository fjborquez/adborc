use crate::command::daemon::backgroun_task::BackgroundTask;
#[cfg(unix)]
use crate::command::daemon::unix::UnixBackgroundTask;
#[cfg(windows)]
use crate::command::daemon::windows::WindowsBackgroundTask;

pub fn init_background_task() -> () {
    // Daemonize the thread, if possible.
    #[cfg(windows)]
    {
        let windows_daemon = WindowsBackgroundTask;
        windows_daemon.execute();
    }

    #[cfg(unix)]
    {
        let unix_daemon = UnixBackgroundTask;
        unix_daemon.execute();
    }
}
