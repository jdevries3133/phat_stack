use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum BaseTemplate {
    BareBones,
    Minimal,
    Moderate,
    Heavy,
    SuperHeavy,
}

pub fn puke_template() {
    let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/template.tgz"));
    print!("{bytes:?}");
}
