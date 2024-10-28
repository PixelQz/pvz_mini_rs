use bevy::prelude::*;
use bevy_lunex::prelude::MainUi;

use crate::GameState;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<HoverPoint>()
        .add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(Update, update_hover_point)
        ;
    }
}
fn setup(mut commands:Commands) {
    commands.spawn(  ( MainUi,Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    },
     MainCamera, 
    ));
}
fn update_hover_point(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window>,
    mut hover_point: ResMut<HoverPoint>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        *hover_point = HoverPoint::None;
        return;
    };

    *hover_point = HoverPoint::Some(point);
}



#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub enum HoverPoint {
    Some(Vec2),
    #[default]
    None,
}
