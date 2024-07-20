mod modular;

use std::{
    collections::{btree_map::Entry, BTreeMap},
    time::Duration,
};

use bevy::{
    animation::AnimationPlayer,
    app::{App, PluginGroup, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    core::Name,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        entity::Entity,
        observer::Trigger,
        system::{Commands, Local, Query, Res, ResMut, Resource},
        world::OnAdd,
    },
    gltf::GltfAssetLabel,
    math::Vec3,
    pbr::AmbientLight,
    prelude::{AnimationGraph, AnimationNodeIndex, AnimationTransitions, SpatialBundle},
    render::texture::ImagePlugin,
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
        .add_systems(Startup, setup_animation_graph)
        .add_systems(Update, cycle_through_animations);

    // Observers
    app.observe(animation_player_added);

    // Run
    app.run();
}

#[derive(Debug, Resource)]
struct AnimationGraphCache {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
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
    scene_spawner.spawn_as_child(
        asset_server.load(GltfAssetLabel::Scene(1).from_asset("Witch.gltf")),
        entity,
    );
}

fn setup_animation_graph(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            (0..24).map(|index| {
                asset_server.load(GltfAssetLabel::Animation(index).from_asset("Witch.gltf"))
            }),
            1.0,
            graph.root,
        )
        .collect();

    let graph_handle = graphs.add(graph);
    commands.insert_resource(AnimationGraphCache {
        animations,
        graph: graph_handle,
    });
}

fn spawn_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Witch.gltf")),
            transform: Transform::from_xyz(1., 0., 0.),
            ..Default::default()
        },
        Name::new("Witch"),
    ));
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("SciFi.gltf")),
            transform: Transform::from_xyz(-1., 0., 0.),
            ..Default::default()
        },
        Name::new("SciFi"),
    ));
}

fn animation_player_added(
    trigger: Trigger<OnAdd, AnimationPlayer>,
    mut commands: Commands,
    graph_cache: Res<AnimationGraphCache>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let mut transitions = AnimationTransitions::new();

    transitions
        .play(
            &mut players.get_mut(trigger.entity()).unwrap(),
            graph_cache.animations[0],
            Duration::ZERO,
        )
        .resume();

    commands
        .entity(trigger.entity())
        .insert(transitions)
        .insert(graph_cache.graph.clone());
}

fn cycle_through_animations(
    mut players: Query<(Entity, &mut AnimationPlayer, &mut AnimationTransitions)>,
    mut animation_id: Local<BTreeMap<Entity, usize>>,
    graph_cache: Res<AnimationGraphCache>,
) {
    for (entity, mut player, mut trasition) in &mut players {
        let next_to_play = match animation_id.entry(entity) {
            Entry::Vacant(e) => {
                e.insert(0);
                Some(0)
            }
            Entry::Occupied(mut e) => {
                if player.all_finished() | player.all_paused() {
                    *e.get_mut() = (e.get() + 1) % 24;
                    Some(*e.get())
                } else {
                    None
                }
            }
        };
        if let Some(next_ani) = next_to_play {
            trasition
                .play(
                    &mut player,
                    graph_cache.animations[next_ani],
                    Duration::from_millis(250),
                )
                .resume();
        }
    }
}
