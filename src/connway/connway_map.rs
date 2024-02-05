use std::mem;

#[repr(u8)] // NOTE: we would want this to be a single byte apparently!
pub enum ConnwayCell {
    Alive = 1,
    Dead = 0,
}

pub struct Universe {
   pub width: u32,
   pub height: u32,
   cells: Vec<ConnwayCell>,
   pub cell_size: f32,
   pub speed: u128,
   last_frame_time: Instant, 
}
impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count: u8 = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;

            
            }
        }
        count
    }
}
