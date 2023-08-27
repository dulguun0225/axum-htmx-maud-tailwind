use maud::{html, Markup, DOCTYPE};

use crate::db::todo::Todo;

fn header() -> Markup {
    html! {
        (DOCTYPE)
        title { "maud" }
        script src="https://unpkg.com/htmx.org@1.9.5" {}
        // script src="https://cdn.tailwindcss.com" {}
    }
}

pub fn todo_list_item(todo: &Todo) -> Markup {
    html! {
        tr {
            td {
                @if todo.done {
                    "☑"
                } @else {
                    "☐"
                }
            }
            td {
                (todo.title)
            }
            td {
                button type="button" { "toggle" }
            }
        }
    }
}

pub fn index(todos: &[Todo]) -> Markup {
    html! {
        (header())

        div class="grid grid-cols-1 justify-items-center" {

            div class="border max-w-7xl" {

                form hx-post="/insert" hx-swap="beforeend" hx-target="#list tbody"{
                    input name="title" type="text" ;
                    button type="submit" { "Submit" }
                }

                table #list {
                    thead {
                        tr {
                            th { "done"}
                            th { "title"}
                            th { "actions" }
                        }
                    }
                    tbody {
                        @for todo in todos {
                            (todo_list_item(todo))
                        }
                    }
                }
            }
        }
    }
}
