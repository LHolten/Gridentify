use crate::grid::Board;
use rand::prelude::ThreadRng;
use rand::RngCore;

pub(crate) trait Random {
    fn new_board(&mut self) -> Board {
        array_init::array_init(|_| self.new_num())
    }
    fn new_num(&mut self) -> u32;
}

pub(crate) type BadRandom = u64;

impl Random for BadRandom {
    fn new_num(&mut self) -> u32 {
        let e = (16807 * *self) % 1924421567;
        *self = if e > 0 { e } else { e + 3229763266 };
        ((e % 3) + 1) as u32
    }
}

pub(crate) type GoodRandom = ThreadRng;

impl Random for GoodRandom {
    fn new_num(&mut self) -> u32 {
        (self.next_u32() % 3) + 1
    }
}
