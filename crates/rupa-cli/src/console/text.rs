/// Standardized text utilities for the CLI Console.
pub struct ConsoleText;

impl ConsoleText {
    pub fn plain(msg: impl AsRef<str>) {
        println!("{}", msg.as_ref());
    }

    pub fn info(msg: impl AsRef<str>) {
        println!("\x1B[34mℹ {}\x1B[0m", msg.as_ref());
    }

    pub fn success(msg: impl AsRef<str>) {
        println!("\x1B[32m✔ {}\x1B[0m", msg.as_ref());
    }

    pub fn warning(msg: impl AsRef<str>) {
        println!("\x1B[33m⚠ {}\x1B[0m", msg.as_ref());
    }

    pub fn error(msg: impl AsRef<str>) {
        eprintln!("\x1B[31m✖ {}\x1B[0m", msg.as_ref());
    }
}
