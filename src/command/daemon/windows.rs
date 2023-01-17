use crate::command::daemon::backgroun_task::BackgroundTask;

pub struct WindowsBackgroundTask;

#[cfg(windows)]
impl BackgroundTask for WindowsBackgroundTask {
    fn execute(&self) -> () {
        log::warn!("Running as a background Windows service is not supported yet");
        println!(
            "Let the server instance run in the current console window or press Ctrl+C to exit"
        );
    }
}
