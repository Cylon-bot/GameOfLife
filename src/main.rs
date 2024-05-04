mod drawing;
mod item;

use std::thread::sleep;
use std::time::Duration;

use drawing::cell_state::CellState;
use drawing::grid_game::GridCreation;
use drawing::Drawing;
use item::{BoxGame, Cell, Coordonate, Pixel};
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{
    DeviceId, ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, WindowBuilder};

fn main() -> Result<(), Error> {
    // create an EventLoop for the game
    let my_event_loop = EventLoop::new();
    // create an Windown and attach the EventLoop to it
    let window = {
        WindowBuilder::new()
            .with_title("Game of life")
            .build(&my_event_loop)
            .unwrap()
    };
    // set the window to fullscreen borderless
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    // create a Pixels structure that will be use to redraw pixels on a SurfaceTexture attache to the window
    let mut pixels = {
        let window_size = window.inner_size();

        let surface_texture: SurfaceTexture<'_, winit::window::Window> =
            SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };
    let mut all_pixels = vec![];
    let mut id_pixels = vec![];
    (all_pixels, id_pixels) = {
        let window_size = window.inner_size();
        Pixel::create_all_from_grid(Coordonate::new(
            window_size.width as u16,
            window_size.height as u16,
        ))
    };

    let box_window = {
        let window_size = window.inner_size();
        BoxGame::new(
            0,
            0,
            window_size.width as u16,
            window_size.height as u16,
            id_pixels,
        )
    };

    // run the event loop of the game
    let mut loop_iteration: u32 = 1;
    pixels.clear_color(Color::BLACK);

    let mut my_grid = GridCreation::new(box_window.clone(), 200, 150, 1);
    my_grid.draw(&mut all_pixels);

    let mut cell_state = CellState::new(&my_grid, &mut all_pixels);

    let mut actual_position_cursor = Coordonate::new(0, 0);

    let mut pause = false;

    my_event_loop.run(move |event, _, control_flow| {
        match event {
            // enter on that arm when event is detected on the window and process only the CloseRequested window event (alt + F4)
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit()
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Pressed
                {
                    pause = !pause;
                    if pause {
                        println!("Pause");
                    } else {
                        println!("Unpause");
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                actual_position_cursor.x = position.x as u16;
                actual_position_cursor.y = position.y as u16;
            }

            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, .. },
                ..
            } => if button == MouseButton::Left && pause {},
            // call this event continuously (main)
            Event::MainEventsCleared => {
                if !pause {
                    println!("iteration number --> {}", loop_iteration);
                    cell_state.draw(&mut all_pixels);

                    for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                        pixel.copy_from_slice(&[
                            all_pixels[i].r,
                            all_pixels[i].g,
                            all_pixels[i].b,
                            all_pixels[i].a,
                        ]);

                        if i >= box_window.size {
                            break;
                        }
                    }
                    if let Err(_err) = pixels.render() {
                        println!("OUPS");
                        control_flow.set_exit();
                    }
                }
            }
            // call when MainEventsCleared is finished
            Event::RedrawEventsCleared => {
                if !pause {
                    loop_iteration += 1;
                }

                // sleep(Duration::new(0, 1e1 as u32));
            }
            _ => (),
        }

        // window.request_redraw();
    })
}
