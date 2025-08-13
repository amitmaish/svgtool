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
    /// this alters the scale at which your svgs are rastored
    #[arg(short, long, default_value_t = 1.0)]
    scale: f64,
    /// for svgs with a size in real units, this sets the dpi. this parameter doesn't change the
    /// aspect ratio, and doesn't affect svgs without physical dimensions
    #[arg(short, long, default_value_t = 96.0)]
    dpi: f64,
    /// sets the target width of the final image, overwriting scale and dpi. if set together with
    /// height, the svg will be enlarged to fit the final dimensions, but it won't be squashed or
    /// stretched
    #[arg(short, long)]
    width: Option<u32>,
    /// sets the target height of the final image, overwriting scale and dpi. if set together with
    /// width, the svg will be enlarged to fit the final dimensions, but it won't be squashed or
    /// stretched
    #[arg(short, long)]
    height: Option<u32>,
}

#[repr(C)]
pub struct CArgs {
    input: *mut c_char,
    output: *mut c_char,
    width: COption<u32>,
    height: COption<u32>,
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
            width: value.width.into(),
            height: value.height.into(),
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
            width: value.width.into(),
            height: value.height.into(),
        })
    }
}

#[derive(Error, Debug)]
pub enum CParseError {
    #[error("input string not valid utf8")]
    Utf8(#[from] std::str::Utf8Error),
}

#[repr(C)]
pub enum COption<T> {
    Some(T),
    None,
}

impl<T> From<Option<T>> for COption<T> {
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            COption::Some(value)
        } else {
            COption::None
        }
    }
}

impl<T> From<COption<T>> for Option<T> {
    fn from(value: COption<T>) -> Self {
        if let COption::Some(value) = value {
            Some(value)
        } else {
            None
        }
    }
}
