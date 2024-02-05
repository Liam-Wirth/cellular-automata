use std::mem;

#[repr(u8)] // NOTE: we would want this to be a single byte apparently!
pub enum ConnwayCellState {
    Alive = 1,
    Dead = 0,
}

