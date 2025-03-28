use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
        info!("Despawned Entity: {:?}", main_menu_entity)
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
            MainMenu {},
        ))
        .with_children(|parent| {
            // == Title ==
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Auto,
                        height: Val::Px(120.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|parent| {
                    // center text
                    parent
                        .spawn(Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            margin: UiRect {
                                left: Val::Px(8.0),
                                right: Val::Px(8.0),
                                top: Val::Px(8.0),
                                bottom: Val::Px(8.0),
                            },

                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                Text("You will Survive".to_string()),
                                TextFont {
                                    font_size: 38.0,
                                    ..default()
                                },
                            ));
                        });
                });

            // == Play Button ==
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.8, 0.8)),
                    Button {},
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play Game".to_string()),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                    ));
                });

            // == Quit Button ==
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.8, 0.8)),
                    Button {},
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Quit Game".to_string()),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                    ));
                });
        })
        .id();
    main_menu_entity
}
