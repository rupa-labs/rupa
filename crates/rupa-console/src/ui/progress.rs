pub struct Progress;

impl Progress {
    pub fn draw_bar(label: &str, current: f32, max: f32) {
        let percent = (current / max).clamp(0.0, 1.0);
        let filled = (percent * 20.0) as usize;
        let empty = 20 - filled;
        
        print!("\r{}: [{}{}] {:.1}%", 
            label, 
            "█".repeat(filled), 
            "░".repeat(empty), 
            percent * 100.0
        );
        
        if percent >= 1.0 {
            println!();
        }
    }
}
