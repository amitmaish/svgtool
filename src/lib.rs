use std::ffi::{CString, c_char};

use clap::Parser;
use thiserror::Error;

#[unsafe(no_mangle)]
pub extern "C" fn parse_args() -> CArgs {
    let args = Args::parse();
    CArgs::from(args)
}

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    scale: f64,
}

#[repr(C)]
pub struct CArgs {
    input: *mut c_char,
    scale: f64,
}

impl From<Args> for CArgs {
    fn from(value: Args) -> Self {
        let c_string = CString::new(value.input).unwrap();
        Self {
            input: c_string.into_raw(),
            scale: value.scale,
        }
    }
}

impl TryFrom<CArgs> for Args {
    type Error = CParseError;

    fn try_from(value: CArgs) -> Result<Self, Self::Error> {
        let input_path = unsafe { CString::from_raw(value.input) };
        Ok(Args {
            input: String::from(input_path.to_str()?),
            scale: value.scale,
        })
    }
}

#[derive(Error, Debug)]
pub enum CParseError {
    #[error("input string not valid utf8")]
    Utf8(#[from] std::str::Utf8Error),
}
