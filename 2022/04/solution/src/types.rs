#[derive(Debug, Clone, Copy)]
pub struct Sections { // struct probably not necessary
    pub start: u32,
    pub end: u32,
}

impl Sections {
    pub fn new(start: u32, end: u32) -> Sections {
        Sections { start, end }
    }
}
