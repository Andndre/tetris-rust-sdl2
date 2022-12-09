use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub mod piece;

pub struct Renderer {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}

impl Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Tetris", 800, 800)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

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
