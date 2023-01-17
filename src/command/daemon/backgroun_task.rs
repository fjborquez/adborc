pub trait BackgroundTask {
    fn execute(&self) -> ();
}
