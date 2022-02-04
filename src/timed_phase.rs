use crate::{common::Focus, tech_reference::TechReference};
use crate::{
    common::{inline_icon_text_phrase, side_buttons},
    rules::rules_reference,
};
use boolinator::Boolinator;
use gloo::{timers::callback::Interval, utils::document};
use gloo_storage::{LocalStorage, Storage};
use web_sys::Element;
use xcom_1_card::TimedPhasePrompt;
use yew::prelude::*;

const LATEST_PROMPT_INDEX_KEY: &str = "TimedPhase_LatestPromptIndex";
const TIME_REMANING_KEY: &str = "TimedPhase_TimeRemaining";

const STARTING_TIME: f64 = 16_000.0;
const TIME_PER_PROMPT: f64 = 8_000.0;

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
    Tick,
    ToggleTech,
    ToggleResearch,
    OnCompleted,
}

pub struct TimedPhase {
    current_prompt_index: usize,
    latest_prompt_index: usize,
    time_remaining_ms: f64,
    last_tick_time: f64,
    tick_interval: Interval,
    focus: Focus,
    prompt_details_ref: NodeRef,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TimedPhaseProps {
    pub prompts: Vec<TimedPhasePrompt>,
    pub round: u32,
    pub on_completed: Callback<()>,
    pub on_alien_base_discovered: Callback<()>,
}

impl Component for TimedPhase {
    type Message = Msg;
    type Properties = TimedPhaseProps;

    fn create(ctx: &Context<Self>) -> Self {
        let tick_interval = {
            let link = ctx.link().clone();
            Interval::new(87, move || link.send_message(Msg::Tick))
        };
        // Load when component is created
        let latest_prompt_index = LocalStorage::get(LATEST_PROMPT_INDEX_KEY).unwrap_or(0);
        // Add an extra second to let the player re-read the prompts etc
        let time_remaining_ms =
            LocalStorage::get(TIME_REMANING_KEY).unwrap_or(STARTING_TIME) + 1_000.0;

        Self {
            current_prompt_index: latest_prompt_index,
            latest_prompt_index,
            time_remaining_ms,
            last_tick_time: js_sys::Date::now(),
            tick_interval,
            focus: Focus::Prompt,
            prompt_details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextPrompt => {
                if (self.current_prompt_index + 1) <= ctx.props().prompts.len() {
                    if matches!(
                        ctx.props().prompts[self.current_prompt_index],
                        TimedPhasePrompt::AlienBaseDiscovered(_)
                    ) {
                        ctx.props().on_alien_base_discovered.emit(());
                    }
                    if self.current_prompt_index + 1 > self.latest_prompt_index {
                        self.latest_prompt_index = self.current_prompt_index + 1;
                        if let Err(_) =
                            LocalStorage::set(LATEST_PROMPT_INDEX_KEY, self.latest_prompt_index)
                        {
                            log::error!("Error saving latest prompt index");
                        }
                        self.time_remaining_ms += TIME_PER_PROMPT;
                    }
                    self.current_prompt_index += 1;

                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::PreviousPrompt => {
                if self.current_prompt_index > 0 {
                    self.current_prompt_index -= 1;
                    if let Some(element) = self.prompt_details_ref.cast::<Element>() {
                        element.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    true
                } else {
                    false
                }
            }
            Msg::Tick => {
                let next_tick_time = js_sys::Date::now();
                let diff = next_tick_time - self.last_tick_time;
                if matches!(self.focus, Focus::Prompt) && document().has_focus().unwrap_or(false) {
                    self.time_remaining_ms = f64::max(self.time_remaining_ms - diff, 0.0);
                    if let Err(_) = LocalStorage::set(TIME_REMANING_KEY, self.time_remaining_ms) {
                        log::error!("Error saving time_remaining");
                    }
                }
                self.last_tick_time = next_tick_time;
                true
            }
            Msg::ToggleTech => {
                self.focus = match self.focus {
                    Focus::Prompt | Focus::RulesReference => Focus::TechReference,
                    Focus::TechReference => Focus::Prompt,
                };
                true
            }
            Msg::ToggleResearch => {
                self.focus = match self.focus {
                    Focus::Prompt | Focus::TechReference => Focus::RulesReference,
                    Focus::RulesReference => Focus::Prompt,
                };
                true
            }
            Msg::OnCompleted => {
                LocalStorage::delete(LATEST_PROMPT_INDEX_KEY);
                LocalStorage::delete(TIME_REMANING_KEY);
                ctx.props().on_completed.emit(());
                false
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (title, next_callback, icons_html, description) = if self.current_prompt_index
            == ctx.props().prompts.len()
        {
            (
                "Completing Timed Phase".to_owned(),
                ctx.link().callback(|_| Msg::OnCompleted),
                html! {
                    <img class="prompt-icon" src="assets/icons/time.png"/>
                },
                html! {
                    <>
                        {"This is a final chance to use "}{inline_icon_text_phrase("time", "Timed Phase")}{" "}{inline_icon_text_phrase("tech", "Technology")}{" or to use "}{inline_icon_text_phrase("satellite", "Satellites")}{" to adjust deployment of your "}{inline_icon_text_phrase("interceptor", "Interceptors.")}
                    </>
                },
            )
        } else {
            (
                ctx.props().prompts[self.current_prompt_index].title(),
                ctx.link().callback(|_| Msg::NextPrompt),
                icon_html_for_prompt(&ctx.props().prompts[self.current_prompt_index]),
                description_html_for_prompt(&ctx.props().prompts[self.current_prompt_index]),
            )
        };
        let time_s = (self.time_remaining_ms / 1000.0).floor();
        let time_ms = ((self.time_remaining_ms % 1000.0) / 10.0).floor();
        let is_not_latest_prompt = self.current_prompt_index < self.latest_prompt_index;
        html! {
            <>
                <h1 class={classes!("prompt-title", is_not_latest_prompt.as_some("faded-text"))}>{ title }</h1>
                <div class="prompt-center-area">
                    {side_buttons(ctx.link().callback(|_| Msg::ToggleTech), ctx.link().callback(|_| Msg::ToggleResearch))}
                    {
                        match self.focus {
                            Focus::Prompt => html!{
                                <div class="prompt-details" ref={self.prompt_details_ref.clone()}>
                                    <div class="prompt-icons">
                                        {icons_html}
                                    </div>
                                    <div class={classes!("prompt-description", is_not_latest_prompt.as_some("faded-text"))}>
                                        {description}
                                    </div>
                                </div>
                            },
                            Focus::TechReference => html!{
                                <div class="tech-ref-container">
                                    <h1 class="prompt-title">{"Technology Reference"}</h1>
                                    <TechReference/>
                                </div>
                            },
                            Focus::RulesReference => html!{
                                <div class="rules-ref-container">
                                    <h1 class="prompt-title">{"Rules Reference"}</h1>
                                    {rules_reference()}
                                </div>
                            },
                        }
                    }
                    <div class="timed-phase-prompt-preview">
                    </div>
                </div>
                <div class="bottom-panel">
                    <button class="button-back" onclick={ctx.link().callback(|_| Msg::PreviousPrompt)} disabled={ !matches!(self.focus, Focus::Prompt) || self.current_prompt_index < 1 }>{ "Back" }</button>
                    <div>
                        <div class="round">{format!("Round {}", ctx.props().round)}</div>
                        <div class={classes!("timer", (time_s < 5.0).as_some("blink-red"))}>{ format!("{:3.0}:{:02.0}", time_s, time_ms) }</div>
                    </div>
                    <button class="button-done" onclick={next_callback} disabled={ !matches!(self.focus, Focus::Prompt) }>{
                        if is_not_latest_prompt {
                            "Next"
                        } else {
                            "Done"
                        }
                    }</button>
                </div>
            </>
        }
    }
}

fn icon_html_for_prompt(prompt: &TimedPhasePrompt) -> Html {
    match prompt {
        TimedPhasePrompt::TakeIncome(_) => html! {
            <img class="prompt-icon" src="assets/icons/income.png"/>
        },
        TimedPhasePrompt::RollUFOLocation(continent) => html! {
            <>
                <img class="prompt-icon" src="assets/icons/roll.png"/>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src={format!("assets/icons/{}-board-position.png", continent.lowercase())}/>
                </div>
            </>
        },
        TimedPhasePrompt::AddUFOsToLocation(continent, n) => html! {
            <>
                <img class="prompt-icon" src={
                    match *n {
                        n if n >= 3 => "assets/icons/increase-3.png",
                        2 => "assets/icons/increase-2.png",
                        _ => "assets/icons/increase-1.png",
                    }
                }/>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src={format!("assets/icons/{}-board-position.png", continent.lowercase())}/>
                </div>
            </>
        },
        TimedPhasePrompt::SwapUFOLocations(from, to) => html! {
            <>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src={format!("assets/icons/{}-board-position.png", from.lowercase())}/>
                </div>
                <img class="prompt-icon" src="assets/icons/swap.png"/>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src={format!("assets/icons/{}-board-position.png", to.lowercase())}/>
                </div>
            </>
        },
        TimedPhasePrompt::ChooseResearch => html! {
            <img class="prompt-icon" src="assets/icons/research.png"/>
        },
        TimedPhasePrompt::SetResearchBudget => html! {
            <img class="prompt-icon" src="assets/icons/research.png"/>
        },
        TimedPhasePrompt::AssignInterceptors(continent) => html! {
            <div class="prompt-icon">
                <img  src="assets/icons/interceptor.png"/>
                <img class="continent-location" src={format!("assets/icons/{}-board-position.png", continent.lowercase())}/>
            </div>
        },
        TimedPhasePrompt::AlienBaseDiscovered(continent) => html! {
            <div class="prompt-icon">
                <img  src="assets/icons/alien-base.png"/>
                <img class="continent-location" src={format!("assets/icons/{}-board-position.png", continent.lowercase())}/>
            </div>
        },
    }
}

fn description_html_for_prompt(prompt: &TimedPhasePrompt) -> Html {
    match prompt {
        TimedPhasePrompt::TakeIncome(n) => html! {
            <>
                {format!("Take §{} from the supply and add it to your funds.", n)}
            </>
        },
        TimedPhasePrompt::RollUFOLocation(location) => html! {
            <>
                {format!("Roll a UFO die and place it on the World Map over {}.", location)}
            </>
        },
        TimedPhasePrompt::AddUFOsToLocation(location, amount) => html! {
            <>
                {format!("Increase the number of UFOs over {} by {}.", location, amount)}
            </>
        },
        TimedPhasePrompt::SwapUFOLocations(from, to) => html! {
            <>
                {format!("Swap the UFO die over {} with the one over {}.", from, to)}
            </>
        },
        TimedPhasePrompt::ChooseResearch => html! {
            <>
                <p>
                    {"Draw two "}{inline_icon_text_phrase("tech", "Technologies")}{" from the "}{inline_icon_text_phrase("tech", "Technology Token stack.")}{" Select one of them to research, or continue researching the currently selected "}{inline_icon_text_phrase("tech", "Technology ")}{" (if any)."}
                </p>
                <p>
                    {"Return any "}{inline_icon_text_phrase("tech", "Technologies")}{" you did not select to the bottom of the "}{inline_icon_text_phrase("tech", "Technology Token stack.")}
                </p>
            </>
        },
        TimedPhasePrompt::SetResearchBudget => html! {
            <>
                <p>
                {"Set the "}{inline_icon_text_phrase("research", "Research Budget")}{" for the round. Each point in the "}{inline_icon_text_phrase("research", "Research Budget")}{" will cost §1."}
                </p>
            </>
        },
        TimedPhasePrompt::AssignInterceptors(location) => html! {
            <>
                <p>
                    {"Assign "}{inline_icon_text_phrase("interceptor", "Interceptors")}{format!(" to {} from your reserves.", location)}
                </p>
                <p>
                    {"Each deployed "}{inline_icon_text_phrase("interceptor", "Interceptor")}{" will cost §1."}
                </p>
            </>
        },
        TimedPhasePrompt::AlienBaseDiscovered(location) => html! {
            <>
                {"Place the "}{inline_icon_text_phrase("alien-base", "Alien Base Token")}{format!(" with the arrow pointing to {}. Any extra successes rolled during ", location)}{inline_icon_text_phrase("interceptor", "UFO Defence")}{" in this region will damage the "}{inline_icon_text_phrase("alien-base", "Alien Base.")}{" Damage the "}{inline_icon_text_phrase("alien-base", "Alien Base")}{" three times to destroy it and win the game!"}
            </>
        },
    }
}
