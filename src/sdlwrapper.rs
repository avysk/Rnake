use resvg::tiny_skia::PixmapPaint;
use std::cmp::min;
use std::collections::HashMap;

use resvg::usvg::TreeParsing;
use resvg::Tree;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, TextureAccess, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::sys::{SDL_Delay, SDL_ShowCursor, SDL_DISABLE};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::{pixels::Color, EventPump};

use crate::sound::{Player, Sounds};
use crate::world::FIELD_SIZE;

/// This macro creates SDL2 Rect, casting the arguments to the appropriate types
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new(($x) as i32, ($y) as i32, ($w) as u32, ($h) as u32)
    };
}

/// This macro creates resvg::tiny_skia::Pixmap of the given size, renders on it image from
/// resources/images/, and pushes the pixmap to a vector in a given hashmap.
macro_rules! load_one_image {
    ($cell:expr, $pixmaps:ident, $name:ident $num:literal) => {{
        let tree = resvg::usvg::Tree::from_str(
            include_str!(concat!(
                "resources/images/",
                stringify!($name),
                "0",
                stringify!($num),
                ".svg"
            )),
            &resvg::usvg::Options::default(),
        )
        .expect("Should be able to parse SVG tree");
        let rtree = Tree::from_usvg(&tree);
        let cell = *$cell;
        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(cell, cell).expect("Should be able to create pixmap");
        let scale = cell as f32 / f32::max(rtree.size.width(), rtree.size.height());
        rtree.render(
            resvg::tiny_skia::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );
        let data = pixmap.data_mut();
        for i in 0..(data.len() / 4) {
            let idx = 4 * i;
            if data[idx] == 0 && data[idx + 1] == 0 && data[idx + 2] == 0 {
                data[idx + 3] = 0;
            } else {
                data[idx + 3] = 255;
            }
        }
        $pixmaps
            .entry(stringify!($name).to_string())
            .or_insert(vec![])
            .insert(0, pixmap);
    }};
}

/// This macro loads the given amount of images
macro_rules! load_images_rec {
    ($cell:expr, $pixmaps:ident, $name:ident, 1) => {
        load_one_image!($cell, $pixmaps, $name 1);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 2) => {
        load_one_image!($cell, $pixmaps, $name 2);
        load_images_rec!($cell, $pixmaps, $name, 1);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 3) => {
        load_one_image!($cell, $pixmaps, $name 3);
        load_images_rec!($cell, $pixmaps, $name, 2);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 4) => {
        load_one_image!($cell, $pixmaps, $name 4);
        load_images_rec!($cell, $pixmaps, $name, 3);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 5) => {
        load_one_image!($cell, $pixmaps, $name 5);
        load_images_rec!($cell, $pixmaps, $name, 4);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 6) => {
        load_one_image!($cell, $pixmaps, $name 6);
        load_images_rec!($cell, $pixmaps, $name, 5);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 7) => {
        load_one_image!($cell, $pixmaps, $name 7);
        load_images_rec!($cell, $pixmaps, $name, 6);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 8) => {
        load_one_image!($cell, $pixmaps, $name 8);
        load_images_rec!($cell, $pixmaps, $name, 7);
    };
    ($cell:expr, $pixmaps:ident, $name:ident, 9) => {
        load_one_image!($cell, $pixmaps, $name 9);
        load_images_rec!($cell, $pixmaps, $name, 8);
    };
}

/// This macro does the following:
/// - defines create_pixmaps() which loads pixmaps
/// - defines a bench of blah() on SDLWrapper
macro_rules! load_images {
    ($($name:ident $count:tt),*) => {
        fn create_pixmaps(cell_size: &u32) -> HashMap<String, Vec<resvg::tiny_skia::Pixmap>> {
            let mut pixmaps = HashMap::new();
            $(load_images_rec!(cell_size, pixmaps, $name, $count);)*
            pixmaps
        }
        $(impl<'a> SDLWrapper<'a> {
            pub fn $name(&mut self, idx: &usize, x: &u32, y: &u32) {
                assert!(*idx < $count,
                    "Programming error: there is no image '{}' with index '{}'",
                    stringify!($name),
                    $count);
                let pixmap = &self.pixmaps[stringify!($name)][*idx];
                let rgba_data = pixmap.data();
                let width = pixmap.width();
                let height = pixmap.height();
                let creator = self.canvas.texture_creator();
                let mut texture = creator
                    .create_texture(
                        Some(PixelFormatEnum::RGBA32),
                        TextureAccess::Target,
                        width,
                        height,
                    )
                    .expect("Should be able to create texture");
                let rx = self.border_x + self.cell * *x;
                let ry = self.border_y + self.cell * *y;
                let tgt = rect!(rx, ry, self.cell, self.cell);
                texture
                    // 4 is one byte for each of RGBA
                    .update(None, rgba_data, 4 * self.cell as usize)
                    .expect("Should be able to update texture");
                texture.set_blend_mode(BlendMode::Blend);
                self.canvas
                    .copy(&texture, None, Some(tgt))
                    .expect("Should be able to copy texture to canvas");
            }
        })*
    };
}

const LINE_INTERVAL: u32 = 10;

pub struct SDLWrapper<'a> {
    // event pump
    pub events: EventPump,
    // graphics
    border_x: u32,
    border_y: u32,
    score_x: u32,
    score_y: u32,
    cell: u32,
    canvas: Canvas<Window>,
    // sound player
    pub sounds: Box<dyn Player>,
    // text
    font: Font<'a, 'static>,
    pixmaps: HashMap<String, Vec<resvg::tiny_skia::Pixmap>>,
    background: resvg::tiny_skia::Pixmap,
}

impl<'a> SDLWrapper<'a> {
    pub fn new(field_size: &u32, context: &'a Sdl2TtfContext) -> Self {
        let sdl_context = sdl2::init().expect("Should be able to get SDL context");
        // Events
        let events = sdl_context
            .event_pump()
            .expect("Should be able to create SDL event pump");
        let video_subsystem = sdl_context
            .video()
            .expect("Should be able to get SDL video subsystem");
        let window = video_subsystem
            .window("rnake", 0, 0)
            .fullscreen_desktop()
            .build()
            .expect("Should be able to create SDL window");
        unsafe {
            SDL_ShowCursor(SDL_DISABLE as i32);
        }
        let window_size = window.size();
        let canvas = window
            .into_canvas()
            .build()
            .expect("Should be able to get window's canvas");
        let square = min(window_size.0, window_size.1);
        // 2 for the wall around the field
        let field_plus_wall = *field_size + 2;
        // we divide and multiply to round the things
        let cell = square / field_plus_wall;
        let border_x = (window_size.0 - cell * field_plus_wall) / 2;
        let border_y = (window_size.1 - cell * field_plus_wall) / 2;

        // Sounds
        let sounds = Sounds::create();

        // Fonts
        let rwops = RWops::from_bytes(include_bytes!("resources/fonts/Aclonica.ttf"))
            .expect("Should be able to load rwops from font bytes.");
        let font_size = if window_size.0 >= 1536 { 72 } else { 36 };
        let font = context
            .load_font_from_rwops(rwops, font_size)
            .expect("Should be able to load font from rwops.");

        let pixmaps = create_pixmaps(&cell);

        load_images!(body 8, fat 3, food 3, headturn 8, headstraight 4, lean 3, mystery 4, obstacle 3, tail 4, wall 1);

        // background
        let background_tree = resvg::usvg::Tree::from_str(
            include_str!("resources/images/grass01.svg"),
            &resvg::usvg::Options::default(),
        )
        .expect("Should be able to parse grass SVG tree");
        let background_resvg_tree = Tree::from_usvg(&background_tree);
        let mut grass_pixmap = resvg::tiny_skia::Pixmap::new(cell, cell)
            .expect("Should be able to create pixmap for grass");
        let grass_scale = cell as f32
            / f32::max(
                background_resvg_tree.size.width(),
                background_resvg_tree.size.height(),
            );
        background_resvg_tree.render(
            resvg::tiny_skia::Transform::from_scale(grass_scale, grass_scale),
            &mut grass_pixmap.as_mut(),
        );
        // This is a background pixmap, no need to adjust black pixels making them transparent
        let mut background = resvg::tiny_skia::Pixmap::new(cell * FIELD_SIZE, cell * FIELD_SIZE)
            .expect("Should be able to create background pixmap");
        for row in 0..FIELD_SIZE {
            for col in 0..FIELD_SIZE {
                background.draw_pixmap(
                    (cell * row) as i32,
                    (cell * col) as i32,
                    grass_pixmap.as_ref(),
                    &PixmapPaint::default(),
                    resvg::tiny_skia::Transform::default(),
                    None,
                );
            }
        }

        Self {
            events,
            border_x,
            border_y,
            score_x: window_size.0 - 150, // TODO
            score_y: 50,
            cell,
            canvas,
            sounds,
            font,
            pixmaps,
            background,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }
    pub fn background(&mut self) {
        let creator = self.canvas.texture_creator();
        let mut texture = creator
            .create_texture(
                Some(PixelFormatEnum::RGBA32),
                TextureAccess::Target,
                self.background.width(),
                self.background.height(),
            )
            .expect("Should be able to create background texture");
        texture
            .update(
                None,
                self.background.data(),
                4 * self.background.width() as usize,
            )
            .expect("Should be able to update background texture");
        let tgt = rect!(
            self.border_x + self.cell,
            self.border_y + self.cell,
            self.background.width(),
            self.background.height()
        );
        self.canvas
            .copy(&texture, None, Some(tgt))
            .expect("Should be able to copy background texture to canvas");
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
    pub fn window(&self) -> Option<&Window> {
        Some(self.canvas.window())
    }
    pub fn banner(&mut self, text: String) {
        self.messages(&vec![text]);
        unsafe {
            SDL_Delay(500);
        }
    }
    pub fn messages(&mut self, messages: &Vec<String>) {
        self.clear();
        let surfaces = messages.iter().map(|line| {
            self.font
                .render(line)
                .solid(Color::BLUE)
                .expect("Should be able to render text line")
        });
        let creator = self.canvas.texture_creator();
        let textures = surfaces.map(|surface| {
            creator
                .create_texture_from_surface(surface)
                .expect("Should be able to create texture from surface")
        });
        let heights: u32 = textures.clone().map(|texture| texture.query().height).sum();
        let total_height = heights + (messages.len() as u32 - 1) * LINE_INTERVAL;
        let (win_width, win_height) = self
            .window()
            .expect("Should be able to get window corresponding to the texture")
            .size();
        if total_height >= win_height {
            panic!("Too many messages");
        }
        let mut pad_h = (win_height - total_height) / 2;

        for texture in textures {
            let TextureQuery { width, height, .. } = texture.query();
            if width >= win_width {
                panic!("Too long message");
            }
            let tgt = rect!((win_width - width) / 2, pad_h, width, height);
            self.canvas
                .copy(&texture, None, Some(tgt))
                .expect("Should be able to copy texture to canvas.");
            pad_h += height;
            pad_h += LINE_INTERVAL;
        }
        self.present();
    }
    pub fn score(&mut self, sc: u32) {
        let surface = self
            .font
            .render(sc.to_string().as_ref())
            .solid(Color::YELLOW)
            .expect("Should be able to render score");
        let creator = self.canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(surface)
            .expect("Should be able to create texture from surface");
        let TextureQuery { width, height, .. } = texture.query();
        let tgt = rect!(self.score_x, self.score_y, width, height);
        self.canvas
            .copy(&texture, None, Some(tgt))
            .expect("Should be able to copy texture to canvas");
    }
}
