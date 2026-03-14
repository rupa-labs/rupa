use crate::templates::{TemplateType, stubs};

pub fn generate(name: &str, template: &TemplateType) -> String {
    let features = match template {
        TemplateType::Native => "[\"desktop\"]",
        TemplateType::Web => "[\"web\"]",
        TemplateType::Terminal => "[\"terminal\"]",
        TemplateType::Mobile => "[\"mobile\"]",
        TemplateType::Server => "[\"server\"]",
        TemplateType::Fullstack => "[\"fullstack\"]",
        TemplateType::Hybrid => "[\"hybrid\"]",
        _ => "[]",
    };

    match template {
        TemplateType::Pure | TemplateType::Headless | TemplateType::Library => {
            stubs::CARGO_BASE
                .replace("{{name}}", name)
        }
        _ => {
            stubs::CARGO
                .replace("{{name}}", name)
                .replace("{{features}}", features)
        }
    }
}
