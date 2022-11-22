extern crate sdl2;

use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{InitFlag, LoadTexture};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use sdl2::libc::FILE;
use sdl2::mouse::SystemCursor::No;
use sdl2::sys::image::{IMG_InitFlags_IMG_INIT_JPG, IMG_InitFlags_IMG_INIT_PNG};

// To make things easier to read, we'll create a constant which
const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                match color {
                    TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                    TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                }
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()>
{
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map(|highscore| highscore.to_string()).collect::<Vec<String>>().join(" ")
}

fn save_highscore_and_lines(highscore: &[u32], number_of_lines: &[u32]) -> bool
{
    let s_highscore = slice_to_string(highscore);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(format!("{}\n{}\n", s_highscore, s_number_of_lines).as_str(), "scores.txt").is_ok()
}

/*fn main() {
    let sdl_context = sdl2::init().expect(
        "SDL initialization
       failed",
    );
    let video_subsystem = sdl_context.video().expect(
        "Couldn't get
          SDL video subsystem",
    );

    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let green_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Green,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");
    let blue_square = create_texture_rect(
        &mut canvas,
        &texture_creator,
        TextureColor::Blue,
        TEXTURE_SIZE,
    )
    .expect("Failed to create a texture");

    let timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect(
        "Failed
         to get SDL event pump",
    );

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => true,
        };
        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };
        canvas
            .copy(
                square_texture,
                None,
                Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE),
            )
            .expect("Couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
*/

pub fn main()
{
    let sdl_context = sdl2::init().expect("SDL initialization
       failed");
    let video_subsystem = sdl_context.video().expect("Couldn't
         get SDL video subsystem");

    sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("");

    let window = video_subsystem.window("rust-sdl2 image demo", 800,
                                        600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas().build().expect("Failed to
        convert window into canvas");


    let texture_creator: TextureCreator<_> =
        canvas.texture_creator();
    let image_texture =
        texture_creator.load_texture("src/assets/my_image.jpg")
            .expect("Couldn't load image");

    let mut event_pump = sdl_context.event_pump().expect("Failed to
         get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..}
                => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&image_texture, None, None).expect("REnder failed");
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}