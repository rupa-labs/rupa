use crate::templates::{TemplateType, stubs};

pub fn generate(name: &str, template: &TemplateType) -> String {
    match template {
        TemplateType::Pure | TemplateType::Headless => stubs::MAIN_PURE.to_string(),
        TemplateType::Terminal => stubs::MAIN_TERMINAL.replace("{{title}}", name),
        TemplateType::Web => stubs::MAIN_WEB.replace("{{title}}", name),
        TemplateType::Server => stubs::MAIN_SERVER.replace("{{title}}", name),
        TemplateType::Mobile => stubs::MAIN_MOBILE.replace("{{title}}", name),
        TemplateType::Hybrid => stubs::MAIN_HYBRID.replace("{{title}}", name),
        TemplateType::Fullstack => stubs::MAIN_FULLSTACK.replace("{{title}}", name),
        TemplateType::Plugin => stubs::MAIN_PLUGIN.replace("{{name}}", name),
        TemplateType::Library => "pub fn init() { println!(\"Rupa Library Initialized\"); }".to_string(),
        _ => stubs::MAIN_NATIVE.replace("{{title}}", name),
    }
}
