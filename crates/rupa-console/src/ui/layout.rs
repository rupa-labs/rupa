pub struct Layout;

impl Layout {
    pub fn draw_line() {
        println!("{}", "─".repeat(50));
    }

    pub fn draw_box(title: &str, lines: Vec<String>) {
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0).max(title.len()) + 4;
        
        println!("┌─ {} {}┐", title, "─".repeat(width - title.len() - 3));
        for line in lines {
            println!("│ {} {}│", line, " ".repeat(width - line.len() - 3));
        }
        println!("└{}┘", "─".repeat(width - 1));
    }
}
