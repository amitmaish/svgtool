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
    output: Option<String>,
    #[arg(short, long, default_value_t = 1.0)]
    scale: f64,
    #[arg(short, long, default_value_t = 96.0)]
    dpi: f64,
}

#[repr(C)]
pub struct CArgs {
    input: *mut c_char,
    output: *mut c_char,
    scale: f64,
    dpi: f64,
}

impl From<Args> for CArgs {
    fn from(value: Args) -> Self {
        let c_string_in = CString::new(value.input).unwrap();
        let c_string_out = value.output.map(|out| CString::new(out).unwrap());
        let output = if let Some(value) = c_string_out {
            value.into_raw()
        } else {
            std::ptr::null_mut()
        };
        Self {
            input: c_string_in.into_raw(),
            output,
            scale: value.scale,
            dpi: value.dpi,
        }
    }
}

impl TryFrom<CArgs> for Args {
    type Error = CParseError;

    fn try_from(value: CArgs) -> Result<Self, Self::Error> {
        let input = unsafe { CString::from_raw(value.input) };
        let output = if value.output.is_null() {
            None
        } else {
            Some(value.output)
        };
        let output = output.map(|out| unsafe { CString::from_raw(out) });
        let output = if let Some(out) = output {
            Some(String::from(out.to_str()?))
        } else {
            None
        };
        Ok(Args {
            input: String::from(input.to_str()?),
            output,
            scale: value.scale,
            dpi: value.dpi,
        })
    }
}

#[derive(Error, Debug)]
pub enum CParseError {
    #[error("input string not valid utf8")]
    Utf8(#[from] std::str::Utf8Error),
}
