use crate::err::*;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum BaseTemplate {
    BareBones,
    Minimal,
    Moderate,
    Heavy,
    SuperHeavy,
}

impl BaseTemplate {
    pub fn tarball(&self) -> Result<&'static [u8], ErrStack> {
        match self {
            Self::Moderate => {
                Ok(include_bytes!(concat!(env!("OUT_DIR"), "/template.tgz")))
            }
            _ => Err(ErrStack::new(ErrT::NotImplemented)),
        }
    }
}
