use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;

use crate::{
    in_game_state, AppState, Articy, ArticyDialogueInstruction, DebugDrawSettings, Dialogue,
    DialogueEvent, GameState, PersistentGameState, Script, UnitComposition, UnitKind,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_resource::<PersistentGameState>()
            .add_system(game_dialogue_events)
            .add_system(game_debug);
        if app.world.contains_resource::<State<AppState>>() {
            app.add_system(game_update.run_if(in_game_state));
        }
    }
}

fn game_dialogue_events(
    mut dialogue_events: EventReader<DialogueEvent>,
    mut game_state: ResMut<GameState>,
) {
    for dialogue_event in dialogue_events.iter() {
        match &dialogue_event.instruction {
            ArticyDialogueInstruction::AddUnits(unit_kind, count) => {
                game_state
                    .available_army
                    .mutate_count(*unit_kind, |i| i + *count);
            }
            ArticyDialogueInstruction::SubtractUnits(unit_kind, count) => {
                game_state
                    .available_army
                    .mutate_count(*unit_kind, |i| (i - *count).max(0));
            }
            ArticyDialogueInstruction::AddFood(count) => {
                game_state.food += count;
            }
            ArticyDialogueInstruction::SubtractFood(count) => {
                if *count > game_state.food {
                    game_state.food = 0;
                } else {
                    game_state.food -= count;
                }
            }
            ArticyDialogueInstruction::AddItem(item) => {
                game_state.inventory.add(*item);
            }
            ArticyDialogueInstruction::SetGlobalVariable(name, value) => {
                game_state.global_variables.insert(name.clone(), *value);
            } /*DialogueEvent::GainIntel(unit_kind) => {

                  game_state.intel.can_see[*unit_kind] = true;
              }*/
        }
    }
}

fn game_update(mut next_app_state: ResMut<NextState<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key0) {
        next_app_state.set(AppState::MainMenu);
    }
}

fn game_debug(
    mut contexts: EguiContexts,
    mut debug_draw_settings: ResMut<DebugDrawSettings>,
    mut game_state: ResMut<GameState>,
    mut dialogue: ResMut<Dialogue>,
    articy: Res<Articy>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut debug_draw_settings.draw_hit_boxes, "Draw Hitboxes");
        ui.checkbox(&mut debug_draw_settings.draw_hurt_boxes, "Draw Hurtboxes");
        ui.checkbox(&mut debug_draw_settings.draw_feelers, "Draw Feelers");
        ui.collapsing("Variables", |ui| {
            for (name, value) in game_state.global_variables.iter_mut() {
                ui.checkbox(value, name);
            }
        });
        ui.collapsing("Quest", |ui| {
            ui.label(format!("War Chef: {}", game_state.quest.war_chef));
            ui.label(format!("Battle: {}", game_state.quest.battle));
            if ui.button("next").clicked() {
                game_state.quest.next();
            }
            if ui.button("checkpoint").clicked() {
                game_state.checkpoint();
            }
        });
        ui.collapsing("Units", |ui| {
            if ui.button("-").clicked() && game_state.food > 0 {
                game_state.food -= 1;
            }
            ui.add(egui::DragValue::new(&mut game_state.food).clamp_range(0..=100));
            if ui.button("+").clicked() && game_state.food < 100 {
                game_state.food += 1;
            }
            for unit_kind in UnitKind::iter() {
                ui.horizontal(|ui| {
                    let mut count = game_state.available_army.get_count(unit_kind);
                    if ui.button("-").clicked() && count > 0 {
                        count -= 1;
                    }
                    ui.add(egui::DragValue::new(&mut count).clamp_range(0..=100));
                    if ui.button("+").clicked() && count < 100 {
                        count += 1;
                    }
                    game_state.available_army.set_count(unit_kind, count);
                    ui.label(unit_kind.name_plural());
                });
            }
            if ui.button("WC2 Loadout").clicked() {
                game_state.food = 45;
                game_state.available_army = UnitComposition {
                    peasants: 25,
                    warriors: 3,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                };
            }
            if ui.button("WC3 Loadout").clicked() {
                game_state.food = 40;
                game_state.available_army = UnitComposition {
                    peasants: 26,
                    warriors: 6,
                    archers: 0,
                    mages: 0,
                    brutes: 0,
                };
            }
        });
        ui.collapsing("Dialogues", |ui| {
            for dialogue_str in [
                "WC1B1",
                "WC1B2",
                "WC1B3",
                "WC2B1",
                "WC2B2",
                "WC2B3",
                "WC3B1",
                "WC3B2",
                "WC3B3",
                "WC3B4",
                "WC4B1",
                "WC4B2",
                "WC4B3",
                "WC4B4",
                "WC5B1",
                "WC5B2",
                "WC5B3",
                "BogHardWeeds",
                "CeleryQuartz",
                "CracklingMoss",
                "AxeShrooms",
                "SquirtBlopBerries",
                "FrostyWebStrands",
                "RewindScreen",
                "Tutorial1",
                "Tutorial2",
                "Tutorial3",
            ] {
                if ui.button(dialogue_str).clicked() {
                    dialogue.queue(
                        Script::new(articy.dialogues[dialogue_str].clone()),
                        game_state.as_mut(),
                    );
                }
            }
            if ui.button("Clear").clicked() {
                dialogue.clear();
            }
        });
    });
}
