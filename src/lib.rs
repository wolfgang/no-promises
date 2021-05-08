pub mod promise;
mod promise_error;

pub use promise::Promise;
pub use promise::Error;

#[cfg(test)]
mod _tests;

