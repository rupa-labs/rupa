use rupaui::prelude::*;

fn main() {
    env_logger::init();
    
    App::new("Rupaui App")
        .root(
            Section::new("Hero")
                .child(Box::new(Text::new("Hello Rupaui")))
        )
        .run();
}
