mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;
mod ai;
mod test_mod;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use specs::prelude::*;
use std::time::{Instant, Duration};

use components::*;
use keyboard::MovementCommand;

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn initialize_player(world: &mut World, player_spritesheet: usize) -> Entity {
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
    };

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Rect::from_center(Point::new(0, 0), 26, 36)))
        .with(Velocity {speed: 0.0, direction: Direction::Right})
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build()
}

fn initialize_enemy(world: &mut World, enemy_spritesheet: usize, position: Point) -> Entity {
    let enemy_top_left_frame = Rect::new(0, 0, 32, 36);

    let enemy_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(enemy_spritesheet, enemy_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(enemy_spritesheet, enemy_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(enemy_spritesheet, enemy_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(enemy_spritesheet, enemy_top_left_frame, Direction::Right),
    };

    world.create_entity()
        .with(Enemy)
        .with(Position(Rect::from_center(position, 32, 36)))
        .with(Velocity {speed: 0.0, direction: Direction::Right})
        .with(enemy_animation.right_frames[0].clone())
        .with(enemy_animation)
        .build()
}

fn initialize_item(world: &mut World, item_spritesheet: usize, position: Point) -> Entity {
    world.create_entity()
        .with(Item)
        .with(Position(Rect::from_center(position, 12, 23)))
        .with(Velocity{direction: Direction::Right, speed: 0.0})
        .with(Sprite{region: Rect::new(18, 112, 12, 23), spritesheet: item_spritesheet})
        .build()
}

fn canvas_rect((width, height): (u32, u32)) -> physics::CanvasRegion {
    physics::CanvasRegion::from_center(Point::new(0, 0), width, height)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("game tutorial", 350, 350)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(ai::Ai, "Ai", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "Ai"])
        .with(animator::Animator, "Animator", &["Keyboard", "Ai"])
        .build();
    let mut world = World::new();
    dispatcher.setup(&mut world);
    renderer::RendererData::setup(&mut world);

    let mut font = ttf_context.load_font("arial.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let surface = font
        .render("Hello Rust!")
        .blended(Color::RGBA(0, 128, 128, 255))
        .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;


    // Initialize resources
    world.insert(Option::<MovementCommand>::None);
    world.insert(canvas_rect(canvas.output_size()?));
    world.insert(Instant::now().elapsed() as physics::FrameDuration);

    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
        texture_creator.load_texture("assets/reaper.png")?,
        texture_creator.load_texture("assets/darkdimension.png")?,
    ];
    let player_spritesheet = 0;
    let enemy_spritesheet = 1;
    let item_spritesheet = 2;

    initialize_enemy(&mut world, enemy_spritesheet, Point::new(-100, 100));
    initialize_enemy(&mut world, enemy_spritesheet, Point::new(100, -100));
    initialize_player(&mut world, player_spritesheet);
    initialize_item(&mut world, item_spritesheet, Point::new(-150, -150));
    initialize_item(&mut world, item_spritesheet, Point::new(150, -150));
    initialize_item(&mut world, item_spritesheet, Point::new(-150, 150));
    initialize_item(&mut world, item_spritesheet, Point::new(150, 150));
    initialize_item(&mut world, item_spritesheet, Point::new(-125, -125));
    initialize_item(&mut world, item_spritesheet, Point::new(125, -125));
    initialize_item(&mut world, item_spritesheet, Point::new(-125, 125));
    initialize_item(&mut world, item_spritesheet, Point::new(125, 125));

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut instant = Instant::now();
    'running: loop {
        // Handle events
        let mut movement_command = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                },
                _ => {},
            }
        }
        *world.write_resource() = movement_command;
        *world.write_resource() = canvas_rect(canvas.output_size()?);
        *world.write_resource() = instant.elapsed() as physics::FrameDuration;
        instant = Instant::now();

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &textures, &font_texture, world.system_data())?;

        // Time management!
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
    }

    Ok(())
}