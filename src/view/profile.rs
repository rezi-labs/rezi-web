use crate::config::Server;
use crate::from_headers::User;
use crate::routes::{self};
use crate::view::icons;
use actix_web::{HttpRequest, Result as AwResult};
use actix_web::{get, web};
use maud::{Markup, html};

#[get("profile")]
pub async fn profile_endpoint(server: web::Data<Server>, req: HttpRequest) -> AwResult<Markup> {
    let user = routes::get_user(req).unwrap();
    let should_poll_reload = server.db_token().is_none();
    Ok(super::index(Some(render(&user)), should_poll_reload))
}

pub fn render(user: &User) -> Markup {
    html! {
      (avatar_card(user))

    }
}

pub fn avatar(initials: &str) -> Markup {
    html! {
        div .avatar .avatar-placeholder {
            div .bg-neutral .text-neutral-content .w-24 .rounded-full {
                span .text-3xl {
                    (initials)
                }
            }
        }

    }
}

fn avatar_card(user: &User) -> Markup {
    html! {
        div class="card w-96 bg-base-100 shadow-sm mx-auto" {
            div class="card-body" {
                figure .mb-4 {
                    (avatar(&user.initials()))
                }
                div class="flex justify-between" {
                    h2 class="text-3xl font-bold" {
                        "Profile"
                    }
                }
                ul class="mt-6 flex flex-col gap-2 text-xl" {
                    li {
                        span {
                            "Initials: " span{(user.initials())}
                        }
                    }
                    li {
                        span {
                            "ID: " span{(user.id())}
                        }
                    }
                    li {
                        span {
                            "Email: " span{(user.email())}
                        }
                    }
                }

                div .join .join-vertical {
                      button name="theme-buttons" onclick="changeTheme('valentine')" class="join-item btn p-2"  { "Valentine"(icons::valentine_icon())  }
                      button name="theme-buttons" onclick="changeTheme('cupcake')"   class="join-item btn p-2"{ "Cupcake"(icons::cupcake_icon())  }
                      button name="theme-buttons" onclick="changeTheme('halloween')" class="join-item btn p-2"  { "Wizard"(icons::wizard_icon()) }
                      button name="theme-buttons" onclick="changeTheme('forest')"    class="join-item btn p-2" { "Forest" (icons::tree_icon()) }
                }

            }
        }
    }
}
