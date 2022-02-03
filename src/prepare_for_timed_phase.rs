use gloo::{timers::callback::Interval, utils::document};
use yew::{html, Callback, Component, Context, Properties};

pub enum Msg {
    Tick,
    BeginCountdown,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub countdown_time: f64,
    pub on_countdown_completed: Callback<()>,
}

pub struct PrepareForTimedPhase {
    time_remaining_ms: f64,
    last_tick_time: f64,
    tick_interval: Option<Interval>,
}

impl Component for PrepareForTimedPhase {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            time_remaining_ms: ctx.props().countdown_time,
            last_tick_time: js_sys::Date::now(),
            tick_interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                let next_tick_time = js_sys::Date::now();
                let diff = next_tick_time - self.last_tick_time;
                if document().has_focus().unwrap_or(false) {
                    self.time_remaining_ms = f64::max(self.time_remaining_ms - diff, 0.0);
                }
                if self.time_remaining_ms == 0.0 {
                    ctx.props().on_countdown_completed.emit(());
                }
                self.last_tick_time = next_tick_time;
                true
            }
            Msg::BeginCountdown => {
                let link = ctx.link().clone();
                self.last_tick_time = js_sys::Date::now();
                self.tick_interval = Some(Interval::new(87, move || link.send_message(Msg::Tick)));
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        if self.tick_interval.is_some() {
            let time_s = (self.time_remaining_ms / 1000.0).floor();
            let time_ms = ((self.time_remaining_ms % 1000.0) / 10.0).floor();
            html! {
                <>
                    <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-with-sunset.png)">
                        <div class="prepare-screen-text">{ "Prepare for Timed Phase" }</div>
                        <div>
                            <div class="round">{"Entering Timed Phase"}</div>
                            <div class="round timer">{ format!("{:3.0}:{:02.0}", time_s, time_ms) }</div>
                        </div>
                    </div>
                </>
            }
        } else {
            html! {
                <div class="background-image prepare-screen" style="background-image: url(assets/background-art/ufos-with-sunset.png)">
                    <div class="prepare-screen-text">{ "Prepare for Timed Phase" }</div>
                        <div class="prepare-screen-button-container">
                        <button class="prepare-screen-button button-shadow" onclick={ctx.link().callback(|_| Msg::BeginCountdown)}> {"Enter Timed Phase"}</button>
                    </div>
                </div>
            }
        }
    }
}
