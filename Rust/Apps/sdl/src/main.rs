use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::VideoSubsystem;

fn main() -> Result<(), String> {

    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl_context: Sdl = sdl2::init()?;
    let video_subsystem: VideoSubsystem = sdl_context.video()?;
    let window: Window = video_subsystem.window(title: "Rust Game!", screen_width, screen_height)WindowBuilder
    .build()Result<Window, WindowBuilder>
    .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas()CanvasBuilder
    .build()Result<Window, WindowBuilder>
    .unwrap()

    let screen_area: Rect = Rect::new(x:0, y: 0, screen_width, screen_height);

    println!("Hello, world!");
    Ok(())
}
