use std::fs::File;
use std::io::BufReader;

use clap::Parser;

use pixels::{Error, Pixels, SurfaceTexture};
use qoi::Header;
use winit::dpi::PhysicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

#[derive(Parser, Debug)]
#[clap(
    author = "Arvin Kushwaha",
    version = "0.1.0",
    about = "Renders a QOI image"
)]
struct Args {
    /// The path to the image to render
    #[clap(parse(from_os_str))]
    image: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut reader = BufReader::new(File::open(&args.image).expect("Could not open file"));
    let mut decoder = qoi::Decoder::from_stream(&mut reader).expect("Invalid QOI file");
    let &Header {
        width,
        height,
        channels,
        ..
    } = decoder.header();
    let img_data = decoder.decode_to_vec().expect("Invalid QOI file");

    let copy_pixels = match channels {
        qoi::Channels::Rgb => |pixels: &mut [u8], img_data: &[u8]| {
            pixels
                .chunks_exact_mut(4)
                .zip(img_data.chunks_exact(3))
                .for_each(|(to, from)| {
                    to[..3].copy_from_slice(from);
                    to[3] = 255;
                })
        },
        qoi::Channels::Rgba => |pixels: &mut [u8], img_data: &[u8]| {
            pixels
                .chunks_exact_mut(4)
                .zip(img_data.chunks_exact(4))
                .for_each(|(to, from)| {
                    to.copy_from_slice(from);
                })
        },
    };

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title(
                args.image
                    .into_os_string()
                    .to_str()
                    .unwrap_or_else(|| "QOI Image <filename could not be displayed>"),
            )
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        match event {
            Event::RedrawRequested(_) => {
                let pix = pixels.get_frame();
                copy_pixels(pix, &img_data);
                if pixels.render().map_err(|_| ()).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
