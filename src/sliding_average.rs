use core::fmt::Debug;
use std::{
    fmt::{self},
    time::Duration,
};

pub struct SlidingAverage<T, const COUNT: usize = 10> {
    values: [T; COUNT],
    current_index: usize,
    sum: T,
    pub average: T,
}

#[allow(dead_code)]
impl<const COUNT: usize> SlidingAverage<Duration, COUNT> {
    pub fn new() -> Self {
        SlidingAverage {
            values: [Duration::ZERO; COUNT],
            current_index: 0,
            average: Duration::ZERO,
            sum: Duration::ZERO,
        }
    }

    pub fn push(&mut self, value: Duration) -> Duration {
        // Remove from sum overloaded value
        self.sum -= self.values[self.current_index];

        // Update value
        self.values[self.current_index] = value;

        // Update index
        self.current_index += 1;
        if self.current_index >= self.values.len() {
            self.current_index = 0;
        }

        // Update average
        self.sum += value;
        self.average = self.sum / self.values.len() as u32;

        // Return average
        self.average
    }

    pub fn last(&self) -> Duration {
        self.values[self.current_index]
    }
}

impl<const COUNT: usize> Debug for SlidingAverage<Duration, COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self.average, f)
    }
}
