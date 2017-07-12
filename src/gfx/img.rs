

use image::{Pixel, ConvertBuffer, ImageBuffer, RgbImage};

use gfx::palette;
use gfx::palette::{Palette};

pub struct Image {
    width: u32,
    height: u32,
    pal: Palette,
    data: Box<[u8]>
}

type I<P,C> = ImageBuffer<P,C>;

impl Image {
    pub fn init<P,C>(im: I<P,C>, indexed: bool) -> Result<Self, palette::Error>
        where I<P,C>: ConvertBuffer<RgbImage>,
              P: Pixel + 'static,
    {
        let result:RgbImage = im.convert();
        if indexed { Self::init_indexed(result) }
        else { Self::init_unindexed(result) }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn pal(&self) -> &Palette { &self.pal }
    pub fn data(self) -> (Box<[u8]>, Palette) { (self.data, self.pal) }

    fn init_indexed(im: RgbImage) -> Result<Self, palette::Error> {
        let pal = Palette::from_indexed_image(&im);
        Ok(Image {
            width: im.width(),
            height: im.height(),
            data: pal.index_image(&im)?,
            pal: pal,
        })
    }

    fn init_unindexed(im: RgbImage) -> Result<Self, palette::Error> {
        let (pal, data) = Palette::from_unindexed_image(&im)?;
        Ok(Image {
            width: im.width(),
            height: im.height(),
            data: data,
            pal: pal,
        })
    }
}

