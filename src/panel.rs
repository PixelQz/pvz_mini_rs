use bevy::prelude::*;
use bevy_lunex::prelude::*;
use crate::{assets_load::MyAssets, controller::HoverPoint, GameState};


pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectCard>()
        .add_plugins( UiDefaultPlugins)
        .add_plugins(UiDebugPlugin::<MainUi>::new())
        
        // .add_plugins(DefaultPickingPlugins.build().disable::<InputPlugin>())
            .add_systems(OnEnter(GameState::Play), setup)
            .add_systems(
                Update,
                (
                   
                    on_button_handler,
                    on_select_card,
                    on_draw_plant,
                ).run_if(in_state(GameState::Play)),
            )
            ;
    }
}

fn setup(mut commands: Commands,image_assets:Res<MyAssets>) {
    commands.spawn(CursorBundle::default());

     commands.spawn((
            UiTreeBundle::<MainUi>::from(UiTree::new2d("Hello UI!")),
            SourceFromCamera,
        )).with_children(|ui| {
            let root = UiLink::<MainUi>::path("Root");
            ui.spawn((
                root.clone(),
                UiLayout::window().size(Rl((100.0, 100.0))).pack::<Base>(),
            ));
            let button_box = root.add("Box");
            ui.spawn((
                button_box.clone(),
                UiLayout::window()
                    .size(Rl((80.0, 10.0)))
                    .pos(Rl((7.0, 10.0)))
                    .pack::<Base>(),
                UiImage2dBundle::from(image_assets.bar.clone()),
                Panel,
            ));
            let h = 8.0;
            let offset = 5.0;

            let button = button_box.add(format!("Button{:?}", 1));

            ui.spawn((
                button,
                UiLayout::window()
                    .x(Rl(  h + offset))
                    .size(Rl((h,100.0 )))
                    .pack::<Base>(),
                UiColor::<Base>::new(Color::Srgba(Srgba::WHITE)),
                UiColor::<Hover>::new(Color::Srgba(Srgba::RED)),
                UiAnimator::<Hover>::new()
                    .forward_speed(5.0)
                    .backward_speed(1.0),
                UiImage2dBundle::from(image_assets.sunflower_card.clone()),
                Card::sunflower,
                UiClickEmitter::SELF,
               
            ));

        });

}
// 点击按钮包之后更新所选卡片
fn on_button_handler(
    button_query: Query<(&Interaction, &Card), Changed<Interaction>>,
    mut select_card: ResMut<SelectCard>,
    mut events: EventReader<UiClickEvent>, query: Query<&Card>
) {
    for event in events.read() {
        // Get our entity
        if let Ok(card) = query.get(event.target) {
            // Process our button click
            info!("Pressed button: {:?}", card);
            *select_card = SelectCard::Some(*card);
                println!("{:?}",*select_card);
        }
    }
    for (interaction, card) in &button_query {
        match interaction {
            Interaction::Pressed => {
                *select_card = SelectCard::Some(*card);
                println!("{:?}",*select_card);
            }
            _ => (),
        }
    }
}

fn on_select_card(
    mut commands: Commands,
    card_query: Query<(Entity, &Card)>,
    select_card: Res<SelectCard>,
) {
    let SelectCard::Some(select_card) = *select_card else {
        for (entity, _) in &card_query {
            commands.entity(entity).remove::<Outline>();
        }
        return;
    };

    for (entity, card) in &card_query {
        if *card == select_card {
            commands.entity(entity).insert(Outline {
                width: Val::Px(5.0),
                offset: Val::Px(0.0),
                color: Color::WHITE,
            });
        } else {
            commands.entity(entity).remove::<Outline>();
        }
    }
}

fn on_draw_plant(
    mut commands: Commands,
    mut plant_shadow_query: Query<(Entity, &mut Transform), With<PanelPlantShadow>>,
    select_card: Res<SelectCard>,
    hover_point: Res<HoverPoint>,
    game_resources: Res<MyAssets>,
) {
    
    let HoverPoint::Some(hover_point) = *hover_point else {
        return;
    };

    let SelectCard::Some(select_card) = *select_card else {
        for (entity, _) in &plant_shadow_query {
            commands.entity(entity).despawn_recursive();
        }
        return;
    };

    if let Ok((_, mut transform)) = plant_shadow_query.get_single_mut() {
        transform.translation = hover_point.extend(1.5);
    } else {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(64.0, 48.0)),
                    ..default()
                },
                texture: game_resources.type_of(select_card),
                transform: Transform::from_translation(hover_point.extend(1.5)),
                ..default()
            },
            
            PanelPlantShadow,
        ));
    }
}

#[derive(Component)]
pub struct Panel;

#[derive(Component)]
pub struct PanelPlantShadow;

#[derive(Resource, Default,Debug)]
pub enum SelectCard {
    Some(Card),
    #[default]
    None,
}

#[derive(Component, PartialEq, Clone, Copy,Debug)]
pub enum Card {
    peashooter,
    sunflower,
}
