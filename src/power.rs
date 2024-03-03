#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RunMode {
    UnderDrive = 0,
    MidDrive = 1,
    StandardDrive = 2,
    OverDrive = 3,
}
