use sdl2::{rect::Rect, render::{Texture, WindowCanvas}};

use crate::tile_data::{TileData};

pub struct Sprite<'a> {
    x: u32,
    y: u32,
    index: u32,
    width: u32,
    height: u32,
    texture: &'a Texture<'a>,
    tile_data: &'a TileData,
}

impl<'a> Sprite<'a> {
    pub fn new(x: u32, y: u32, index: u32, width: u32, height: u32, texture: &'a Texture, tile_data: &'a TileData) -> Self {
        Sprite {
            x,
            y,
            index,
            width,
            height,
            texture,
            tile_data
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        // let texture = texture_creator.load_texture(self.tile_data.tiles.file_name)?;
        let tile_x = (self.index % self.width) * self.width;
        let tile_y = 0; // All tiles are in the first row

        let src_rect = Rect::new(tile_x as i32, tile_y as i32, self.width, self.height);
        let dst = Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32);
        canvas.copy(self.texture, src_rect, Some(dst))?;
        Ok(())
    }
}