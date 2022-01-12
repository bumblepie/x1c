use yew::prelude::*;

pub fn inline_icon_text_phrase(icon: &str, title: &str) -> Html {
    // Ensure icon is attached to the first word in the phrase
    let mut words = title.split_whitespace();
    let first_word = words.next().unwrap();
    let remaining_words = words.collect::<Vec<_>>().join(" ");
    html! {
        <span class="icon-text-phrase">
            <span style="display: inline-block;">
            <img class="inline-icon" src={format!("assets/icons/{}.png", icon)}/>
            {format!(" {}", first_word)}
            </span>
            {
                if !remaining_words.is_empty() {
                    format!(" {}", remaining_words)
                } else {
                    "".to_owned()
                }
            }
        </span>
    }
}

pub fn side_buttons(
    tech_callback: Callback<MouseEvent>,
    rules_callback: Callback<MouseEvent>,
) -> Html {
    html! {
        <div class="side-buttons">
            <button class="side-button-tech" onclick={tech_callback}>
                <img src="assets/icons/tech.png" />
            </button>
            <button class="side-button-rules" onclick={rules_callback}>
                <img src="assets/icons/help.png" />
            </button>
        </div>
    }
}

pub enum Focus {
    Prompt,
    TechReference,
    RulesReference,
}
