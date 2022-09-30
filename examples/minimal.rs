use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResizeConstraints,
};
use bevy_softbuffer::prelude::*;
//use rand::prelude::*;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

#[derive(Bundle, Debug)]
struct ObjectBundle {
    position: Position,
    velocity: Velocity,
    size: Size,
    color: Color,
}

#[derive(Component, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Component, Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Component, Debug)]
struct Color(u8, u8, u8, u8);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy Pixels".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            resize_constraints: WindowResizeConstraints {
                min_width: WIDTH as f32,
                min_height: HEIGHT as f32,
                ..default()
            },
            ..default()
        })
        .insert_resource(SoftBufferOptions {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SoftBufferPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        //.add_system(bounce)
        //.add_system(movement.after(bounce))
        .add_system(exit_on_escape)        
        .add_system_to_stage(SoftBufferStage::Draw, draw_background)
        .add_system_to_stage(SoftBufferStage::Draw, draw_objects)
        .add_system_to_stage(SoftBufferStage::Draw, draw_objects.after(draw_background))
        .run();
}

fn setup(mut commands: Commands) {
    let box_object = ObjectBundle {
        position: Position { x: 24, y: 16 },
        velocity: Velocity { x: 1, y: 1 },
        size: Size {
            width: 64,
            height: 64,
        },
        color: Color(0x5e, 0x48, 0xe8, 0xff),
    };
    commands.spawn().insert_bundle(box_object);
}

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x = (position.x as i16 + velocity.x) as u32;
        position.y = (position.y as i16 + velocity.y) as u32;
    }
}

fn exit_on_escape(keyboard_input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn draw_background(
    mut pixels_resource: ResMut<SoftBufferResource>,
    options: Res<SoftBufferOptions>,
) {
    let buffer = (0..((options.width * options.height) as usize))
                    .map(|_| {                        
                        let color = 128 | (128 << 8) | (128 << 16);

                        color as u32
                    })
                    .collect::<Vec<_>>();    

    pixels_resource.buffer = buffer;
}

#[inline]
fn get_index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

#[inline]
fn set_pixel(b: &mut Vec<u32>, index: usize, color: u32) {
    b[index] = color;
}

fn draw_objects(
    mut resource: ResMut<SoftBufferResource>,
    options: Res<SoftBufferOptions>,
    query: Query<(&Position, &Size, &Color)>,
) {

    let color = 0;

    for i in 10..30 {
        let index = get_index(options.width as usize, i, 50);
        set_pixel(&mut resource.buffer, index, color);
    }
}
