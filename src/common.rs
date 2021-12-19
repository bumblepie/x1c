use yew::{html, Html};

pub fn inline_icon_text_phrase(icon: &str, title: &str) -> Html {
    html! {
        <span class="icon-text-phrase"><img class="inline-icon" src=format!("assets/icons/{}.png", icon)/>{title}</span>
    }
}
