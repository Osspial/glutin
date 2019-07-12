mod support;

use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context =
        ContextBuilder::new().with_vsync(true).build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl = support::load(&windowed_context.context());

    let mut redraw_requested = false;
    let mut redraw_in_event_cleared = false;

    println!("redraw in RedrawRequested");
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let redraw = || {
            // 5 ms seems like a Reasonable amount of time for a real app to spend rendering.
            // without it, nothing is noticable because clearing the screen and drawing a
            // single triangle is absolutely trivial.
            ::std::thread::sleep_ms(5);
            gl.draw_frame([1.0, 1.0, 1.0, 1.0]);
            windowed_context.swap_buffers().unwrap();
        };

        match event {
            Event::LoopDestroyed => return,
            Event::EventsCleared if redraw_in_event_cleared && redraw_requested => {
                redraw_requested = false;
                redraw()
            },
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor =
                        windowed_context.window().hidpi_factor();
                    windowed_context
                        .resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::KeyboardInput {input: KeyboardInput {virtual_keycode: Some(VirtualKeyCode::Space), state: ElementState::Pressed, ..}, ..} => {
                    redraw_in_event_cleared = !redraw_in_event_cleared;
                    match redraw_in_event_cleared {
                        true => println!("redraw in EventsCleared"),
                        false => println!("redraw in RedrawRequested"),
                    }
                }
                WindowEvent::RedrawRequested if !redraw_in_event_cleared => redraw(),
                WindowEvent::RedrawRequested if redraw_in_event_cleared => redraw_requested = true,
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            _ => (),
        }
    });
}
