use pixels::{Pixels, SurfaceTexture};
// We need to alias these to be crystal clear about which version we are using
extern crate raw_window_handle as rwh05;

use rwh05::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

// This struct will now manually bridge the gap
struct CompatibilityWrapper<'a>(&'a Window);

unsafe impl HasRawWindowHandle for CompatibilityWrapper<'_> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        // We use winit's internal method to get the 0.6 handle,
        // then convert it into a 0.5 handle.
        match winit::raw_window_handle::HasRawWindowHandle::raw_window_handle(self.0).unwrap() {
            winit::raw_window_handle::RawWindowHandle::Win32(handle) => {
                let mut h = rwh05::Win32WindowHandle::empty();
                h.hwnd = handle.hwnd.get() as *mut _;
                h.hinstance = handle.hinstance.map(|v| v.get() as *mut _).unwrap_or(std::ptr::null_mut());
                RawWindowHandle::Win32(h)
            }
            _ => panic!("Unsupported platform"),
        }
    }
}

unsafe impl HasRawDisplayHandle for CompatibilityWrapper<'_> {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        match winit::raw_window_handle::HasRawDisplayHandle::raw_display_handle(self.0).unwrap() {
            winit::raw_window_handle::RawDisplayHandle::Windows(_) => {
                RawDisplayHandle::Windows(rwh05::WindowsDisplayHandle::empty())
            }
            _ => panic!("Unsupported platform"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Rust Graphics Window")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)?;

    let window_size = window.inner_size();
    let wrapper = CompatibilityWrapper(&window);

    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &wrapper);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => elwt.exit(),
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let frame = pixels.frame_mut(); // Note: pixels 0.13 uses frame_mut(), not get_frame_mut()

                // Clear screen to dark gray
                frame.fill(30);

                // Draw white diagonal line
                draw_line(frame, 50, 50, 350, 350, [255, 255, 255, 255]);

                if let Err(err) = pixels.render() {
                    eprintln!("Pixels error: {}", err);
                    elwt.exit();
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    })?;

    Ok(())
}

fn draw_line(frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; 4]) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
            let offset = (y as usize * WIDTH as usize + x as usize) * 4;
            if offset + 4 <= frame.len() {
                frame[offset..offset + 4].copy_from_slice(&color);
            }
        }
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x += sx; }
        if e2 <= dx { err += dx; y += sy; }
    }
}