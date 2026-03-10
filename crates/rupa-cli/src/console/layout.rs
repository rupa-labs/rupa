/// Standardized layout utilities for the CLI Console.
pub struct ConsoleLayout;

impl ConsoleLayout {
    /// Draws a horizontal line separator.
    pub fn draw_line() {
        println!("\x1B[90m──────────────────────────────────────────────────\x1B[0m");
    }

    /// Draws a box with a title and content lines.
    pub fn draw_box(title: &str, lines: Vec<String>) {
        let width = title.len().max(lines.iter().map(|l| l.len()).max().unwrap_or(0)) + 4;
        let border_color = "\x1B[36m"; 
        let reset = "\x1B[0m";

        println!("{}{}{}{}", border_color, "┌", "─".repeat(width - 2), "┐");
        println!("{}│ {} {}│", border_color, format!("{:^width$}", title, width = width - 4), border_color);
        println!("{}{}{}{}", border_color, "├", "─".repeat(width - 2), "┤");
        
        for line in lines {
            println!("{}│ {} {}│", border_color, format!("{:<width$}", line, width = width - 4), border_color);
        }
        
        println!("{}{}{}{}", border_color, "└", "─".repeat(width - 2), "┘", reset);
    }
}
