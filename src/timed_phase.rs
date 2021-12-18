use boolinator::Boolinator;
use gloo::timers::callback::Interval;
use xcom_1_card::TimedPhasePrompt;
use yew::{prelude::*, web_sys::Element};

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
    Tick,
}

pub struct TimedPhase {
    link: ComponentLink<Self>,
    current_prompt_index: usize,
    latest_prompt_index: usize,
    time_remaining_ms: f64,
    last_tick_time: f64,
    tick_interval: Interval,
    props: TimedPhaseProps,
    prompt_details_ref: NodeRef,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TimedPhaseProps {
    pub prompts: Vec<TimedPhasePrompt>,
    pub on_completed: Callback<MouseEvent>,
    pub on_alien_base_discovered: Callback<()>,
}

impl Component for TimedPhase {
    type Message = Msg;
    type Properties = TimedPhaseProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tick_interval = {
            let link = link.clone();
            Interval::new(87, move || link.send_message(Msg::Tick))
        };
        Self {
            link,
            current_prompt_index: 0,
            latest_prompt_index: 0,
            time_remaining_ms: 15_000.0,
            last_tick_time: js_sys::Date::now(),
            tick_interval,
            props,
            prompt_details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextPrompt => {
                if (self.current_prompt_index + 1) <= self.props.prompts.len() {
                    if matches!(
                        self.props.prompts[self.current_prompt_index],
                        TimedPhasePrompt::AlienBaseDiscovered(_)
                    ) {
                        self.props.on_alien_base_discovered.emit(());
                    }
                    if self.current_prompt_index + 1 > self.latest_prompt_index {
                        self.latest_prompt_index = self.current_prompt_index + 1;
                        self.time_remaining_ms += 5000.0;
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
                self.time_remaining_ms = f64::max(self.time_remaining_ms - diff, 0.0);
                self.last_tick_time = next_tick_time;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let (title, next_callback, icons_html) =
            if self.current_prompt_index == self.props.prompts.len() {
                (
                    "Completing Timed Phase".to_owned(),
                    self.props.on_completed.clone(),
                    html! {
                        <img class="prompt-icon" src="assets/icons/time.png"/>
                    },
                )
            } else {
                (
                    self.props.prompts[self.current_prompt_index].title(),
                    self.link.callback(|_| Msg::NextPrompt),
                    icon_html_for_prompt(&self.props.prompts[self.current_prompt_index]),
                )
            };
        let time_s = (self.time_remaining_ms / 1000.0).floor();
        let time_ms = ((self.time_remaining_ms % 1000.0) / 10.0).floor();
        html! {
            <>
                <h1 class="prompt-title">{ title }</h1>
                <div class="prompt-center-area">
                    <div class="side-buttons">
                    </div>
                    <div class="prompt-details" ref=self.prompt_details_ref.clone()>
                        <div class="prompt-icons">
                            {icons_html}
                        </div>
                        <div class="prompt-description">
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                        </div>
                    </div>
                    <div class="timed-phase-prompt-preview">
                    </div>
                </div>
                <div class="bottom-panel">
                    <button class="button-back" onclick=self.link.callback(|_| Msg::PreviousPrompt) disabled={ self.current_prompt_index < 1 }>{ "Back" }</button>
                    <span class=classes!("timer", (time_s < 5.0).as_some("blink-red"))>{ format!("{:3.0}:{:02.0}", time_s, time_ms) }</span>
                    <button class="button-done" onclick=next_callback>{ "Done" }</button>
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
                    <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(continent))/>
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
                    <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(continent))/>
                </div>
            </>
        },
        TimedPhasePrompt::SwapUFOLocations(from, to) => html! {
            <>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(from))/>
                </div>
                <img class="prompt-icon" src="assets/icons/swap.png"/>
                <div class="prompt-icon">
                    <img class="alien-dice-back" src="assets/icons/ufo.png"/>
                    <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(to))/>
                </div>
            </>
        },
        TimedPhasePrompt::ChooseResearch => html! {
            <img class="prompt-icon" src="assets/icons/research.png"/>
        },
        TimedPhasePrompt::AssignInterceptors(continent) => html! {
            <div class="prompt-icon">
                <img  src="assets/icons/interceptor.png"/>
                <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(continent))/>
            </div>
        },
        TimedPhasePrompt::AlienBaseDiscovered(continent) => html! {
            <div class="prompt-icon">
                <img  src="assets/icons/alien-base.png"/>
                <img class="continent-location" src=format!("assets/icons/{}-board-position.png", Into::<String>::into(continent))/>
            </div>
        },
    }
}
