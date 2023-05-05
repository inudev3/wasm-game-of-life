mod utils;

use std::fmt;
use std::fmt::Formatter;
use std::ptr::write;
use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone,Debug,Eq, PartialEq)]
pub enum Cell{
    Dead =0,
    Alive = 1
}
#[wasm_bindgen]
pub struct Universe{
    width:u32,
    height:u32,
    cells:FixedBitSet,
}
#[wasm_bindgen]
impl Universe {
    pub fn new()->Self{
        let width = 64;
        let height=64;
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size{
            cells.set(i, js_sys::Math::random()<0.5);
        }
        Self{
            width,height,cells
        }
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self){
        let mut next = self.cells.clone();
        for row in 0..self.height{
            for col in 0..self.width{
                let idx=self.get_index(row,col);
                let cell=self.cells[idx];
                let live_cnt = self.live_neighbor_count(row,col);
                next.set(idx, match(cell, live_cnt){
                    (true, x) if x<2=>false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (other,_)=>other,
                });
            }
        }
        self.cells=next;
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}
impl fmt::Display for Universe{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks((self.width/8) as usize){
            for i in 0..line.len(){
                let byte = i/8;
                let mask = 1<<(i%8);
                if line[byte]&mask==mask{
                    write!(f, "{}", '◼')?;
                }else{
                    write!(f, "{}", '◻')?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

