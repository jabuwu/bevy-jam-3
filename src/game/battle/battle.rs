use bevy::prelude::*;

use crate::{AddFixedEvent, AppState, AssetLibrary, EventSet, FixedInput, UnitSpawnEvent};

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum BattleSystem {
    Enter,
    Start,
    Update,
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(
                battle_enter
                    .in_schedule(OnEnter(AppState::GameBattle))
                    .in_set(BattleSystem::Enter),
            );
        }
        app.init_resource::<BattleState>()
            .add_fixed_event::<BattleStartEvent>()
            .add_system(
                battle_start
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::Start)
                    .in_set(EventSet::<UnitSpawnEvent>::Sender),
            )
            .add_system(
                battle_update
                    .run_if(in_state(AppState::GameBattle))
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(BattleSystem::Update),
            );
    }
}

#[derive(Resource)]
pub struct BattleState {
    battle_time: f32,
}

impl Default for BattleState {
    fn default() -> Self {
        Self { battle_time: 1.5 }
    }
}

#[derive(Default)]
pub struct BattleStartEvent;

fn battle_enter(
    mut battle_state: ResMut<BattleState>,
    mut start_events: EventWriter<BattleStartEvent>,
) {
    *battle_state = BattleState::default();
    start_events.send_default();
}

fn battle_start(
    mut start_events: EventReader<BattleStartEvent>,
    mut unit_spawn_events: EventWriter<UnitSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in start_events.iter() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                "Battling!",
                TextStyle {
                    font: asset_library.font_placeholder.clone(),
                    font_size: 72.,
                    color: Color::WHITE,
                    ..Default::default()
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0., 200., 0.),
            ..Default::default()
        });
        unit_spawn_events.send_default();
    }
}

fn battle_update(
    mut battle_state: ResMut<BattleState>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<FixedTime>,
    keys: Res<FixedInput<KeyCode>>,
) {
    battle_state.battle_time -= time.period.as_secs_f32();
    if battle_state.battle_time < 0. || keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::GamePlanning);
    }
}
