use std::time::SystemTime;

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

mod rendering;
mod util;
mod xox;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const SCALE: u32 = 3;

fn main() -> Result<(), Error> {
    let start_time = SystemTime::now();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("XOX")
        .with_inner_size(LogicalSize::new(WIDTH * SCALE, HEIGHT * SCALE))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut screen = rendering::Screen::new(WIDTH, HEIGHT, pixels);

    let mut ttt = xox::TicTacToe::new();

    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
        let elapsed = SystemTime::now()
            .duration_since(start_time)
            .expect("time went backwards");

        if let Event::RedrawRequested(_) = event {
            screen.clear(0xeeeeeeff);
            let elapsed = elapsed.as_millis() as f32 / 1000.0;
            let x0 = (elapsed * 2.0).cos() * 30.0 + 50.0;
            let y0 = (elapsed * 2.0).sin() * 20.0 + 30.0;
            let x1 = elapsed.cos() * 100.0 + 150.0;
            let y1 = elapsed.sin() * 50.0 + 100.0;
            screen.draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, 0x6655bbff);
            rendering::draw_grid(&mut screen, 0x334455ff, (10, 10));
            screen.render();
        }

        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                screen.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
