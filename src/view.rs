use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
    html! {
        (DOCTYPE)
        title { "maud" }
        script src="https://unpkg.com/htmx.org@1.9.5" {}
        script src="https://cdn.tailwindcss.com" {}
    }
}

pub fn index() -> Markup {
    html! {
        (header())

        div class="grid grid-cols-1 justify-items-center" {

            div class="border max-w-7xl" {
                h1 { "Hello World" }

                button class="border-2 border-black p-1" hx-post="/" hx-swap="outerHTML" {
                    "click"
                }
            }
        }
    }
}
