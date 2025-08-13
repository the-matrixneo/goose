use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// A non-blocking input handler that allows capturing user input while processing
pub struct QueuedInputHandler {
    queued_message: Arc<Mutex<Option<String>>>,
}

impl QueuedInputHandler {
    pub fn new() -> Self {
        Self {
            queued_message: Arc::new(Mutex::new(None)),
        }
    }

    /// Start listening for input in a background thread
    /// Returns a handle that can be used to check for queued input
    pub fn start_listening(&self) -> InputHandle {
        let queued = self.queued_message.clone();

        // Print prompt instructions right before we start listening
        println!();
        println!(
            "{}",
            console::style("╭─────────────────────────────────────────────────╮").dim()
        );
        println!(
            "{}",
            console::style("│ 💡 Type your next message below (it will queue) │")
                .cyan()
                .dim()
        );
        println!(
            "{}",
            console::style("╰─────────────────────────────────────────────────╯").dim()
        );
        print!("{} ", console::style("queue>").cyan().bold());

        // Flush to ensure the prompt is displayed
        use std::io::{self, Write};
        let _ = io::stdout().flush();

        // Spawn a thread to listen for input
        let handle = thread::spawn(move || {
            // Use stdin directly to read a line
            use std::io::{BufRead, BufReader};

            let stdin = io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut line = String::new();

            // Try to read a line from stdin
            if reader.read_line(&mut line).is_ok() {
                let trimmed = line.trim();
                if !trimmed.is_empty()
                    && !trimmed.starts_with("/exit")
                    && !trimmed.starts_with("/quit")
                {
                    if let Ok(mut queued_guard) = queued.lock() {
                        *queued_guard = Some(line.trim().to_string());
                        // Print confirmation that we captured the input
                        println!(
                            "{}",
                            console::style(format!(
                                "✓ Queued message will run after current task completes"
                            ))
                            .green()
                            .dim()
                        );
                    }
                }
            }
        });

        InputHandle {
            thread_handle: Some(handle),
            queued_message: self.queued_message.clone(),
        }
    }

    /// Check if there's a queued message without blocking
    #[allow(dead_code)]
    pub fn get_queued_message(&self) -> Option<String> {
        if let Ok(mut guard) = self.queued_message.lock() {
            guard.take()
        } else {
            None
        }
    }
}

/// Handle for managing the background input thread
pub struct InputHandle {
    thread_handle: Option<thread::JoinHandle<()>>,
    queued_message: Arc<Mutex<Option<String>>>,
}

impl InputHandle {
    /// Stop listening for input and retrieve any queued message
    pub fn stop(mut self) -> Option<String> {
        // The thread will naturally exit after reading one line or on error
        if let Some(handle) = self.thread_handle.take() {
            // Wait a short time for the thread to finish
            let _ = handle.join();
        }

        // Get any queued message
        if let Ok(mut guard) = self.queued_message.lock() {
            guard.take()
        } else {
            None
        }
    }

    /// Check if input has been queued without stopping the listener
    #[allow(dead_code)]
    pub fn check_queued(&self) -> bool {
        if let Ok(guard) = self.queued_message.lock() {
            guard.is_some()
        } else {
            false
        }
    }
}

impl Drop for InputHandle {
    fn drop(&mut self) {
        // Clean up the thread if still running
        if let Some(handle) = self.thread_handle.take() {
            // We can't really force the thread to stop, but it will exit after one input
            let _ = handle.join();
        }
    }
}
