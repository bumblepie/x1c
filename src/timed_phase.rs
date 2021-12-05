use xcom_1_card::TimedPhasePrompt;
use yew::prelude::*;

pub enum Msg {
    NextPrompt,
    PreviousPrompt,
}

pub struct TimedPhase {
    link: ComponentLink<Self>,
    current_prompt_index: usize,
    props: TimedPhaseProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TimedPhaseProps {
    pub prompts: Vec<TimedPhasePrompt>,
    pub completed_callback: Callback<()>,
}

impl Component for TimedPhase {
    type Message = Msg;
    type Properties = TimedPhaseProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            current_prompt_index: 0,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextPrompt => {
                if (self.current_prompt_index + 1) >= self.props.prompts.len() {
                    self.props.completed_callback.emit(());
                }
                if (self.current_prompt_index + 1) < self.props.prompts.len() {
                    self.current_prompt_index += 1;
                    true
                } else {
                    false
                }
            }
            Msg::PreviousPrompt => {
                if self.current_prompt_index > 0 {
                    self.current_prompt_index -= 1;
                    true
                } else {
                    false
                }
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
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::PreviousPrompt)>{ "Prev" }</button>
                <button onclick=self.link.callback(|_| Msg::NextPrompt)>{ "Next" }</button>
                <p>{ format!("{:?}", self.props.prompts[self.current_prompt_index]) }</p>
            </div>
        }
    }
}
