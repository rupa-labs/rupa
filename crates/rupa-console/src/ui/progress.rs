use std::io::Write;

/// Standardized progress indicators for the CLI Console.
pub struct ConsoleProgress;

impl ConsoleProgress {
    /// Draws a progress bar.
    pub fn draw_bar(label: &str, current: f32, max: f32) {
        let percent = (current / max * 100.0) as u32;
        let bar_width = 30;
        let filled = (current / max * bar_width as f32) as usize;
        let empty = bar_width - filled;

        print!("\r\x1B[36m{}\x1B[0m: [\x1B[32m{}\x1B[90m{}\x1B[0m] {}% ", 
            label, 
            "█".repeat(filled), 
            "░".repeat(empty), 
            percent
        );
        std::io::stdout().flush().unwrap();
        if current >= max { println!(); }
    }
}
