mod modular;

use std::collections::{btree_map::Entry, BTreeMap};

use bevy::{
    animation::AnimationPlayer,
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core::Name,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        entity::Entity,
        system::{Commands, Local, Query, Res, ResMut},
    },
    math::Vec3,
    pbr::AmbientLight,
    prelude::SpatialBundle,
    render::{color::Color, texture::ImagePlugin},
    scene::{SceneBundle, SceneSpawner},
    text::TextStyle,
    transform::components::Transform,
    ui::node_bundles::TextBundle,
    DefaultPlugins,
};
#[cfg(feature = "with-inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use modular::*;

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ModularPlugin);
    #[cfg(feature = "with-inspector")]
    app.add_plugins(WorldInspectorPlugin::new());

    // Resources
    app.insert_resource(AmbientLight {
        brightness: 600.,
        color: Color::WHITE,
    });

    // Systems
    app.add_systems(Startup, spawn_text)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_models)
        .add_systems(Startup, spawn_modular)
        .add_systems(Update, cycle_through_animations);

    // Run
    app.run();
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TextBundle::from_section(
        "Cycle through heads with Q and W\n\
        Cycle through bodies with E and R\n\
        Cycle through heads with T and Y\n\
        Cycle through heads with U and I",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0.5, 5.).looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y),
        ..Default::default()
    });
}

fn spawn_modular(
    mut commands: Commands,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    let entity = commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Modular"),
            ModularCharacterHead {
                id: 0,
                instance_id: Some(scene_spawner.spawn(asset_server.load(modular::HEADS[0]))),
                entities: vec![],
            },
            ModularCharacterBody {
                id: 0,
                instance_id: Some(scene_spawner.spawn(asset_server.load(modular::BODIES[0]))),
                entities: vec![],
            },
            ModularCharacterLegs {
                id: 0,
                instance_id: Some(scene_spawner.spawn(asset_server.load(modular::LEGS[0]))),
                entities: vec![],
            },
            ModularCharacterFeet {
                id: 0,
                instance_id: Some(scene_spawner.spawn(asset_server.load(modular::FEET[0]))),
                entities: vec![],
            },
        ))
        .id();
    // Armature
    scene_spawner.spawn_as_child(asset_server.load("Witch.gltf#Scene1"), entity);
}

fn spawn_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Witch.gltf#Scene0"),
            transform: Transform::from_xyz(1., 0., 0.),
            ..Default::default()
        },
        Name::new("Witch"),
    ));
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("SciFi.gltf#Scene0"),
            transform: Transform::from_xyz(-1., 0., 0.),
            ..Default::default()
        },
        Name::new("SciFi"),
    ));
}

fn cycle_through_animations(
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    mut animation_id: Local<BTreeMap<Entity, usize>>,
    asset_loader: Res<AssetServer>,
) {
    for (entity, mut player) in &mut players {
        let next_to_play = match animation_id.entry(entity) {
            Entry::Vacant(e) => {
                e.insert(0);
                Some(0)
            }
            Entry::Occupied(mut e) => {
                if player.is_finished() | player.is_paused() {
                    *e.get_mut() = (e.get() + 1) % 24;
                    Some(*e.get())
                } else {
                    None
                }
            }
        };
        if let Some(next_ani) = next_to_play {
            player
                .play(asset_loader.load(format!("Witch.gltf#Animation{next_ani}")))
                .resume();
        }
    }
}
