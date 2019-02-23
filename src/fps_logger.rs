use log::*;

use crate::*;

/// Logs the current FPS (with the `log` crate) at a specified interval.
pub struct FpsLogger {
    start_time: f64,
    prev_time: f64,
    seconds_between_logs: f64,
    next_log_time: f64,
    total_frames: i64,
    frames: i64,
    last_fps: i32,
    stopwatch: Stopwatch,
}

impl FpsLogger {
    /// Creates a new `FpsLogger`. `seconds_between_logs` is the interval at which it logs the FPS.
    pub fn new(seconds_between_logs: f64) -> Self {
        let stopwatch = Stopwatch::new();
        let start_time = stopwatch.get_time();
        Self {
            start_time,
            prev_time: start_time,
            seconds_between_logs,
            next_log_time: seconds_between_logs,
            total_frames: 0,
            frames: 0,
            last_fps: 0,
            stopwatch,
        }
    }

    /// Updates the FPS logger and logs the current FPS every `seconds_between_logs`.
    /// Should be called once per frame.
    pub fn update(&mut self) {
        self.frames += 1;
        self.total_frames += 1;
        let time = self.stopwatch.get_time();
        if time - self.start_time >= self.next_log_time {
            let fps = (self.frames as f64 / (time - self.prev_time)).floor() as i64;
            self.last_fps = fps as i32;
            info!(
                "FPS: {}; ms/frame: {:.2}; average FPS: {}",
                fps,
                (time - self.prev_time) * 1000.0 / self.frames as f64,
                (self.total_frames as f64 / (time - self.start_time)).floor() as i64
            );
            self.prev_time = time;
            self.next_log_time += self.seconds_between_logs;
            self.frames = 0;
        }
    }

    /// Returns the most recently computed FPS. This is updated every time the FPS is logged.
    pub fn last_fps(&self) -> i32 {
        self.last_fps
    }
}
