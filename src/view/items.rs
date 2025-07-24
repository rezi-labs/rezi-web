use crate::database::items::Item;
use crate::database::{self, DBClient};
use crate::routes::{self};
use crate::view::icons;
use actix_web::error::ParseError;
use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

#[get("items")]
pub async fn index_route(client: web::Data<DBClient>, req: HttpRequest) -> AwResult<Markup> {
    let client = client.get_ref();
    let user = routes::get_user(req).unwrap();

    log::info!("getting items endpoint");

    let Ok(items) = database::items::get_items(client, user.id().to_string()).await else {
        return Err(ParseError::Incomplete.into());
    };

    Ok(super::index(Some(render(&items))))
}

pub fn render(items: &[Item]) -> Markup {
    html! {
        div .p-2 {
            div class="card bg-base-200 shadow-xl" {
                div class="card-body" {

                    h2 class="card-title text-2xl mb-4" {
                        svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" {
                            }
                        }
                        "Items"
                    }
                    form class="flex gap-2 mb-4" hx-post="/items/single" hx-target="#todo-list" hx-swap="beforeend" hx-on--after-request="this.reset()" {
                        input class="input input-bordered flex-1" type="text" name="task" placeholder="Add a new task..." required;
                        button class="btn btn-primary" type="submit" {
                            (icons::add_icon())
                            "Add"
                        }
                    }

                    div id="todo-list" class="todo-container space-y-2 h-[700px] overflow-y-auto" {
                        @for item in items {
                            (render_item(item))
                        }
                    }
                }
            }
        }

    }
}

pub fn render_item(item: &Item) -> Markup {
    render_item_display(item)
}

pub fn render_item_display(item: &Item) -> Markup {
    log::info!("Item: {}", item.id());
    html! {
        div class="flex items-center gap-3 p-3 bg-base-100 rounded-lg" id=(format!("c-todo-{}", item.id())) {

            input class="checkbox checkbox-primary" type="checkbox"
                id=(format!("todo-{}", item.id()))
                name=(format!("todo-{}", item.id()))
                checked[item.completed()]

                hx-patch=(format!("/items/{}/toggle", item.id()))
                hx-target=(format!("#c-todo-{}", item.id()))
                hx-swap="outerHTML";
            span class={
                @if item.completed() {
                    "flex-1 line-through opacity-60 cursor-pointer"
                } @else {
                    "flex-1 cursor-pointer"
                }
            }
            hx-get=(format!("/items/{}/edit", item.id()))
            hx-target=(format!("#c-todo-{}", item.id()))
            hx-swap="outerHTML"
            title="Click to edit" {
                (item.task)
            }
            button class="btn btn-sm btn-error btn-outline"
                hx-delete=(format!("/items/{}", item.id()))
                hx-target="closest div"
                hx-swap="outerHTML"
                hx-confirm="Are you sure you want to delete this item?" {
                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {
                    }
                }
            }
        }
    }
}

pub fn render_item_edit(item: &Item) -> Markup {
    html! {
        div class="flex items-center gap-3 p-3 bg-base-100 rounded-lg" id=(format!("c-todo-{}", item.id())) {

            input class="checkbox checkbox-primary" type="checkbox"
                id=(format!("todo-{}", item.id()))
                name=(format!("todo-{}", item.id()))
                checked[item.completed()]

                hx-patch=(format!("/items/{}/toggle", item.id()))
                hx-target=(format!("#c-todo-{}", item.id()))
                hx-swap="outerHTML";

            form class="flex-1 flex gap-2"
                hx-patch=(format!("/items/{}", item.id()))
                hx-target=(format!("#c-todo-{}", item.id()))
                hx-swap="outerHTML" {
                input class="input input-bordered flex-1"
                    type="text"
                    name="task"
                    value=(item.task)
                    required
                    autofocus;
                button class="btn btn-sm btn-primary" type="submit" {
                    svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" {
                        }
                    }
                }
                button class="btn btn-sm btn-ghost" type="button"
                    hx-get=(format!("/items/{}/cancel", item.id()))
                    hx-target=(format!("#c-todo-{}", item.id()))
                    hx-swap="outerHTML" {
                    svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" {
                        }
                    }
                }
            }

            button class="btn btn-sm btn-error btn-outline"
                hx-delete=(format!("/items/{}", item.id()))
                hx-target="closest div"
                hx-swap="outerHTML"
                hx-confirm="Are you sure you want to delete this item?" {
                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {
                    }
                }
            }
        }
    }
}
