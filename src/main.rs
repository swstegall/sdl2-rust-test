extern crate sdl2;

mod game;

struct WindowConfig<'a> {
  title: &'a str,
  width: u32,
  height: u32,
}

fn main() {
  let cfg: WindowConfig = WindowConfig {
    title: "rust-sdl2 demo",
    width: 1280,
    height: 720,
  };
  game::setup(cfg.title, cfg.width, cfg.height);
}
