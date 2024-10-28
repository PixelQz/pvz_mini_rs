use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*};
use crate::{ assets_load::MyAssets, controller::HoverPoint, panel::{Card, SelectCard}, setting::{OFFSET_X, OFFSET_Y, TILE_SIZE}, GameState};


pub struct PlayPlugin;
impl Plugin for PlayPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Play), play_start)
        .add_systems(Update, (draw_plant_shadow,grow_plant,on_grow_plant).run_if(in_state(GameState::Play)))
        ;
    
    }
}


fn play_start(
    mut commands: Commands,
    image_assets:Res<MyAssets>
) {
    println!("Play");
    commands.spawn(SpriteBundle{
        texture:image_assets.bg.clone(),
        ..Default::default()
    });
// tile
    for i in 0..5 {
        let i = i as f32;
        for j in 0..9 {
            let j = j as f32;

            let color = if j % 2.0 == 0.0 {
                if i % 2.0 == 0.0 {
                    Color::rgba(0.0, 0.0, 0.0, 0.1)
                } else {
                    Color::rgba(0.0, 0.0, 0.0, 0.1)
                }
            } else {
                if i % 2.0 == 0.0 {
                    Color::rgba(0.0, 0.0, 0.0, 0.1)
                } else {
                    Color::rgba(0.0, 0.0, 0.0, 0.1)
                }
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(TILE_SIZE - Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        j * TILE_SIZE.x - (4.0 * TILE_SIZE.x)-OFFSET_X,
                        i * TILE_SIZE.y - (3.2 * TILE_SIZE.y)+OFFSET_Y,
                        1.0,
                    ),
                    ..default()
                },Tile
               
            ));
        }
    }
}

// 当悬浮在格子上是生成shadow
fn draw_plant_shadow(
    mut commands: Commands,
    tile_query: Query<(Entity, &Transform, Option<&Children>), With<Tile>>,
    plant_shadow_query: Query<Entity, With<TilePlantShadow>>,
    plant_query: Query<Entity, With<Plant>>,
    hover_point: Res<HoverPoint>,
    select_card: Res<SelectCard>,
    game_resources: Res<MyAssets>,
) {
    let HoverPoint::Some(hover_point) = *hover_point else {
        return;
    };
    let SelectCard::Some(select_card) = *select_card else {
        for entity in &plant_shadow_query {
            commands.entity(entity).despawn();
        }
        return;
    };
// 创建鼠标检测盒子
    let mouse_aabb = Aabb2d::new(hover_point, Vec2::new(1.0, 1.0));
// 循环检测每个带有tile组建的格子是否已经生成child（即已经放置了）没有就生成shadow
    for (entity, transform, child) in &tile_query {
        let tile_aabb = Aabb2d::new(transform.translation.xy(), TILE_SIZE / 2.0);
// 检测是否相交，有就在格子生成shadow,没有就在离开格子时（即不相交）将其销毁
        if tile_aabb.intersects(&mouse_aabb) {
            if let Some(children) = child {
                if children
                    .iter()
                    .find(|c| plant_query.get(**c).is_ok())
                    .is_some()
                {
                    break;
                }
                if children
                    .iter()
                    .find(|e| plant_shadow_query.get(**e).is_ok())
                    .is_some()
                {
                    continue;
                }
            }

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.0, 0.0, 0.0, 0.6),
                            custom_size: Some(Vec2::new(64.0, 48.0)),
                            ..default()
                        },
                        texture: game_resources.type_of(select_card),
                        transform: Transform::from_xyz(0.0, 0.0, 0.1),
                        ..default()
                    },
                    TilePlantShadow(select_card),
                ));
            });
        } else {
            if let Some(children) = child {
                if let Some(entity) = children
                    .iter()
                    .find(|c| plant_shadow_query.get(**c).is_ok())
                {
                    if let Ok(entity) = plant_shadow_query.get(*entity) {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

fn grow_plant(
    mut commands: Commands,
    tile_query: Query<Entity, With<Tile>>,
    plant_shadow_query: Query<(&Parent, &TilePlantShadow)>,
    input: Res<ButtonInput<MouseButton>>,
    mut select_card: ResMut<SelectCard>,
) {
    if input.just_pressed(MouseButton::Left) {
        for (parent, tile_plant_shadow) in &plant_shadow_query {
            if let Ok(entity) = tile_query.get(parent.get()) {
                commands
                    .entity(entity)
                    .insert(WantGrowPlant(tile_plant_shadow.0));
                *select_card = SelectCard::None;
            }
        }
    }

    if input.just_pressed(MouseButton::Right) {
        *select_card = SelectCard::None;
    }
}
fn on_grow_plant(
    mut commands: Commands,
    grow_plant_query: Query<(Entity, &WantGrowPlant)>,
    game_resources: Res<MyAssets>,
    
) {
  
  
    for (entity, want_grow_plant) in &grow_plant_query {
        commands
            .entity(entity)
            .remove::<WantGrowPlant>()
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(75.0, 75.0)),
                            ..default()
                        },
                        texture: game_resources.type_of(want_grow_plant.0),
                        transform: Transform::from_xyz(0.0, 0.0, 0.1),
                        ..default()
                    },
                    Plant,
                   
                ));
            });
    }
}
#[derive(Component)]
struct Bar;
#[derive(Component)]
struct Tile;

#[derive(Component)]
pub struct TilePlantShadow(pub Card);

#[derive(Component)]
pub struct WantGrowPlant(pub Card);

#[derive(Component)]
pub struct Plant;