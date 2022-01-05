mod common;
mod resolution_phase;
mod setup;
mod tech_reference;
mod timed_phase;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use rand::thread_rng;
use resolution_phase::ResolutionPhase;
use serde::{Deserialize, Serialize};
use setup::SetupComponent;
use timed_phase::TimedPhase;
use xcom_1_card::{
    generate_timed_phase_prompts, GameResult, PanicLevel, ResolutionPhasePrompt, TimedPhasePrompt,
};
use yew::prelude::*;

const GAMESTATE_KEY: &str = "GameState";
const PHASE_KEY: &str = "Phase";

enum Msg {
    BeginSetup,
    BeginGame,
    ContinueGame,
    EnterTimedPhase,
    TimedPhaseCompleted,
    EnterResolutionPhase,
    AlienBaseDiscovered,
    ResolutionPhaseCompleted {
        panic_level: PanicLevel,
        ufos_left: u32,
    },
    GameCompleted(GameResult),
    UndoGameCompleted,
    ReturnToMainMenu,
    ClearSavedGame,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GameState {
    round: u32,
    alien_base_discovered: bool,
    panic_level: PanicLevel,
    ufos_left: u32,
}

impl GameState {
    fn new() -> Self {
        Self {
            round: 1,
            alien_base_discovered: false,
            panic_level: PanicLevel::Yellow,
            ufos_left: 0,
        }
    }
}

struct Model {
    phase: Phase,
    game_state: GameState,
    resolution_phase_starting_prompt: ResolutionPhasePrompt,
}

impl Model {
    fn save(&self) -> Result<(), StorageError> {
        LocalStorage::set(GAMESTATE_KEY, &self.game_state)?;
        LocalStorage::set(PHASE_KEY, &self.phase)?;
        Ok(())
    }

    fn load() -> Result<Self, StorageError> {
        let game_state = LocalStorage::get(GAMESTATE_KEY)?;
        let phase = LocalStorage::get(PHASE_KEY)?;
        Ok(Self {
            game_state,
            phase,
            resolution_phase_starting_prompt: ResolutionPhasePrompt::start(),
        })
    }

    fn clear_saved_game() {
        LocalStorage::clear()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Phase {
    MainMenu,
    Setup,
    PrepareForTimedPhase,
    TimedPhase(Vec<TimedPhasePrompt>),
    PrepareForResolutionPhase,
    ResolutionPhase,
    GameCompleted(GameResult),
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            phase: Phase::MainMenu,
            game_state: GameState::new(),
            resolution_phase_starting_prompt: ResolutionPhasePrompt::start(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BeginSetup => {
                self.phase = Phase::Setup;
                true
            }
            Msg::BeginGame => {
                self.phase = Phase::PrepareForTimedPhase;
                true
            }
            Msg::EnterTimedPhase => {
                let prompts = generate_timed_phase_prompts(
                    self.game_state.round,
                    &self.game_state.panic_level,
                    self.game_state.ufos_left,
                    self.game_state.round == 5,
                    &mut thread_rng(),
                );
                self.phase = Phase::TimedPhase(prompts);
                if let Err(_) = self.save() {
                    log::error!("Error saving game");
                }
                true
            }
            Msg::AlienBaseDiscovered => {
                self.game_state.alien_base_discovered = true;
                false
            }
            Msg::TimedPhaseCompleted => {
                self.phase = Phase::PrepareForResolutionPhase;
                if let Err(_) = self.save() {
                    log::error!("Error saving game");
                }
                true
            }
            Msg::EnterResolutionPhase => {
                self.resolution_phase_starting_prompt = ResolutionPhasePrompt::start();
                self.phase = Phase::ResolutionPhase;
                if let Err(_) = self.save() {
                    log::error!("Error saving game");
                }
                true
            }
            Msg::ResolutionPhaseCompleted {
                panic_level,
                ufos_left,
            } => {
                self.phase = Phase::PrepareForTimedPhase;
                self.game_state = GameState {
                    panic_level,
                    ufos_left,
                    round: self.game_state.round + 1,
                    ..self.game_state
                };
                if let Err(_) = self.save() {
                    log::error!("Error saving game");
                }
                true
            }
            Msg::GameCompleted(result) => {
                self.phase = Phase::GameCompleted(result);
                true
            }
            Msg::UndoGameCompleted => {
                self.phase = Phase::ResolutionPhase;
                self.resolution_phase_starting_prompt = ResolutionPhasePrompt::AskForBoardState;
                true
            }
            Msg::ReturnToMainMenu => {
                self.phase = Phase::MainMenu;
                self.game_state = GameState::new();
                self.resolution_phase_starting_prompt = ResolutionPhasePrompt::start();
                true
            }
            Msg::ContinueGame => {
                if let Ok(model) = Self::load() {
                    self.phase = model.phase;
                    self.game_state = model.game_state;
                    true
                } else {
                    false
                }
            }
            Msg::ClearSavedGame => {
                Self::clear_saved_game();
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class="main">
                {
                    match self.phase {
                        Phase::MainMenu => {
                            html! {
                                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/alien-head.png)">
                                    <div class="prepare-screen-text">{ "X-1C" }</div>
                                    <div class="prepare-screen-button-container">
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::BeginSetup)}> {"Instructions"}</button>
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().batch_callback(|_| vec![Msg::ClearSavedGame, Msg::BeginGame])}> {"New Game"}</button>
                                        {
                                            if Self::load().is_ok() {
                                                html!{
                                                    <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::ContinueGame)}> {"Continue"}</button>
                                                }
                                            } else {
                                                html!{}
                                            }
                                        }
                                    </div>
                                </div>
                            }
                        }
                        Phase::Setup => {
                            html!{
                                <SetupComponent on_main_menu={ctx.link().callback(|_| Msg::ReturnToMainMenu)} on_completed={ctx.link().batch_callback(|_| vec![Msg::ClearSavedGame, Msg::BeginGame])}/>
                            }
                        }
                        Phase::PrepareForTimedPhase => {
                            html! {
                                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-with-sunset.png)">
                                    <div class="prepare-screen-text">{ "Prepare for Timed Phase" }</div>
                                    <div class="prepare-screen-button-container">
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::EnterTimedPhase)}> {"Enter Timed Phase"}</button>
                                    </div>
                                </div>
                            }
                        }
                        Phase::TimedPhase(ref prompts) => {
                            html! {
                                <TimedPhase
                                    prompts={prompts.clone()}
                                    round={self.game_state.round}
                                    on_completed={ctx.link().callback(|_| Msg::TimedPhaseCompleted)}
                                    on_alien_base_discovered={ctx.link().callback(|_| Msg::AlienBaseDiscovered)}
                                />
                            }
                        },
                        Phase::PrepareForResolutionPhase => {
                            html! {

                                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-over-city.png)">
                                    <div class="prepare-screen-text">{ "Prepare for Resolution Phase" }</div>
                                    <div class="prepare-screen-button-container">
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::EnterResolutionPhase)}> {"Enter Resolution Phase"}</button>
                                    </div>
                                </div>
                            }
                        }
                        Phase::ResolutionPhase => {
                            html! {
                                <ResolutionPhase
                                    starting_prompt={self.resolution_phase_starting_prompt.clone()}
                                    panic_level={self.game_state.panic_level.clone()}
                                    ufos_left={self.game_state.ufos_left}
                                    alien_base_discovered={self.game_state.alien_base_discovered}
                                    round={self.game_state.round}
                                    on_completed={ctx.link().callback(|(panic_level, ufos_left)| Msg::ResolutionPhaseCompleted {
                                        panic_level,
                                        ufos_left,
                                    })}
                                    on_game_end={ctx.link().callback(|result| Msg::GameCompleted(result))}
                                />
                            }
                        },
                        Phase::GameCompleted(ref result) => {
                            html!{
                                <div class="background-image prepare-screen" style={format!("background-image: url({})", image_for_result(result))}>
                                    <div class="prepare-screen-text">{ format!("{}", result) }</div>
                                    <div class="prepare-screen-button-container">
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::UndoGameCompleted)} >{ "Back" }</button>
                                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().batch_callback(|_| vec![Msg::ClearSavedGame, Msg::ReturnToMainMenu])} >{ "Quit" }</button>
                                    </div>
                                </div>
                            }
                        },
                    }
                }
            </div>
            </>
        }
    }
}

fn image_for_result(result: &GameResult) -> String {
    match result {
        GameResult::Victory | GameResult::PyrrhicVictory => {
            "assets/background-art/alien-base-destroyed.png"
        }
        GameResult::Defeat => "assets/background-art/alien-base-heart.png",
    }
    .to_owned()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
