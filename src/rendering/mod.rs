use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub mod board;
pub mod gui;
pub mod piece;

pub struct Renderer {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}

impl Renderer {
    pub const BACKGROUND: Color = Color::RGB(12, 27, 35);
    pub const SECONDARY_BACKGROUND: Color = Color::RGB(29, 45, 54);
    pub const PRIMARY: Color = Color::RGB(52, 165, 229);
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let canvas = {
            let video_subsystem = sdl_context.video().unwrap();
            let window = video_subsystem
                .window("Tetris", 800, 800)
                .opengl()
                .position_centered()
                .build()
                .expect("Failded creating window");
            window.into_canvas().build().unwrap()
        };
        Self {
            canvas,
            sdl_context,
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }
}

pub trait Render {
    fn render(self, renderer: &mut Renderer) -> ();
}
