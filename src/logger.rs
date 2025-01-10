use wg_2024::network::NodeId;

/// Used in the log method
/// * `INFO`: default log level, will always be printed
/// * `DEBUG`: used only in debug situation, will not print if the debug flag is `false`
/// * `ERROR`: will print the message to `io::stderr`
pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR,
}

/// Struct used to cleanly log messages and information
///
/// * `node_type: String` - the type of the node logging the information, e.g. Chat Client
/// * `node_id: NodeId` - the id of the node logging the information
/// * `debug: bool ` - debug flag, if true then it will log DEBUG level
pub struct Logger {
    node_type: String,
    node_id: NodeId,
    debug: bool,
}

impl Logger {
    pub fn new(node_type: String, node_id: NodeId, debug: bool) -> Logger {
        Logger {
            node_type,
            node_id,
            debug,
        }
    }

    /// Utility method used to cleanly log information, differentiating on three different levels
    ///
    /// # Args
    /// * `log_message: &str` - the message to log
    /// * `log_level: LogLevel` - the level of the log:
    ///     * `INFO`: default log level, will always be printed
    ///     * `DEBUG`: used only in debug situation, will not print if the debug flag is `false`
    ///     * `ERROR`: will print the message to `io::stderr`
    pub fn log(&self, log_message: &str, log_level: LogLevel) {
        match log_level {
            LogLevel::INFO => {
                println!(
                    "[{} {}] - LEVEL: INFO >>> {}",
                    self.node_type, self.node_id, log_message
                );
            }
            LogLevel::DEBUG => {
                if self.debug {
                    println!(
                        "[{} {}] - LEVEL: DEBUG >>> {}",
                        self.node_type, self.node_id, log_message
                    );
                }
            }
            LogLevel::ERROR => {
                eprintln!(
                    "[{} {}] - LEVEL: ERROR >>> {}",
                    self.node_type, self.node_id, log_message
                );
            }
        }
    }
}
