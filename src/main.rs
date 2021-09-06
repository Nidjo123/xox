use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

mod xox;
mod rendering;
mod util;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("XOX")
            .build(&event_loop)
            .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(
        window_size.width,
        window_size.height,
        &window
    );
    let pixels = Pixels::new(
        WIDTH,
        HEIGHT, 
        surface_texture
    )?;

    let mut screen = rendering::Screen::new(WIDTH, HEIGHT, pixels);

    let mut input = WinitInputHelper::new();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        input.update(&event);
        if input.key_released(VirtualKeyCode::Escape) || input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if let Some(size) = input.window_resized() {
            screen.resize_surface(size.width, size.height);
        }

        if let Event::RedrawRequested(_) = event {
            screen.clear(0xffaabb);
            screen.render();
        }

        window.request_redraw();
    });
}
