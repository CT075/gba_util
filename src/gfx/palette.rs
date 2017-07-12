
use std::cmp::min;
use image::{Pixel, RgbImage, Rgb};

pub const PALETTE_LEN: usize = 16;
const COL_MASK: u8 = 0xF8;

pub struct Palette {
    used: usize,
    entries: [u16; PALETTE_LEN],
}

pub enum Error {
    TooManyColors,
    UnindexedColor,
}

impl Palette {
    pub fn new() -> Self {
        Palette { used: 0, entries: [0xFFFF; PALETTE_LEN] }
    }

    pub fn init(cols: [u16; PALETTE_LEN], used: usize) -> Self {
        Palette { used: min(used, PALETTE_LEN), entries: cols }
    }

    pub fn from_indexed_image(im: &RgbImage) -> Self {
        let mut result: [u16; PALETTE_LEN] = [0xFFFF; PALETTE_LEN];
        let mut used = 0;
        for w in (im.width()-8)..im.width() {
            let col = to_bgr(im.get_pixel(w,0));
            match result.iter().find(|&&c| c==col) {
                Some(_) => (),
                None => {
                    result[used] = col;
                    used += 1;
                }
            }
        }
        Palette { used: used, entries: result }
    }

    pub fn index_image(&self, im: &RgbImage) -> Result<Box<[u8]>, Error> {
        let mut result = Vec::new();
        for pix in im.pixels() {
            let col = to_bgr(pix);
            match self.find(col) {
                Some(x) => result.push(x),
                None => return Err(Error::UnindexedColor),
            }
        }
        Ok(result.into_boxed_slice())
    }


    pub fn from_unindexed_image(im: &RgbImage)
        -> Result<(Self, Box<[u8]>), Error>
    {
        let mut result = Self::new();
        let mut img = Vec::new();
        for pix in im.pixels() {
            let col = to_bgr(pix);
            match result.find(col) {
                Some(i) => img.push(i),
                None => { result.add_color(col)?; }
            }
        }
        Ok((result, img.into_boxed_slice()))
    }

    pub fn get_color(&self, i: usize) -> u16 {
        self.entries[i]
    }

    pub fn set_color(&mut self, i: usize, col: u16) -> () {
        self.entries[i] = col;
    }

    pub fn add_color(&mut self, col: u16) -> Result<(), Error> {
        if self.used >= PALETTE_LEN { return Err(Error::TooManyColors); }
        let used = self.used;
        self.set_color(used, col);
        self.used += 1;
        Ok(())
    }

    pub fn map<F>(&mut self, f: F) -> ()
        where F: Fn(u16) -> u16
    {
        // I wish we could use "iter().map(f)", but alas
        for i in 0..self.used {
            let x = self.entries[i];
            self.entries[i] = f(x);
        }
    }

    pub fn contains(&self, col: u16) -> bool {
        self.entries.iter().any(|&c| c==col)
    }

    pub fn find(&self, col: u16) -> Option<u8> {
        for i in 0..self.used {
            if self.entries[i] == col { return Some(i as u8); }
        }
        None
    }
}

pub fn to_bgr(pix: &Rgb<u8>) -> u16 {
    let ch = pix.channels();
    if ch.len() < 3 { panic!("Pixel has less than three channels!"); }
    condense(ch[0], ch[1], ch[2])
}

pub fn condense(r: u8, g: u8, b: u8) -> u16 {
    let rval = (r & COL_MASK) as u16;
    let gval = (g & COL_MASK) as u16;
    let bval = (b & COL_MASK) as u16;
    (bval << 10) + (gval << 5) + rval
}

