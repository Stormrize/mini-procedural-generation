#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Details {
    Castle,
}

impl Details {

    pub fn path(self) -> &'static str {
        match self {
            Details::Castle => "tower-top.glb",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Details::Castle => 0,
        }
    }
    pub const ALL: [Details; 1] = [Details::Castle];
}