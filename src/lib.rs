pub mod assembler;
pub mod logger;
pub mod messages;
pub mod topology;

#[cfg(test)]
mod tests {
    mod topology_tests;
}

pub const TIMEOUT_TIMER_MS: u64 = 500;
pub const TIMEOUT_BETWEEN_FLOODS_MS: u64 = 500;
