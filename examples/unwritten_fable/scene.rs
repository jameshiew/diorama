//! Scene construction for the Unwritten Fable example.

use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::light::NotShadowCaster;
use bevy::math::Vec4;
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use diorama::picking::Hint;
use diorama::player::Player;
use examples_common::noise::Perlin;

use crate::animation::{DriftMote, GlyphFlight, InkDrip, PulseLight, QuillMotion};
use crate::materials::{InkFlowData, InkFlowMaterial, StoryPageData, StoryPageMaterial};

/// Deep indigo void around the desk - the unlit corners of a study at night.
const CLEAR_COLOR: Color = Color::srgb(0.016, 0.014, 0.045);

/// The open spread of the book: x spans both pages, z runs down the page.
const PAGE_SPREAD_WIDTH: f32 = 96.0;
const PAGE_DEPTH: f32 = 64.0;
const PAGE_HALF_WIDTH: f32 = PAGE_SPREAD_WIDTH / 2.0;
const PAGE_SUBDIVISIONS: u32 = 120;
const PAGE_SEED: u32 = 5;

/// World-space x where the story is currently being written.
const FRONTIER_X: f32 = 14.0;

/// Top surface of the writing desk, far below the open book.
const DESK_TOP_Y: f32 = -12.0;

/// Where the great quill hovers, nib down over the frontier.
const QUILL_ANCHOR: Vec3 = Vec3::new(14.0, 7.0, 3.0);

/// Approximate world position of the quill nib while it writes.
const GLYPH_SOURCE: Vec3 = Vec3::new(15.2, 2.8, 3.0);

/// Spawn on the written page near the river bank, looking across the ink
/// river to the quill and the frontier.
const PLAYER_SPAWN: Vec3 = Vec3::new(-14.0, 3.2, 19.0);
const PLAYER_LOOK_AT: Vec3 = Vec3::new(14.0, 3.5, 0.0);

struct Palette {
    desk: Handle<StandardMaterial>,
    cover: Handle<StandardMaterial>,
    gold_trim: Handle<StandardMaterial>,
    page_block: Handle<StandardMaterial>,
    leathers: [Handle<StandardMaterial>; 6],
    trunk: Handle<StandardMaterial>,
    canopies: [Handle<StandardMaterial>; 3],
    birch: Handle<StandardMaterial>,
    ribbon: Handle<StandardMaterial>,
    quill_white: Handle<StandardMaterial>,
    nib_gold: Handle<StandardMaterial>,
    ink_dark: Handle<StandardMaterial>,
    glass: Handle<StandardMaterial>,
    wall: Handle<StandardMaterial>,
    roof: Handle<StandardMaterial>,
    door: Handle<StandardMaterial>,
    window_glow: Handle<StandardMaterial>,
    ghost: Handle<StandardMaterial>,
    glyph_gold: Handle<StandardMaterial>,
    mote_dust: Handle<StandardMaterial>,
    starlight: Handle<StandardMaterial>,
    lamp: Handle<StandardMaterial>,
    lamp_halo: Handle<StandardMaterial>,
}

struct Inks {
    page: Handle<StoryPageMaterial>,
    river: Handle<InkFlowMaterial>,
    fall: Handle<InkFlowMaterial>,
    pool: Handle<InkFlowMaterial>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
    mut page_materials: ResMut<Assets<StoryPageMaterial>>,
    mut ink_materials: ResMut<Assets<InkFlowMaterial>>,
) {
    commands.insert_resource(ClearColor(CLEAR_COLOR));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.32, 0.32, 0.50),
        brightness: 190.0,
        affects_lightmapped_meshes: true,
    });

    let noise = Perlin::new(PAGE_SEED);
    let palette = build_palette(&mut standard);
    let inks = build_inks(&mut page_materials, &mut ink_materials);

    spawn_lighting(&mut commands);
    spawn_desk(&mut commands, &mut meshes, &palette, &inks);
    spawn_tome_steps(&mut commands, &mut meshes, &palette);
    spawn_book(&mut commands, &mut meshes, &palette, &inks, &noise);
    spawn_ink(&mut commands, &mut meshes, &inks);
    spawn_quill(&mut commands, &mut meshes, &palette);
    spawn_paper_grove(&mut commands, &mut meshes, &palette, &noise);
    spawn_cottage(&mut commands, &mut meshes, &palette, &noise);
    spawn_bridge(&mut commands, &mut meshes, &palette);
    spawn_bookmark(&mut commands, &mut meshes, &palette);
    spawn_sketch_world(&mut commands, &mut meshes, &palette, &noise);
    spawn_glyphs(&mut commands, &mut meshes, &palette, &noise);
    spawn_sky(&mut commands, &mut meshes, &palette);
}

pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn = Transform::from_translation(PLAYER_SPAWN).looking_at(PLAYER_LOOK_AT, Vec3::Y);
    player.translation = spawn.translation;
    player.rotation = spawn.rotation;
}

fn build_palette(materials: &mut Assets<StandardMaterial>) -> Palette {
    let leather = |materials: &mut Assets<StandardMaterial>, color: Color| {
        materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.04,
            perceptual_roughness: 0.72,
            ..default()
        })
    };
    let canopy = |materials: &mut Assets<StandardMaterial>, color: Color| {
        materials.add(StandardMaterial {
            base_color: color,
            metallic: 0.0,
            perceptual_roughness: 0.95,
            ..default()
        })
    };
    Palette {
        desk: materials.add(StandardMaterial {
            base_color: Color::srgb(0.10, 0.062, 0.042),
            metallic: 0.05,
            perceptual_roughness: 0.82,
            ..default()
        }),
        cover: leather(materials, Color::srgb(0.30, 0.08, 0.07)),
        gold_trim: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.66, 0.25),
            emissive: Color::srgb(0.45, 0.32, 0.08).into(),
            metallic: 0.85,
            perceptual_roughness: 0.35,
            ..default()
        }),
        page_block: materials.add(StandardMaterial {
            base_color: Color::srgb(0.87, 0.82, 0.70),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        }),
        leathers: [
            leather(materials, Color::srgb(0.42, 0.10, 0.10)),
            leather(materials, Color::srgb(0.10, 0.26, 0.14)),
            leather(materials, Color::srgb(0.10, 0.14, 0.32)),
            leather(materials, Color::srgb(0.26, 0.12, 0.30)),
            leather(materials, Color::srgb(0.45, 0.32, 0.18)),
            leather(materials, Color::srgb(0.22, 0.24, 0.28)),
        ],
        trunk: materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.38, 0.26),
            perceptual_roughness: 0.92,
            ..default()
        }),
        canopies: [
            canopy(materials, Color::srgb(0.45, 0.62, 0.42)),
            canopy(materials, Color::srgb(0.85, 0.55, 0.55)),
            canopy(materials, Color::srgb(0.55, 0.75, 0.60)),
        ],
        birch: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.78, 0.64),
            emissive: Color::srgb(0.05, 0.045, 0.035).into(),
            perceptual_roughness: 0.88,
            ..default()
        }),
        ribbon: materials.add(StandardMaterial {
            base_color: Color::srgb(0.62, 0.10, 0.14),
            emissive: Color::srgb(0.10, 0.015, 0.02).into(),
            metallic: 0.1,
            perceptual_roughness: 0.45,
            ..default()
        }),
        quill_white: materials.add(StandardMaterial {
            base_color: Color::srgb(0.96, 0.95, 0.92),
            emissive: Color::srgb(0.14, 0.13, 0.17).into(),
            perceptual_roughness: 0.6,
            ..default()
        }),
        nib_gold: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.7, 0.28),
            emissive: Color::srgb(0.6, 0.42, 0.10).into(),
            metallic: 0.9,
            perceptual_roughness: 0.3,
            ..default()
        }),
        ink_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.03, 0.08),
            metallic: 0.2,
            perceptual_roughness: 0.2,
            ..default()
        }),
        glass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.07, 0.12),
            metallic: 0.1,
            perceptual_roughness: 0.22,
            ..default()
        }),
        wall: materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.84, 0.74),
            perceptual_roughness: 0.95,
            ..default()
        }),
        roof: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.30, 0.26),
            perceptual_roughness: 0.9,
            ..default()
        }),
        door: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.20, 0.12),
            perceptual_roughness: 0.85,
            ..default()
        }),
        window_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.8, 0.45),
            emissive: Color::srgb(2.4, 1.4, 0.5).into(),
            unlit: true,
            ..default()
        }),
        ghost: materials.add(StandardMaterial {
            base_color: Color::srgba(0.88, 0.90, 0.96, 0.30),
            emissive: Color::srgb(0.05, 0.06, 0.09).into(),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        glyph_gold: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.85, 0.4),
            emissive: Color::srgb(1.7, 1.25, 0.42).into(),
            unlit: true,
            ..default()
        }),
        mote_dust: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.92, 0.7),
            emissive: Color::srgb(1.3, 1.05, 0.55).into(),
            unlit: true,
            ..default()
        }),
        starlight: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::srgb(0.85, 0.82, 0.72).into(),
            unlit: true,
            ..default()
        }),
        lamp: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.95, 0.8),
            emissive: Color::srgb(2.3, 1.95, 1.4).into(),
            unlit: true,
            ..default()
        }),
        lamp_halo: materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.9, 0.6, 0.16),
            emissive: Color::srgb(0.5, 0.4, 0.2).into(),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
    }
}

fn build_inks(
    page_materials: &mut Assets<StoryPageMaterial>,
    ink_materials: &mut Assets<InkFlowMaterial>,
) -> Inks {
    let base_ink = Vec4::new(0.015, 0.02, 0.06, 0.96);
    let glint = Vec4::new(1.5, 1.15, 0.45, 1.0);
    Inks {
        page: page_materials.add(StoryPageMaterial {
            data: StoryPageData {
                parchment_color: Vec4::new(0.66, 0.56, 0.38, 1.0),
                ink_color: Vec4::new(0.03, 0.04, 0.11, 1.0),
                glow_color: Vec4::new(1.35, 1.0, 0.4, 1.0),
                sketch_color: Vec4::new(0.35, 0.38, 0.45, 1.0),
                frontier_x: FRONTIER_X,
                band_width: 4.2,
                line_spacing: 1.35,
                script_scale: 2.6,
            },
        }),
        river: ink_materials.add(InkFlowMaterial {
            data: InkFlowData {
                base_color: base_ink,
                glint_color: glint,
                flow_speed: 0.8,
                streak_scale: 0.55,
                fall_mode: 0.0,
                phase: 0.0,
            },
        }),
        fall: ink_materials.add(InkFlowMaterial {
            data: InkFlowData {
                base_color: base_ink,
                glint_color: glint,
                flow_speed: 2.6,
                streak_scale: 0.5,
                fall_mode: 1.0,
                phase: 1.3,
            },
        }),
        pool: ink_materials.add(InkFlowMaterial {
            data: InkFlowData {
                base_color: base_ink,
                glint_color: glint,
                flow_speed: 0.18,
                streak_scale: 0.8,
                fall_mode: 0.0,
                phase: 2.1,
            },
        }),
    }
}

fn spawn_lighting(commands: &mut Commands) {
    // Warm light slanting down from the reading lamp, in over the front
    // edge of the book so that faces toward the opening view catch it.
    commands.spawn((
        Name::new("Lamplight"),
        DirectionalLight {
            illuminance: 5_200.0,
            color: Color::srgb(1.0, 0.92, 0.78),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-60.0, 110.0, 120.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Golden shimmer along the frontier, where new words cool from the nib.
    for (index, z) in [-18.0, 2.0, 22.0].into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Frontier Glow {}", index.saturating_add(1))),
            PointLight {
                intensity: 3_000.0,
                range: 24.0,
                radius: 0.4,
                color: Color::srgb(1.0, 0.82, 0.4),
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(FRONTIER_X, 3.5, z),
            PulseLight {
                base_intensity: 3_000.0,
                amplitude: 1_400.0,
                speed: 0.9,
                phase: index as f32 * 2.1,
            },
        ));
    }

    // Faint golden glow where the inkfall meets the desk.
    commands.spawn((
        Name::new("Puddle Glow"),
        PointLight {
            intensity: 1_300.0,
            range: 16.0,
            radius: 0.6,
            color: Color::srgb(0.9, 0.75, 0.4),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, DESK_TOP_Y + 2.5, 34.0),
        PulseLight {
            base_intensity: 1_300.0,
            amplitude: 500.0,
            speed: 0.6,
            phase: 0.7,
        },
    ));
}

fn spawn_desk(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette, inks: &Inks) {
    commands.spawn((
        Name::new("Writing Desk"),
        Hint::new("An expanse of old walnut, far below the story"),
        Mesh3d(meshes.add(Cuboid::new(420.0, 2.0, 420.0))),
        MeshMaterial3d(palette.desk.clone()),
        Transform::from_xyz(0.0, DESK_TOP_Y - 1.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(420.0, 2.0, 420.0),
    ));

    // Spilled ink where the inkfall lands.
    commands.spawn((
        Name::new("Ink Puddle"),
        Mesh3d(meshes.add(Cylinder::new(7.5, 0.1))),
        MeshMaterial3d(inks.pool.clone()),
        Transform::from_xyz(0.0, DESK_TOP_Y + 0.06, 34.0),
    ));

    // A great inkwell standing open on the desk.
    let well = Vec3::new(-58.0, 0.0, 34.0);
    commands.spawn((
        Name::new("Inkwell"),
        Hint::new("Midnight, bottled"),
        Mesh3d(meshes.add(Cylinder::new(6.5, 10.0))),
        MeshMaterial3d(palette.glass.clone()),
        Transform::from_xyz(well.x, DESK_TOP_Y + 5.0, well.z),
        RigidBody::Static,
        Collider::cylinder(6.5, 10.0),
    ));
    commands.spawn((
        Name::new("Inkwell Shoulder"),
        Mesh3d(meshes.add(Sphere::new(6.5))),
        MeshMaterial3d(palette.glass.clone()),
        Transform::from_xyz(well.x, DESK_TOP_Y + 10.0, well.z)
            .with_scale(Vec3::new(1.0, 0.42, 1.0)),
    ));
    commands.spawn((
        Name::new("Inkwell Neck"),
        Mesh3d(meshes.add(Cylinder::new(2.6, 3.0))),
        MeshMaterial3d(palette.glass.clone()),
        Transform::from_xyz(well.x, DESK_TOP_Y + 12.7, well.z),
    ));
    commands.spawn((
        Name::new("Inkwell Lip"),
        Mesh3d(meshes.add(Torus::new(2.5, 2.95))),
        MeshMaterial3d(palette.gold_trim.clone()),
        Transform::from_xyz(well.x, DESK_TOP_Y + 14.2, well.z),
    ));
    commands.spawn((
        Name::new("Inkwell Ink"),
        Mesh3d(meshes.add(Cylinder::new(2.45, 0.08))),
        MeshMaterial3d(inks.pool.clone()),
        Transform::from_xyz(well.x, DESK_TOP_Y + 13.9, well.z),
    ));
    commands.spawn((
        Name::new("Inkwell Gleam"),
        PointLight {
            intensity: 1_400.0,
            range: 18.0,
            radius: 0.5,
            color: Color::srgb(0.45, 0.55, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(well.x, DESK_TOP_Y + 16.0, well.z),
        PulseLight {
            base_intensity: 1_400.0,
            amplitude: 600.0,
            speed: 0.5,
            phase: 3.3,
        },
    ));
}

/// Stacks of giant closed books beside the right page - a staircase for
/// readers who fall off the story.
fn spawn_tome_steps(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    // (size, position-above-desk, yaw, leather index)
    let books: [(Vec3, Vec3, f32, usize); 6] = [
        (
            Vec3::new(26.0, 3.2, 18.0),
            Vec3::new(70.0, 1.6, 26.0),
            0.05,
            0,
        ),
        (
            Vec3::new(24.0, 3.5, 16.0),
            Vec3::new(66.0, 1.75, 10.0),
            -0.08,
            2,
        ),
        (
            Vec3::new(22.0, 3.5, 14.5),
            Vec3::new(66.5, 5.25, 10.2),
            0.06,
            4,
        ),
        (
            Vec3::new(20.0, 3.5, 13.0),
            Vec3::new(62.0, 1.75, -6.0),
            0.1,
            1,
        ),
        (
            Vec3::new(19.0, 3.5, 12.5),
            Vec3::new(62.3, 5.25, -5.8),
            -0.05,
            3,
        ),
        (
            Vec3::new(18.0, 3.5, 12.0),
            Vec3::new(61.8, 8.75, -6.1),
            0.08,
            5,
        ),
    ];
    for (index, (size, offset, yaw, leather)) in books.into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Closed Tome {}", index.saturating_add(1))),
            Hint::new("A fable already told"),
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(palette.leathers[leather].clone()),
            Transform::from_xyz(offset.x, DESK_TOP_Y + offset.y, offset.z)
                .with_rotation(Quat::from_rotation_y(yaw)),
            RigidBody::Static,
            Collider::cuboid(size.x, size.y, size.z),
        ));
    }
}

fn spawn_book(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    inks: &Inks,
    noise: &Perlin,
) {
    // Leather cover, slightly larger than the page block.
    commands.spawn((
        Name::new("Book Cover"),
        Hint::new("Bound in red leather, older than its reader"),
        Mesh3d(meshes.add(Cuboid::new(102.0, 1.6, 70.0))),
        MeshMaterial3d(palette.cover.clone()),
        Transform::from_xyz(0.0, -11.2, 0.0),
        RigidBody::Static,
        Collider::cuboid(102.0, 1.6, 70.0),
    ));

    // Gold tooling along the cover's edges.
    for (size, position) in [
        (Vec3::new(102.0, 0.12, 0.7), Vec3::new(0.0, -10.35, 34.4)),
        (Vec3::new(102.0, 0.12, 0.7), Vec3::new(0.0, -10.35, -34.4)),
        (Vec3::new(0.7, 0.12, 70.0), Vec3::new(50.4, -10.35, 0.0)),
        (Vec3::new(0.7, 0.12, 70.0), Vec3::new(-50.4, -10.35, 0.0)),
    ] {
        commands.spawn((
            Name::new("Cover Tooling"),
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(palette.gold_trim.clone()),
            Transform::from_translation(position),
        ));
    }

    // The closed pages beneath the open spread.
    commands.spawn((
        Name::new("Page Block"),
        Mesh3d(meshes.add(Cuboid::new(96.0, 8.8, 64.0))),
        MeshMaterial3d(palette.page_block.clone()),
        Transform::from_xyz(0.0, -6.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(96.0, 8.8, 64.0),
    ));

    // The open spread itself: a displaced plane shaped like resting paper,
    // with a gutter valley for the ink river.
    let mut page_mesh = Plane3d::default()
        .mesh()
        .size(PAGE_SPREAD_WIDTH, PAGE_DEPTH)
        .subdivisions(PAGE_SUBDIVISIONS)
        .build();
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        page_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for position in positions.iter_mut() {
            position[1] = page_height(noise, position[0], position[2]);
        }
    }
    page_mesh.compute_normals();

    let vertex_positions: Vec<Vec3> = page_mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attribute| match attribute {
            VertexAttributeValues::Float32x3(positions) => Some(
                positions
                    .iter()
                    .map(|position| Vec3::from_array(*position))
                    .collect(),
            ),
            _ => None,
        })
        .unwrap_or_default();
    let indices: Vec<[u32; 3]> = page_mesh
        .indices()
        .map(|indices| match indices {
            Indices::U32(indices) => indices
                .chunks(3)
                .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                .collect(),
            Indices::U16(indices) => indices
                .chunks(3)
                .map(|chunk| {
                    [
                        u32::from(chunk[0]),
                        u32::from(chunk[1]),
                        u32::from(chunk[2]),
                    ]
                })
                .collect(),
        })
        .unwrap_or_default();

    commands.spawn((
        Name::new("The Open Page"),
        Hint::new("The story so far"),
        Mesh3d(meshes.add(page_mesh)),
        MeshMaterial3d(inks.page.clone()),
        RigidBody::Static,
        Collider::trimesh(vertex_positions, indices),
    ));

    // Where the pages fold together at the back of the spine.
    commands.spawn((
        Name::new("Spine Fold"),
        Mesh3d(meshes.add(Cuboid::new(8.0, 2.6, 5.0))),
        MeshMaterial3d(palette.page_block.clone()),
        Transform::from_xyz(0.0, -0.8, -30.5),
        RigidBody::Static,
        Collider::cuboid(8.0, 2.6, 5.0),
    ));
}

fn spawn_ink(commands: &mut Commands, meshes: &mut Assets<Mesh>, inks: &Inks) {
    // The river of ink running down the spine gutter.
    commands.spawn((
        Name::new("Ink River"),
        Hint::new("The story's own current"),
        Mesh3d(meshes.add(Cuboid::new(4.4, 0.12, 62.0))),
        MeshMaterial3d(inks.river.clone()),
        Transform::from_xyz(0.0, -0.96, 1.0),
    ));

    // The inkfall, pouring off the front edge of the book.
    commands.spawn((
        Name::new("Inkfall"),
        Hint::new("Every story spills somewhere"),
        Mesh3d(meshes.add(Cuboid::new(4.4, 12.4, 0.6))),
        MeshMaterial3d(inks.fall.clone()),
        Transform::from_xyz(0.0, -6.6, 32.25),
    ));
}

fn spawn_quill(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let vane_mesh = meshes.add(Sphere::new(1.0));

    commands
        .spawn((
            Name::new("The Quill"),
            Hint::new("It writes the world, and never rests"),
            Transform::from_translation(QUILL_ANCHOR),
            Visibility::default(),
            QuillMotion {
                anchor: QUILL_ANCHOR,
                bob_amplitude: 0.35,
                sway_amplitude: 0.25,
                drift_span: 11.0,
                write_speed: 7.0,
                phase: 0.0,
            },
        ))
        .with_children(|quill| {
            quill.spawn((
                Name::new("Quill Shaft"),
                Mesh3d(meshes.add(Cylinder::new(0.13, 9.0))),
                MeshMaterial3d(palette.quill_white.clone()),
                Transform::from_xyz(0.0, 0.6, 0.0),
            ));
            quill.spawn((
                Name::new("Quill Nib"),
                Mesh3d(meshes.add(Cone::new(0.42, 1.5))),
                MeshMaterial3d(palette.nib_gold.clone()),
                Transform::from_xyz(0.0, -4.2, 0.0).with_rotation(Quat::from_rotation_x(PI)),
            ));
            quill.spawn((
                Name::new("Quill Ink Bead"),
                Mesh3d(meshes.add(Sphere::new(0.24))),
                MeshMaterial3d(palette.ink_dark.clone()),
                Transform::from_xyz(0.0, -4.85, 0.0),
            ));

            // Feather vanes fanned out along the upper shaft, opening in the
            // y-z plane so the feather shows its profile across the page.
            for index in 0..7 {
                let height = 1.6 + index as f32 * 0.6;
                let grow = 0.8 + index as f32 * 0.12;
                for side in [-1.0f32, 1.0] {
                    quill.spawn((
                        Name::new("Quill Vane"),
                        Mesh3d(vane_mesh.clone()),
                        MeshMaterial3d(palette.quill_white.clone()),
                        Transform::from_xyz(0.0, height + 0.35, side * 0.5 * grow)
                            .with_rotation(Quat::from_rotation_x(-side * 0.65))
                            .with_scale(Vec3::new(0.16, 1.15 * grow, 0.45 * grow)),
                    ));
                }
            }
            quill.spawn((
                Name::new("Quill Plume"),
                Mesh3d(vane_mesh.clone()),
                MeshMaterial3d(palette.quill_white.clone()),
                Transform::from_xyz(0.0, 6.1, 0.0).with_scale(Vec3::new(0.24, 1.5, 0.8)),
            ));

            quill.spawn((
                Name::new("Quill Light"),
                PointLight {
                    intensity: 1_700.0,
                    range: 18.0,
                    radius: 0.3,
                    color: Color::srgb(1.0, 0.9, 0.6),
                    shadows_enabled: false,
                    ..default()
                },
                Transform::from_xyz(0.0, -3.0, 1.0),
                PulseLight {
                    base_intensity: 1_700.0,
                    amplitude: 700.0,
                    speed: 1.3,
                    phase: 1.9,
                },
            ));
        });
}

/// Papercraft trees standing up from the written page like a pop-up book.
fn spawn_paper_grove(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    let spots: [(f32, f32, f32, usize); 11] = [
        (-38.0, 8.0, 1.2, 0),
        (-33.0, -16.0, 1.0, 1),
        (-20.0, 16.0, 0.9, 2),
        (-15.0, -20.0, 1.15, 0),
        (-42.0, -4.0, 0.85, 2),
        (-24.0, 4.0, 0.7, 1),
        (-36.0, 22.0, 1.0, 2),
        (-9.0, -1.0, 0.8, 0),
        (-18.0, -10.0, 0.65, 1),
        (-6.0, 6.0, 0.85, 0),
        (-8.0, 12.0, 0.6, 1),
    ];
    for (index, (x, z, scale, canopy)) in spots.into_iter().enumerate() {
        let ground = page_height(noise, x, z);
        let trunk_height = 3.0 * scale;
        commands.spawn((
            Name::new(format!("Paper Tree Trunk {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cylinder::new(0.32 * scale, trunk_height))),
            MeshMaterial3d(palette.trunk.clone()),
            Transform::from_xyz(x, ground + trunk_height / 2.0, z),
            RigidBody::Static,
            Collider::cylinder(0.32 * scale, trunk_height),
        ));
        commands.spawn((
            Name::new(format!("Paper Tree Canopy {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cone::new(2.0 * scale, 4.2 * scale))),
            MeshMaterial3d(palette.canopies[canopy].clone()),
            Transform::from_xyz(x, ground + trunk_height + 2.1 * scale - 0.4 * scale, z),
        ));
    }
}

fn spawn_cottage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    let (x, z) = (-27.0, -6.0);
    let ground = page_height(noise, x, z);

    commands.spawn((
        Name::new("Storybook Cottage"),
        Hint::new("Somebody's happily-ever-after"),
        Mesh3d(meshes.add(Cuboid::new(6.0, 4.2, 5.2))),
        MeshMaterial3d(palette.wall.clone()),
        Transform::from_xyz(x, ground + 2.1, z),
        RigidBody::Static,
        Collider::cuboid(6.0, 4.2, 5.2),
    ));
    // A diamond-section roof: a cuboid rolled 45 degrees.
    commands.spawn((
        Name::new("Cottage Roof"),
        Mesh3d(meshes.add(Cuboid::new(4.6, 4.6, 5.8))),
        MeshMaterial3d(palette.roof.clone()),
        Transform::from_xyz(x, ground + 5.1, z).with_rotation(Quat::from_rotation_z(PI / 4.0)),
    ));
    commands.spawn((
        Name::new("Cottage Chimney"),
        Mesh3d(meshes.add(Cuboid::new(0.8, 2.4, 0.8))),
        MeshMaterial3d(palette.door.clone()),
        Transform::from_xyz(x + 1.6, ground + 6.6, z - 1.2),
    ));
    commands.spawn((
        Name::new("Cottage Door"),
        Mesh3d(meshes.add(Cuboid::new(1.4, 2.2, 0.15))),
        MeshMaterial3d(palette.door.clone()),
        Transform::from_xyz(x - 0.2, ground + 1.1, z + 2.62),
    ));
    for side in [-1.0f32, 1.0] {
        commands.spawn((
            Name::new("Cottage Window"),
            Mesh3d(meshes.add(Cuboid::new(0.9, 0.9, 0.12))),
            MeshMaterial3d(palette.window_glow.clone()),
            Transform::from_xyz(x + side * 1.9, ground + 2.4, z + 2.62),
        ));
    }
    commands.spawn((
        Name::new("Hearth Light"),
        PointLight {
            intensity: 2_600.0,
            range: 18.0,
            radius: 0.3,
            color: Color::srgb(1.0, 0.7, 0.35),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(x, ground + 2.6, z + 3.4),
        PulseLight {
            base_intensity: 2_600.0,
            amplitude: 900.0,
            speed: 2.3,
            phase: 0.4,
        },
    ));
}

/// A little paper bridge over the ink river.
fn spawn_bridge(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let z = 10.0;

    commands.spawn((
        Name::new("Bridge Deck"),
        Hint::new("Paper holds, if you believe it"),
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.35, 3.2))),
        MeshMaterial3d(palette.birch.clone()),
        Transform::from_xyz(0.0, 0.45, z),
        RigidBody::Static,
        Collider::cuboid(6.0, 0.35, 3.2),
    ));
    for side in [-1.0f32, 1.0] {
        commands.spawn((
            Name::new("Bridge Ramp"),
            Mesh3d(meshes.add(Cuboid::new(3.6, 0.3, 3.2))),
            MeshMaterial3d(palette.birch.clone()),
            Transform::from_xyz(side * 4.4, 0.12, z)
                .with_rotation(Quat::from_rotation_z(side * 0.16)),
            RigidBody::Static,
            Collider::cuboid(3.6, 0.3, 3.2),
        ));
        commands.spawn((
            Name::new("Bridge Rail"),
            Mesh3d(meshes.add(Cuboid::new(11.0, 0.5, 0.14))),
            MeshMaterial3d(palette.birch.clone()),
            Transform::from_xyz(0.0, 1.0, z + side * 1.65),
        ));
    }
}

/// A crimson ribbon draped over the front edge of the written page.
fn spawn_bookmark(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    commands.spawn((
        Name::new("Bookmark"),
        Hint::new("Keeping the reader's place"),
        Mesh3d(meshes.add(Cuboid::new(3.2, 0.14, 12.5))),
        MeshMaterial3d(palette.ribbon.clone()),
        Transform::from_xyz(-24.0, 0.35, 25.5).with_rotation(Quat::from_rotation_x(0.13)),
    ));
    commands.spawn((
        Name::new("Bookmark Tail"),
        Mesh3d(meshes.add(Cuboid::new(3.2, 9.5, 0.14))),
        MeshMaterial3d(palette.ribbon.clone()),
        Transform::from_xyz(-24.0, -5.6, 35.4),
    ));
    commands.spawn((
        Name::new("Bookmark Emblem"),
        Mesh3d(meshes.add(Cuboid::new(3.2, 1.0, 0.16))),
        MeshMaterial3d(palette.gold_trim.clone()),
        Transform::from_xyz(-24.0, -9.9, 35.4),
    ));
}

/// Ghost-pale outlines on the far side of the frontier - the world that has
/// not been written yet.
fn spawn_sketch_world(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    let trees: [(f32, f32, f32); 4] = [
        (26.0, -14.0, 1.1),
        (33.0, 5.0, 0.9),
        (40.0, -3.0, 1.2),
        (29.0, 18.0, 0.8),
    ];
    for (index, (x, z, scale)) in trees.into_iter().enumerate() {
        let ground = page_height(noise, x, z);
        let trunk_height = 3.0 * scale;
        commands.spawn((
            Name::new(format!("Sketch Tree Trunk {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cylinder::new(0.32 * scale, trunk_height))),
            MeshMaterial3d(palette.ghost.clone()),
            Transform::from_xyz(x, ground + trunk_height / 2.0, z),
        ));
        commands.spawn((
            Name::new(format!("Sketch Tree Canopy {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cone::new(2.0 * scale, 4.2 * scale))),
            MeshMaterial3d(palette.ghost.clone()),
            Transform::from_xyz(x, ground + trunk_height + 1.7 * scale, z),
        ));
    }

    let (x, z) = (37.0, 12.0);
    let ground = page_height(noise, x, z);
    commands.spawn((
        Name::new("Unwritten Tower"),
        Hint::new("Not yet written"),
        Mesh3d(meshes.add(Cuboid::new(4.0, 7.0, 4.0))),
        MeshMaterial3d(palette.ghost.clone()),
        Transform::from_xyz(x, ground + 3.5, z),
    ));
    commands.spawn((
        Name::new("Unwritten Tower Roof"),
        Mesh3d(meshes.add(Cuboid::new(3.2, 3.2, 4.0))),
        MeshMaterial3d(palette.ghost.clone()),
        Transform::from_xyz(x, ground + 8.2, z).with_rotation(Quat::from_rotation_z(PI / 4.0)),
    ));
}

/// Golden glyphs streaming from the quill nib to the frontier, ink drips,
/// and lazy dust motes over the written page.
fn spawn_glyphs(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    let glyph_mesh = meshes.add(Cuboid::new(0.62, 0.62, 0.08));
    for index in 0..26u32 {
        let fraction = index as f32 / 26.0;
        let end_z = -26.0 + fraction * 53.0;
        let end_x = FRONTIER_X + (index as f32 * 1.7).sin() * 1.8;
        let end_y = page_height(noise, end_x, end_z) + 0.5;
        commands.spawn((
            Name::new(format!("Glyph {}", index.saturating_add(1))),
            Mesh3d(glyph_mesh.clone()),
            MeshMaterial3d(palette.glyph_gold.clone()),
            Transform::from_translation(GLYPH_SOURCE),
            GlyphFlight {
                start: GLYPH_SOURCE,
                end: Vec3::new(end_x, end_y, end_z),
                arc_height: 2.5 + (index as f32 * 0.91).sin().abs() * 2.5,
                speed: 0.07 + ((index as f32 * 0.37).sin() * 0.5 + 0.5) * 0.06,
                phase: index as f32 * 0.137,
                spin: 1.5 + (index as f32 * 0.53).cos() * 0.8,
            },
        ));
    }

    // Ink drips falling from the nib.
    let drip_mesh = meshes.add(Sphere::new(0.13));
    for index in 0..4u32 {
        commands.spawn((
            Name::new(format!("Ink Drip {}", index.saturating_add(1))),
            Mesh3d(drip_mesh.clone()),
            MeshMaterial3d(palette.ink_dark.clone()),
            Transform::from_translation(GLYPH_SOURCE),
            InkDrip {
                origin: GLYPH_SOURCE - Vec3::new(0.0, 0.2, 0.0),
                fall: 1.9,
                speed: 0.45,
                phase: index as f32 * 0.25,
            },
        ));
    }

    // Dust of old words drifting up off the written page.
    let mote_mesh = meshes.add(Sphere::new(0.16));
    for index in 0..36u32 {
        let t = (index as f32 * 0.618_034).fract();
        let s = (index as f32 * 0.381_966).fract();
        let x = -44.0 + t * 38.0;
        let z = -26.0 + s * 52.0;
        let ground = page_height(noise, x, z);
        commands.spawn((
            Name::new(format!("Word Dust {}", index.saturating_add(1))),
            Mesh3d(mote_mesh.clone()),
            MeshMaterial3d(palette.mote_dust.clone()),
            Transform::from_xyz(x, ground + 1.5, z),
            DriftMote {
                anchor: Vec3::new(x, ground + 1.2, z),
                rise: 4.0 + (index as f32 * 0.77).sin().abs() * 3.0,
                sway: 0.6 + (index as f32 * 0.41).cos().abs() * 0.5,
                speed: 0.05 + ((index as f32 * 0.29).sin() * 0.5 + 0.5) * 0.04,
                phase: index as f32 * 0.211,
                base_scale: 0.5 + (index as f32 * 0.67).sin().abs() * 0.5,
            },
        ));
    }
}

/// The reading lamp's distant glow and a canopy of faint, warm stars.
fn spawn_sky(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    // The lamp sits along the directional light's axis, so it must not cast
    // a shadow or it would eclipse the whole scene.
    commands.spawn((
        Name::new("Lamp Star"),
        Mesh3d(meshes.add(Sphere::new(9.0))),
        MeshMaterial3d(palette.lamp.clone()),
        Transform::from_xyz(-70.0, 115.0, 130.0),
        NotShadowCaster,
    ));
    commands.spawn((
        Name::new("Lamp Halo"),
        Mesh3d(meshes.add(Sphere::new(14.0))),
        MeshMaterial3d(palette.lamp_halo.clone()),
        Transform::from_xyz(-70.0, 115.0, 130.0),
        NotShadowCaster,
    ));

    const GOLDEN_ANGLE: f32 = 2.399_963_1;
    let star_mesh = meshes.add(Sphere::new(0.6));
    for index in 0..150 {
        let fraction = index as f32 / 150.0;
        let y = 0.12 + fraction * 0.85;
        let ring = (1.0 - y * y).max(0.0).sqrt();
        let angle = index as f32 * GOLDEN_ANGLE;
        let radius = 175.0 + (index % 5) as f32 * 5.0;
        let scale = 0.5 + ((index as f32 * 0.83).sin() * 0.5 + 0.5) * 0.6;
        commands.spawn((
            Name::new("Star"),
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(palette.starlight.clone()),
            Transform::from_xyz(
                angle.cos() * ring * radius,
                y * radius,
                angle.sin() * ring * radius,
            )
            .with_scale(Vec3::splat(scale)),
        ));
    }
}

/// Height of the open page spread at a world-space (x, z).
///
/// Each page bows gently upward, droops at its outer edges to drape over the
/// page block, and dips into a gutter valley along the spine where the ink
/// river runs. Clamped so draped corners rest on the page block rim.
fn page_height(noise: &Perlin, x: f32, z: f32) -> f32 {
    let px = x.abs();
    let dome = 1.05 * (PI * (px / PAGE_HALF_WIDTH).clamp(0.0, 1.0)).sin();
    let droop_x = -1.7 * smooth01((px - 42.0) / 6.0);
    // The gutter lip stays high at the front so the river pours over it.
    let droop_z = -1.7 * smooth01((z.abs() - 27.0) / 5.0) * smooth01((px - 4.0) / 4.0);
    let gutter = -1.5 * (-(x / 3.2) * (x / 3.2)).exp();
    let ripple = 0.18 * noise.get([f64::from(x) * 0.07, f64::from(z) * 0.07]) as f32;
    (dome + droop_x + droop_z + gutter + ripple).max(-1.55)
}

fn smooth01(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
