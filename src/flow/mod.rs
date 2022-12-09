use std::time::Duration;

use cgmath::Vector2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{logic::TetrisGame, rendering::{Renderer}};

pub struct GameFlow;

impl GameFlow {
	pub fn run(game: &mut TetrisGame) {
		game.spawn_piece(Vector2 {x: 2, y: 2});
		
		let renderer = &mut Renderer::new();
		
		let mut event_pump = renderer.sdl_context.event_pump().unwrap();
		let mut i = 0;
		'running: loop {
			i = (i + 1) % 255;
			renderer.fill(Color::RGB(i, 64, 255 - i));
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running
					},
					_ => {}
				}
			}
			// The rest of the game loop goes here...
	
			renderer.canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
}
