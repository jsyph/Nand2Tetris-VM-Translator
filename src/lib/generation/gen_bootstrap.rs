use handlebars::Handlebars;

use crate::error::TranslatorResult;

#[derive(serde::Serialize)]
struct BootstrapTemplateData {
    optimize: bool,
}

pub fn gen_bootstrap(handlebars: &Handlebars, optimize: bool) -> TranslatorResult<String> {
    let data = BootstrapTemplateData { optimize };
    Ok(handlebars.render("bootstrap.hbs", &data)?)
}
