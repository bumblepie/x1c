mod timed_phase;

use timed_phase::TimedPhase;

use rand::thread_rng;
use xcom_1_card::{generate_timed_phase_prompts, PanicLevel, TimedPhasePrompt};
use yew::prelude::*;

enum Msg {
    TimedPhaseCompleted,
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
    link: ComponentLink<Self>,
}
enum Phase {
    // Setup,
    // Prepare for timed phase
    TimedPhase(Vec<TimedPhasePrompt>),
    // Completing timed phase
    ResolutionPhase,
    // CleanUp,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let game_state = GameState {
            round: 1,
            alien_base_discovered: false,
            panic_level: PanicLevel::Yellow,
            ufos_left: 0,
        };
        let prompts = generate_timed_phase_prompts(
            game_state.round,
            &game_state.panic_level,
            game_state.ufos_left,
            game_state.round == 5,
            &mut thread_rng(),
        );
        Self {
            phase: Phase::TimedPhase(prompts),
            game_state,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TimedPhaseCompleted => match self.phase {
                Phase::TimedPhase(_) => {
                    self.phase = Phase::ResolutionPhase;
                    return true;
                }
                _ => return false,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.phase {
            Phase::TimedPhase(ref prompts) => {
                html! {
                    <TimedPhase prompts={prompts.clone()} completed_callback=self.link.callback(|_| Msg::TimedPhaseCompleted)/>
                }
            }
            Phase::ResolutionPhase => {
                html! {
                    <p>{"Resolution phase"}</p>
                }
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
