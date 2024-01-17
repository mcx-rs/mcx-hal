mod scg;
mod syscon;

pub use scg::*;
pub use syscon::*;

pub enum Error {
    InvalidFrequency,
    Busy,
}
