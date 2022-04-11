use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct BouncyBall;

pub fn setup_bouncy_ball(mut commands: Commands, asset_server: Res<AssetServer>) {

    let rigid_body = RigidBodyBundle {
        position: Vec2::new(100.0, 100.0).into(),
        body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
        velocity: RigidBodyVelocity {
            //angvel: 0.0,
            linvel: Vec2::new(100.0, 100.0).into(),
            ..Default::default()
        }.into(),
        mass_properties: RigidBodyMassPropsComponent {
            ..Default::default()
        },
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::ball(32.0).into(),
        ..Default::default()
    };

    commands.spawn()
        .insert(BouncyBall)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("Circle_x64.png"),
            ..Default::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}