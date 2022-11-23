mod tetrimino;

extern crate rand;
extern crate sdl2;

use crate::tetrimino::create_tetriminos::create_tetriminos::TetriminoGenerator;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::libc::FILE;
use sdl2::mouse::SystemCursor::No;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::sys::image::{IMG_InitFlags_IMG_INIT_JPG, IMG_InitFlags_IMG_INIT_PNG};
use sdl2::video::{Window, WindowContext};
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tetrimino::create_tetriminos::create_tetriminos;

// To make things easier to read, we'll create a constant which
const TEXTURE_SIZE: u32 = 32;

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

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn save_highscore_and_lines(highscore: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscore = slice_to_string(highscore);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        format!("{}\n{}\n", s_highscore, s_number_of_lines).as_str(),
        "scores.txt",
    )
    .is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("score.txt") {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn create_new_tetrimino() -> create_tetriminos::Tetrimino {
    static mut PREV: u8 = 7;
    let mut rand_nb = rand::random::<u8>() % 7;
    if unsafe { PREV } == rand_nb {
        rand_nb = rand::random::<u8>() % 7;
    }
    match rand_nb {
        0 => create_tetriminos::TetriminoI::new(),
        1 => create_tetriminos::TetriminoJ::new(),
        2 => create_tetriminos::TetriminoL::new(),
        3 => create_tetriminos::TetriminoO::new(),
        4 => create_tetriminos::TetriminoS::new(),
        5 => create_tetriminos::TetriminoZ::new(),
        6 => create_tetriminos::TetriminoT::new(),
        _ => unreachable!(),
    }
}

fn main() {
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
