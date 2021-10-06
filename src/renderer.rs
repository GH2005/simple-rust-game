use specs::prelude::*;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Rect, Point};

use crate::components::*;

pub type RendererData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    font_texture: &Texture,
    data: RendererData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    canvas.copy(font_texture, None, Rect::from_center(Point::new(width as i32 / 2, height as i32 / 2), 200, 50))?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        let screen_rect = {
            let mut screen_rect = pos.0;
            screen_rect.offset(width as i32 / 2, height as i32 / 2);
            screen_rect
        };

        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}