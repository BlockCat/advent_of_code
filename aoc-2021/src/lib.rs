pub mod vector;
pub mod direction;
pub mod grid;




pub fn stopwatch<F>(action: F) -> Result<std::time::Duration, std::time::SystemTimeError> where F: FnOnce() {
    use std::time;
    let start = time::SystemTime::now();
    action();
    let end = time::SystemTime::now();

    end.duration_since(start)
}