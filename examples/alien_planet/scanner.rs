use avian3d::prelude::*;
use bevy::prelude::*;
use diorama::player::Player;

use crate::flora::Scannable;

pub struct ScannerPlugin;

impl Plugin for ScannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, scan_system);
    }
}

#[derive(Component)]
struct ScannerUi;

#[derive(Component)]
struct ScannerText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ScannerUi,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Scanning..."),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScannerText,
            ));
        });

    // Crosshair
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::WHITE),
    ));
}

fn scan_system(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    spatial_query: SpatialQuery,
    scannable_query: Query<&Scannable>,
    name_query: Query<&Name>,
    player_query: Query<Entity, With<Player>>,
    mut text_query: Query<&mut Text, With<ScannerText>>,
) {
    let Some((transform, _camera)) = camera_query.iter().next() else {
        return;
    };

    let origin = transform.translation();
    let direction = transform.forward();

    let mut filter = SpatialQueryFilter::default();
    if let Some(player_entity) = player_query.iter().next() {
        filter = filter.with_excluded_entities([player_entity]);
    }

    if let Some(hit) = spatial_query.cast_ray(origin, direction, 100.0, true, &filter) {
        if let Ok(scannable) = scannable_query.get(hit.entity) {
            for mut text in &mut text_query {
                text.0 = format!("Target: {}\n{}", scannable.name, scannable.description);
            }
        } else if let Ok(name) = name_query.get(hit.entity) {
            for mut text in &mut text_query {
                text.0 = format!("Object: {}", name);
            }
        } else {
            for mut text in &mut text_query {
                text.0 = "Unknown Signal".to_string();
            }
        }
    } else {
        for mut text in &mut text_query {
            text.0 = "Scanning...".to_string();
        }
    }
}
