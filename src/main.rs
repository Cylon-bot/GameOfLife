use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::monitor::MonitorHandle;
use winit::window::{Fullscreen, WindowBuilder};

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        WindowBuilder::new()
            .with_title("Game of life")
            .build(&event_loop)
            .unwrap()
        // .fullscreen()
    };
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    // window.set_inner_size(LogicalSize::new(2000.0, 1000.0));
    // window.set_outer_position(LogicalPosition::new(1500.0, 1300.0));
    // let mut pixels = {
    //     let window_size = window.inner_size();
    //     let surface_texture: SurfaceTexture<'_, winit::window::Window> =
    //         SurfaceTexture::new(window_size.width, window_size.height, &window);
    //     Pixels::new(320, 240, surface_texture)?
    // };
    event_loop.run(move |event, _, control_flow| {
        // *control_flow = ControlFlow::Poll;
        // pixels.clear_color(Color::BLUE);
        // pixels.resize_surface(1500, 1500);
        // *control_flow = ControlFlow::Exit;
        // match event {
        //     Event::WindowEvent {
        //         event: WindowEvent::CloseRequested,
        //         ..
        //     } => elwt.exit(),
        //     _ => (),
        // }
    })
}
