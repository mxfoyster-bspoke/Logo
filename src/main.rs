use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create the event loop and handle the Result with '?'
    let event_loop = EventLoop::new()?;

    // 2. Build the window using the event_loop's target
    // Note: We use &event_loop here because it implements Deref to the target
    let _window = WindowBuilder::new()
        .with_title("Rust Graphics Window")
        .build(&event_loop)?;

    // 3. Run the event loop
    event_loop.run(move |event, elwt| {
        // elwt = EventLoopWindowTarget
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            _ => (),
        }
    })?;

    Ok(())
}
