pub struct Text;

impl Text {
    pub fn info(msg: impl AsRef<str>) {
        println!("\x1B[34m[INFO]\x1B[0m {}", msg.as_ref());
    }

    pub fn success(msg: impl AsRef<str>) {
        println!("\x1B[32m[SUCCESS]\x1B[0m {}", msg.as_ref());
    }

    pub fn warning(msg: impl AsRef<str>) {
        println!("\x1B[33m[WARNING]\x1B[0m {}", msg.as_ref());
    }

    pub fn error(msg: impl AsRef<str>) {
        eprintln!("\x1B[31m[ERROR]\x1B[0m {}", msg.as_ref());
    }

    pub fn plain(msg: impl AsRef<str>) {
        println!("{}", msg.as_ref());
    }
}
