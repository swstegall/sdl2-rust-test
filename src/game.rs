use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// This renders a loop of color from blue to red.
fn color_loop(context: sdl2::Sdl, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
  canvas.set_draw_color(Color::RGB(0, 255, 255));
  canvas.clear();
  canvas.present();
  let mut event_pump = context.event_pump().unwrap();
  let mut i = 0;
  'running: loop {
    i = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}

pub fn setup(title: &str, width: u32, height: u32) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window(title, width, height)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  color_loop(sdl_context, &mut canvas);
}
