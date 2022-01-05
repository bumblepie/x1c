use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::{Element, HtmlInputElement};
use xcom_1_card::{GameResult, PanicLevel, ResolutionPhasePrompt};
use yew::prelude::*;

use crate::common::{inline_icon_text_phrase, side_buttons};
use crate::tech_reference::TechReference;

const LATEST_PROMPT_INDEX_KEY: &str = "ResolutionPhase_LatestPromptIndex";
const PANIC_LEVEL_INPUT_KEY: &str = "ResolutionPhase_PanicLevelInput";
const UFOS_INPUT_KEY: &str = "ResolutionPhase_UFOsInput";
const ALIEN_BASE_DESTROYED_INPUT_KEY: &str = "ResolutionPhase_AlienBaseDestroyedInput";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanicLevelInput {
    PanicLevel(PanicLevel),
    AlienSpace,
}

impl Into<String> for PanicLevelInput {
    fn into(self) -> String {
        match self {
            PanicLevelInput::PanicLevel(PanicLevel::Yellow) => "yellow",
            PanicLevelInput::PanicLevel(PanicLevel::Orange) => "orange",
            PanicLevelInput::PanicLevel(PanicLevel::Red) => "red",
            PanicLevelInput::AlienSpace => "alien",
        }
        .to_owned()
    }
}

pub struct InvalidPanicLevelString;

impl TryFrom<&str> for PanicLevelInput {
    type Error = InvalidPanicLevelString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "yellow" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Yellow)),
            "orange" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Orange)),
            "red" => Ok(PanicLevelInput::PanicLevel(PanicLevel::Red)),
            "alien" => Ok(PanicLevelInput::AlienSpace),
            _ => Err(InvalidPanicLevelString),
        }
    }
}

pub struct ResolutionPhase {
    prompts: Vec<ResolutionPhasePrompt>,
    prompt_index: usize,
    latest_prompt_index: usize,
    panic_level_input: PanicLevelInput,
    ufos_left_input: u32,
    alien_base_destroyed_input: bool,
    show_tech: bool,
    prompt_details_ref: NodeRef,
}

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
    UpdatePanicLevel(PanicLevelInput),
    IncreaseUFOsLeft,
    DecreaseUFOsLeft,
    UpdateAlienBaseDestroyed(bool),
    CheckGameEnd,
    ToggleTech,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub panic_level: PanicLevel,
    pub ufos_left: u32,
    pub alien_base_discovered: bool,
    pub round: u32,
    pub on_completed: Callback<(PanicLevel, u32)>,
    pub on_game_end: Callback<GameResult>,
}

impl Component for ResolutionPhase {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // Load when component is created
        let latest_prompt_index = LocalStorage::get(LATEST_PROMPT_INDEX_KEY).unwrap_or(0);
        let panic_level_input = LocalStorage::get(PANIC_LEVEL_INPUT_KEY)
            .unwrap_or(PanicLevelInput::PanicLevel(ctx.props().panic_level.clone()));
        let ufos_left_input = LocalStorage::get(UFOS_INPUT_KEY).unwrap_or(ctx.props().ufos_left);
        let alien_base_destroyed_input =
            LocalStorage::get(ALIEN_BASE_DESTROYED_INPUT_KEY).unwrap_or(false);

        Self {
            prompts: ResolutionPhasePrompt::all(),
            prompt_index: latest_prompt_index,
            latest_prompt_index,
            panic_level_input,
            ufos_left_input,
            alien_base_destroyed_input,
            show_tech: false,
            prompt_details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPrompt => {
                if self.prompt_index + 1 < self.prompts.len() {
                    self.prompt_index += 1;
                    if self.prompt_index > self.latest_prompt_index {
                        self.latest_prompt_index = self.prompt_index;
                        if let Err(_) =
                            LocalStorage::set(LATEST_PROMPT_INDEX_KEY, self.latest_prompt_index)
                        {
                            log::error!("Error saving latest prompt index");
                        }
                    }
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    if let PanicLevelInput::PanicLevel(level) = self.panic_level_input.clone() {
                        ctx.props().on_completed.emit((level, self.ufos_left_input));
                        false
                    } else {
                        false
                    }
                }
            }
            Msg::PreviousPrompt => {
                if self.prompt_index > 0 {
                    self.prompt_index -= 1;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::UpdatePanicLevel(panic_level_input) => {
                self.panic_level_input = panic_level_input;
                if let Err(_) =
                    LocalStorage::set(PANIC_LEVEL_INPUT_KEY, self.panic_level_input.clone())
                {
                    log::error!("Error saving panic level input");
                }
                false
            }
            Msg::IncreaseUFOsLeft => {
                if self.ufos_left_input < 18 {
                    self.ufos_left_input += 1;
                }
                if let Err(_) = LocalStorage::set(UFOS_INPUT_KEY, self.ufos_left_input) {
                    log::error!("Error saving UFOs left input");
                }
                true
            }
            Msg::DecreaseUFOsLeft => {
                if self.ufos_left_input > 0 {
                    self.ufos_left_input -= 1;
                }
                if let Err(_) = LocalStorage::set(UFOS_INPUT_KEY, self.ufos_left_input) {
                    log::error!("Error saving UFOs left input");
                }
                true
            }
            Msg::UpdateAlienBaseDestroyed(alien_base_destroyed) => {
                self.alien_base_destroyed_input = alien_base_destroyed;
                if let Err(_) = LocalStorage::set(
                    ALIEN_BASE_DESTROYED_INPUT_KEY,
                    self.alien_base_destroyed_input,
                ) {
                    log::error!("Error saving UFOs left input");
                }
                false
            }
            Msg::CheckGameEnd => {
                if let Some(game_result) = match (
                    self.alien_base_destroyed_input,
                    self.panic_level_input.clone(),
                ) {
                    (true, PanicLevelInput::PanicLevel(_)) => Some(GameResult::Victory),
                    (true, PanicLevelInput::AlienSpace) => Some(GameResult::PyrrhicVictory),
                    (false, PanicLevelInput::AlienSpace) => Some(GameResult::Defeat),
                    (false, PanicLevelInput::PanicLevel(_)) => None,
                } {
                    if let Some(input_index) = self
                        .prompts
                        .iter()
                        .position(|prompt| *prompt == ResolutionPhasePrompt::AskForBoardState)
                    {
                        if let Err(_) = LocalStorage::set(LATEST_PROMPT_INDEX_KEY, input_index) {
                            log::error!("Error saving latest prompt index");
                        }
                    }
                    ctx.props().on_game_end.emit(game_result);
                } else {
                    ctx.link().send_message(Msg::NextPrompt);
                }
                false
            }
            Msg::ToggleTech => {
                self.show_tech = !self.show_tech;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let panic_input_change_callback = ctx.link().batch_callback(|e: Event| {
            if let Some(element) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(panic_level_input) = PanicLevelInput::try_from(element.value().as_str()) {
                    return vec![Msg::UpdatePanicLevel(panic_level_input)];
                }
            }
            return vec![];
        });
        let prompt = &self.prompts[self.prompt_index];
        let main_section = match prompt {
            ResolutionPhasePrompt::AskForBoardState => {
                let current_panic_level: String = self.panic_level_input.clone().into();
                html! {
                    <div class="board-input-container">
                        <div>
                            <div class="board-input-title">{ "Global Panic Level:"} </div>
                            <div class="panic-input-container">
                            {
                                vec!["yellow", "orange", "red", "alien"].into_iter()
                                    .map(|input| html!{
                                        <>
                                            <input
                                                class="panic-input-radio"
                                                type="radio"
                                                id={format!("panic-{}", input)}
                                                name="panic-input"
                                                value={input}
                                                onchange={panic_input_change_callback.clone()}
                                                checked={input == current_panic_level}
                                            />
                                            <label
                                                class="panic-input-label"
                                                for={format!("panic-{}", input)}
                                            >
                                                <img src={format!("assets/icons/panic-input-{}.png", input)}/>
                                            </label>
                                        </>
                                    })
                                    .collect::<Html>()
                            }
                            </div>
                        </div>
                        <div>
                            <div class="board-input-title">{ "UFOs left on map:"} </div>
                            <div class="ufo-input-container">
                                <button class="ufo-input-button" onclick={ctx.link().callback(|_| Msg::DecreaseUFOsLeft)} disabled={self.ufos_left_input < 1}>{"-"}</button>
                                <span class="ufo-input-text" >{ self.ufos_left_input }</span>
                                <button class="ufo-input-button" onclick={ctx.link().callback(|_| Msg::IncreaseUFOsLeft)} disabled={self.ufos_left_input > 17}>{"+"}</button>
                            </div>
                        </div>
                        {
                            if ctx.props().alien_base_discovered {
                            html!{
                                <div class="alien-base-destroyed-input-container">
                                    <label for="alien_base_destroyed_input">{ "Alien Base destroyed?" }</label>
                                    <input
                                    class="alien-base-destroyed-input-checkbox"
                                        type="checkbox"
                                        name="alien_base_destroyed_input"
                                        checked={self.alien_base_destroyed_input}
                                        onchange={ctx.link().batch_callback(move |e: Event| {
                                            if let Some(input_element) = e.target_dyn_into::<HtmlInputElement>() {
                                                return vec![Msg::UpdateAlienBaseDestroyed(input_element.checked())];
                                            }
                                            return vec![];
                                        })}
                                    />
                                </div>
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                }
            }
            _ => html! {
                <>
                    <h1 class="prompt-title">{ prompt.title() }</h1>
                    <div class="prompt-center-area">
                        {side_buttons(ctx.link().callback(|_| Msg::ToggleTech))}
                        {
                            if self.show_tech {
                                html!{
                                    <div class="tech-ref-container">
                                        <TechReference/>
                                    </div>
                                }
                            } else {
                                html!{
                                    <div class="prompt-details" ref={self.prompt_details_ref.clone()}>
                                        <div class="prompt-icons">
                                            {icon_html_for_prompt(&prompt)}
                                        </div>
                                        <div class="prompt-description">
                                            {description_html_for_prompt(&prompt, ctx.props().alien_base_discovered)}
                                        </div>
                                    </div>
                                }
                            }
                        }
                    </div>
                </>
            },
        };
        let next_callback = match prompt {
            ResolutionPhasePrompt::AskForBoardState => ctx.link().callback(|_| Msg::CheckGameEnd),
            _ => ctx.link().callback(|_| Msg::NextPrompt),
        };
        html! {
            <>
                {main_section}
                <div class="bottom-panel">
                    <button class="button-back" onclick={ctx.link().callback(|_| Msg::PreviousPrompt)} disabled={ self.show_tech || self.prompt_index < 1}>{ "Back" }</button>
                    <div class="round">
                        {format!("Round {}", ctx.props().round)}
                    </div>
                    <button class="button-done" onclick={next_callback} disabled={ self.show_tech }>{ "Done" }</button>
                </div>
            </>
        }
    }
}

fn icon_html_for_prompt(prompt: &ResolutionPhasePrompt) -> Html {
    match prompt {
        ResolutionPhasePrompt::AuditSpending => html! {
            <img class="prompt-icon" src="assets/icons/audit.png"/>
        },
        ResolutionPhasePrompt::ResolveResearch => html! {
            <img class="prompt-icon" src="assets/icons/research.png"/>
        },
        ResolutionPhasePrompt::ResolveUFODefense => html! {
            <img class="prompt-icon" src="assets/icons/interceptor.png"/>
        },
        ResolutionPhasePrompt::IncreasePanic => html! {
            <img class="prompt-icon" src="assets/icons/alien.png"/>
        },
        ResolutionPhasePrompt::AskForBoardState => html! {},
        ResolutionPhasePrompt::ResolveContinentBonuses => html! {
            <>
                <img class="prompt-icon" src="assets/icons/america.png"/>
                <img class="prompt-icon" src="assets/icons/africa.png"/>
                <img class="prompt-icon" src="assets/icons/eurasia.png"/>
            </>
        },
        ResolutionPhasePrompt::CleanUp => html! {
            <img class="prompt-icon" src="assets/icons/cleanup.png"/>
        },
        ResolutionPhasePrompt::PurchaseReplacementForces => html! {
            <img class="prompt-icon" src="assets/icons/replenish.png"/>
        },
    }
}

fn description_html_for_prompt(
    prompt: &ResolutionPhasePrompt,
    alien_base_discovered: bool,
) -> Html {
    match prompt {
        ResolutionPhasePrompt::AuditSpending => html! {
            <>
                <p>
                    {"For each deployed "}{inline_icon_text_phrase("interceptor", "Interceptor")}{" and each point of "}{inline_icon_text_phrase("research", "Research Budget")}{", pay §1 from your funds to the supply."}
                </p>
                <p>
                    {"If you cannot afford a payment, instead increase the "}{inline_icon_text_phrase("panic", "Panic Track")}{" one space for each §1 you cannot pay."}
                </p>
            </>
        },
        ResolutionPhasePrompt::ResolveResearch => html! {
            <>
                <p>
                    {"Attempt the "}{inline_icon_text_phrase("research", "Research")}{" task, rolling a number of "}{inline_icon_text_phrase("success", "Success Dice")}{" equal to the "}{inline_icon_text_phrase("research", "Research Budget")}{"."}
                </p>
                <p>
                    {"Remember to increase the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" by one after each attempt."}
                </p>
                <div class="prompt-success-outcome-container">
                    <img class="icon-header" src="assets/icons/success.png" />
                    <p>
                        {"Add a "}{inline_icon_text_phrase("success", "Success Token")}{" to the selected "}{inline_icon_text_phrase("tech", "Technology.")}{" If there are tokens equal to the technology's "}{inline_icon_text_phrase("research", "Research Cost")}{", gain the "}{inline_icon_text_phrase("tech", "Technology")}{"."}
                    </p>
                </div>
                <div class="prompt-threat-outcome-container">
                    <img class="icon-header" src="assets/icons/alien.png" />
                    <p>
                        {"The volatile alien technology explodes. Remove the selected "}{inline_icon_text_phrase("tech", "Technology")}{" from the game."}
                    </p>
                    <p>
                        {"If you also rolled enough "}{inline_icon_text_phrase("success", "Successes")}{" to fully research the "}{inline_icon_text_phrase("tech", "Technology")}{", remove a different "}{inline_icon_text_phrase("tech", "Technology")}{" of your choice instead (you still gain the researched "}{inline_icon_text_phrase("tech", "Technology")}{")."}
                    </p>
                </div>
            </>
        },
        ResolutionPhasePrompt::ResolveUFODefense => html! {
            <>
                <p>
                    {"In any order, complete the "}{inline_icon_text_phrase("interceptor", "UFO Defense Task")}{" for each continent on the world map. Roll a number of "}{inline_icon_text_phrase("success", "Success Dice")}{" equal to the number of "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" assigned to the continent."}
                </p>
                <p>
                    {"Remember to increase the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" by one after each attempt, and to reset the "}{inline_icon_text_phrase("alien", "Alien Threat")}{" when changing to a different continent."}
                </p>
                <div class="prompt-success-outcome-container">
                    <img class="icon-header" src="assets/icons/success.png" />
                    <p>
                        {"Remove one UFO from the continent."}
                    </p>
                    {
                        if alien_base_discovered {
                            html!{
                                <p>
                                    {"Once all UFOs have been removed from the continent containing the "}{inline_icon_text_phrase("alien", "Alien Base,")}{" any additional "}{inline_icon_text_phrase("success", "Successes")}{" rolled in this continent's "}{inline_icon_text_phrase("interceptor", "UFO Defense Task")}{" instead add a "}{inline_icon_text_phrase("success", "Success Token")}{" on the "}{inline_icon_text_phrase("alien", "Alien Base.")}{" Once the third "}{inline_icon_text_phrase("success", "Success Token")}{" has been added to the "}{inline_icon_text_phrase("alien", "Alien Base")}{", it is destroyed!"}
                                </p>
                            }
                        } else {
                            html!{}
                        }
                    }
                </div>
                <div class="prompt-threat-outcome-container">
                    <img class="icon-header" src="assets/icons/alien.png" />
                    <p>
                        {"Your interceptors are shot down by the UFOs. Remove half of the "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" assigned to this task (rounded up) - add them back to the supply (not your reserves)."}
                    </p>
                    <p>
                        {"Note: you will roll fewer "}{inline_icon_text_phrase("success", "Success Dice")}{" in subsequent attempts at this task as the removed "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" are no longer assigned to the task."}
                    </p>
                </div>
            </>
        },
        ResolutionPhasePrompt::IncreasePanic => html! {
            <>
                <p>
                    {"For each continent with any remaining UFOs, increase the "}{inline_icon_text_phrase("panic", "Panic Track")}{" one space."}
                </p>
            </>
        },
        ResolutionPhasePrompt::AskForBoardState => html! {},
        ResolutionPhasePrompt::ResolveContinentBonuses => html! {
            <>
                <p>
                    {"For each continent with no remaining UFOs, gain that continent's bonus."}
                </p>
                <div class="prompt-success-outcome-container">
                    <div class="float-left">
                        <img class="icon-header" src="assets/icons/america.png" />
                        <img class="icon-header" src="assets/icons/america-board-position.png" />
                    </div>
                    <h2 class="continent-bonus-header">{"America"}</h2>
                    <h4 class="continent-bonus-header">{"Air and Space:"}</h4>
                    <p>
                        {"Add one "}{inline_icon_text_phrase("interceptor", "Interceptor")}{" from the supply to your reserves."}
                    </p>
                    <p>
                        {"Increase your number of "}{inline_icon_text_phrase("satellite", "Satellites")}{" by 1 (to a maximum of 3)."}
                    </p>
                </div>
                <div class="prompt-success-outcome-container">
                    <div class="float-left">
                        <img class="icon-header" src="assets/icons/africa.png" />
                        <img class="icon-header" src="assets/icons/africa-board-position.png" />
                    </div>
                    <h2 class="continent-bonus-header">{"Africa"}</h2>
                    <h4 class="continent-bonus-header">{"All In:"}</h4>
                    <p>
                        {"Take §2 from the supply and add it to your funds."}
                    </p>
                </div>
                <div class="prompt-success-outcome-container">
                    <div class="float-left">
                        <img class="icon-header" src="assets/icons/eurasia.png" />
                        <img class="icon-header" src="assets/icons/eurasia-board-position.png" />
                    </div>
                    <h2 class="continent-bonus-header">{"Eurasia"}</h2>
                    <h4 class="continent-bonus-header">{"Expert Knowledge:"}</h4>
                    <p>
                        {"Add one "}{inline_icon_text_phrase("success", "Success Token")}{" to the "}{inline_icon_text_phrase("tech", "Technology")}{" currently selected for research."}
                    </p>
                    <p>
                        {"If there is no "}{inline_icon_text_phrase("tech", "Technology")}{" currently selected, select a "}{inline_icon_text_phrase("tech", "Technology")}{" with a "}{inline_icon_text_phrase("research", "Research Cost")}{" of at least 2 and then add a "}{inline_icon_text_phrase("success", "Success Token")}{" to it."}
                    </p>
                </div>
            </>
        },
        ResolutionPhasePrompt::CleanUp => html! {
            <p>
                {"Remove all UFO dice from the world map. Return all assigned "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" to your reserves."}
            </p>
        },
        ResolutionPhasePrompt::PurchaseReplacementForces => html! {
            <>
                <p>
                    {"You may purchase additional "}{inline_icon_text_phrase("interceptor", "Interceptors")}{" and "}{inline_icon_text_phrase("satellite", "Satellites:")}
                </p>
                <p>
                    {"For §1 each, add an "}{inline_icon_text_phrase("interceptor", "Interceptor")}{" from the supply to your reserves."}
                </p>
                <p>
                    {"For §2 each, increase your number of "}{inline_icon_text_phrase("satellite", "Satellites")}{" by 1 (to a maximum of 3)."}
                </p>
            </>
        },
    }
}
