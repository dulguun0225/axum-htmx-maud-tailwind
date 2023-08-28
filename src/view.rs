use maud::{html, Markup, DOCTYPE};

use crate::db::todo::Todo;

fn header() -> Markup {
    html! {
        (DOCTYPE)
        title { "TODO" }
        script src="https://unpkg.com/htmx.org@1.9.5" {}
        // script src="https://cdn.tailwindcss.com" {}
    }
}

pub static DONE: &str = "☑";
pub static NOT_DONE: &str = "☐";

pub fn todo_done_indicator(done: bool) -> &'static str {
    if done {
        DONE
    } else {
        NOT_DONE
    }
}

pub fn todo_list_item(todo: &Todo) -> Markup {
    let done_id = format!("indicator-{}", todo.id);
    let item_id = format!("todo-{}", todo.id);

    html! {
        tr #(item_id){
            td #(done_id) hx-patch={"/"(todo.id)} hx-swap="innerHTML" style="cursor: default; user-select: none; font-size: 2rem" {
                (todo_done_indicator(todo.done))
            }
            td  style="cursor: default; user-select: none;"  {
                (todo.title)
            }
            td {
                button type="button" hx-delete={"/"(todo.id)} hx-swap="delete" hx-target={"#"(item_id)} { "Delete" }
            }
        }
    }
}

pub fn index(todos: &[Todo]) -> Markup {
    html! {
        (header())

        div class="grid grid-cols-1 justify-items-center" {

            div class="border max-w-7xl" {

                form hx-post="/" hx-swap="beforeend" hx-target="#list tbody"{
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
