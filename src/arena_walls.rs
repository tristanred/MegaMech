use bevy::{prelude::*, utils::tracing::Instrument, winit::WinitWindows};
use bevy_rapier2d::prelude::*;
use winit::dpi::PhysicalSize;

pub enum WallSide {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM
}

#[derive(Component)]
pub struct ArenaWall {
    pub side: WallSide
}

pub fn setup_arena_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert(ArenaWall { side: WallSide::LEFT })
        .insert_bundle(create_wall_sprite())
        .insert_bundle(create_wall_collider())
        .insert_bundle(create_wall_body())
        .insert(RigidBodyPositionSync::Discrete);

    commands.spawn()
        .insert(ArenaWall { side: WallSide::TOP })
        .insert_bundle(create_wall_sprite())
        .insert_bundle(create_wall_collider())
        .insert_bundle(create_wall_body())
        .insert(RigidBodyPositionSync::Discrete);

    commands.spawn()
        .insert(ArenaWall { side: WallSide::RIGHT })
        .insert_bundle(create_wall_sprite())
        .insert_bundle(create_wall_collider())
        .insert_bundle(create_wall_body())
        .insert(RigidBodyPositionSync::Discrete);

    commands.spawn()
        .insert(ArenaWall { side: WallSide::BOTTOM })
        .insert_bundle(create_wall_sprite())
        .insert_bundle(create_wall_collider())
        .insert_bundle(create_wall_body())
        .insert(RigidBodyPositionSync::Discrete);
}

pub fn update_walls(mut commands: Commands, mut walls: Query<(&ArenaWall, &mut Sprite, &mut ColliderShapeComponent, &mut RigidBodyPositionComponent)>, windows: Res<Windows>) {
    let win = windows.get_primary();

    if let Some(window) = win {
        for (a, mut s, mut cs, mut rp) in walls.iter_mut() {
            move_wall(a, &window, &mut s, &mut cs, &mut rp);
        }
    }
}

fn move_wall(a: &ArenaWall, win: &Window, s: &mut Sprite, cs: &mut ColliderShapeComponent, rp: &mut RigidBodyPositionComponent) {
    match a.side {
        WallSide::LEFT => {
            let height = win.height();
            let width = 20;

            s.color = Color::rgb(1.0, 0.0, 0.0);

            s.custom_size = Some(Vec2::new(width as f32, height as f32));
            cs.0 = ColliderShape::cuboid(width as f32 /2.0, height as f32/2.0);
            rp.position = Vec2::new(-(win.width()/2.0), 0.0).into();
        },
        WallSide::TOP => {
            let height = 20;
            let width = win.width();

            s.color = Color::rgb(0.0, 1.0, 0.0);

            s.custom_size = Some(Vec2::new(width as f32, height as f32));
            cs.0 = ColliderShape::cuboid(width as f32 /2.0, height as f32/2.0);
            rp.position = Vec2::new(0.0, win.height()/2.0).into();
        },
        WallSide::RIGHT => {
            let height = win.height();
            let width = 20;

            s.color = Color::rgb(0.0, 0.0, 1.0);

            s.custom_size = Some(Vec2::new(width as f32, height as f32));
            cs.0 = ColliderShape::cuboid(width as f32 /2.0, height as f32/2.0);
            rp.position = Vec2::new(win.width()/2.0, 0.0).into();
        },
        WallSide::BOTTOM => {
            let height = 20;
            let width = win.width();

            s.color = Color::rgb(1.0, 1.0, 0.0);

            s.custom_size = Some(Vec2::new(width as f32, height as f32));
            cs.0 = ColliderShape::cuboid(width as f32 /2.0, height as f32/2.0);
            rp.position = Vec2::new(0.0, -(win.height()/2.0)).into();
        },
    }
}

fn create_wall_sprite() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0.0, 0.0),
            custom_size: Some(Vec2::new(10.0, 300.0)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn create_wall_collider() -> ColliderBundle {
    ColliderBundle {
        position: Vec2::new(0.0, 0.0).into(),
        ..Default::default()
    }
}

fn create_wall_body() -> RigidBodyBundle {
    RigidBodyBundle {
        position: Vec2::new(0.0, 0.0).into(),
        body_type: RigidBodyTypeComponent(RigidBodyType::Static),
        ..Default::default()
    }
}