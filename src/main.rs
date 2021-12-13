mod resolution_phase;
mod timed_phase;

use resolution_phase::ResolutionPhase;
use timed_phase::TimedPhase;

use rand::thread_rng;
use xcom_1_card::{
    generate_timed_phase_prompts, GameResult, PanicLevel, ResolutionPhasePrompt, TimedPhasePrompt,
};
use yew::prelude::*;

enum Msg {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GameState {
    round: u32,
    alien_base_discovered: bool,
    panic_level: PanicLevel,
    ufos_left: u32,
}

struct Model {
    phase: Phase,
    game_state: GameState,
    resolution_phase_starting_prompt: ResolutionPhasePrompt,
    link: ComponentLink<Self>,
}
enum Phase {
    // MainMenu,
    // Setup,
    PrepareForTimedPhase,
    TimedPhase(Vec<TimedPhasePrompt>),
    PrepareForResolutionPhase,
    ResolutionPhase,
    GameCompleted(GameResult),
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            phase: Phase::PrepareForTimedPhase,
            game_state: GameState {
                round: 1,
                alien_base_discovered: false,
                panic_level: PanicLevel::Yellow,
                ufos_left: 0,
            },
            resolution_phase_starting_prompt: ResolutionPhasePrompt::start(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EnterTimedPhase => match self.phase {
                Phase::PrepareForTimedPhase => {
                    let prompts = generate_timed_phase_prompts(
                        self.game_state.round,
                        &self.game_state.panic_level,
                        self.game_state.ufos_left,
                        self.game_state.round == 5,
                        &mut thread_rng(),
                    );
                    self.phase = Phase::TimedPhase(prompts);
                    true
                }
                _ => false,
            },
            Msg::AlienBaseDiscovered => {
                self.game_state.alien_base_discovered = true;
                false
            }
            Msg::TimedPhaseCompleted => {
                self.phase = Phase::PrepareForResolutionPhase;
                true
            }
            Msg::EnterResolutionPhase => {
                self.resolution_phase_starting_prompt = ResolutionPhasePrompt::start();
                self.phase = Phase::ResolutionPhase;
                true
            }
            Msg::ResolutionPhaseCompleted {
                panic_level,
                ufos_left,
            } => match self.phase {
                Phase::ResolutionPhase => {
                    self.phase = Phase::PrepareForTimedPhase;
                    self.game_state = GameState {
                        panic_level,
                        ufos_left,
                        round: self.game_state.round + 1,
                        ..self.game_state
                    };
                    true
                }
                _ => false,
            },
            Msg::GameCompleted(result) => {
                self.phase = Phase::GameCompleted(result);
                true
            }
            Msg::UndoGameCompleted => {
                self.phase = Phase::ResolutionPhase;
                self.resolution_phase_starting_prompt = ResolutionPhasePrompt::AskForBoardState;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                // <div>
                //     <p> { format!("Round {}", self.game_state.round) }</p>
                // </div>
                {
                    match self.phase {
                        Phase::PrepareForTimedPhase => {
                            html! {
                                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-with-sunset.png)">
                                    <div class="prepare-screen-text">{ "Prepare for Timed Phase" }</div>
                                    <button class="prepare-screen-button" onclick=self.link.callback(|_| Msg::EnterTimedPhase)> {"Enter Timed Phase"}</button>
                                </div>
                            }
                        }
                        Phase::TimedPhase(ref prompts) => {
                            html! {
                                <TimedPhase
                                    prompts={prompts.clone()}
                                    on_completed=self.link.callback(|_| Msg::TimedPhaseCompleted)
                                    on_alien_base_discovered=self.link.callback(|_| Msg::AlienBaseDiscovered)
                                />
                            }
                        },
                        Phase::PrepareForResolutionPhase => {
                            html! {

                                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-over-city.png)">
                                    <div class="prepare-screen-text">{ "Prepare for Resolution Phase" }</div>
                                    <button class="prepare-screen-button" onclick=self.link.callback(|_| Msg::EnterResolutionPhase)> {"Enter Resolution Phase"}</button>
                                </div>
                            }
                        }
                        Phase::ResolutionPhase => {
                            html! {
                                <ResolutionPhase
                                    starting_prompt=self.resolution_phase_starting_prompt.clone()
                                    panic_level=self.game_state.panic_level.clone()
                                    ufos_left=self.game_state.ufos_left
                                    alien_base_discovered=self.game_state.alien_base_discovered
                                    on_completed=self.link.callback(|(panic_level, ufos_left)| Msg::ResolutionPhaseCompleted {
                                        panic_level,
                                        ufos_left,
                                    })
                                    on_game_end=self.link.callback(|result| Msg::GameCompleted(result))
                                />
                            }
                        },
                        Phase::GameCompleted(ref result) => {
                            html!{
                                <div class="prepare-screen">
                                    <p>{ format!("{:?}", result) }</p>
                                    <button onclick=self.link.callback(|_| Msg::UndoGameCompleted) >{ "Back" }</button>
                                </div>
                            }
                        },
                    }
                }
                // <div>
                //     <p>{ format!("{:?}", self.game_state) }</p>
                // </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
