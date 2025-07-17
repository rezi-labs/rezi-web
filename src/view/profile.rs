use crate::routes::{self};
use crate::unsafe_token_decode::User;
use actix_web::get;
use actix_web::{HttpRequest, Result as AwResult};
use maud::{Markup, html};

#[get("profile")]
pub async fn profile_endpoint(req: HttpRequest) -> AwResult<Markup> {
    let user = routes::get_user(req).unwrap();

    Ok(super::index(Some(render(&user))))
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
                      input name="theme-buttons" onclick="changeTheme('valentine')" type="radio" class="join-item btn p-2" aria-label="💝 Valentine" {   }
                      input name="theme-buttons" onclick="changeTheme('cupcake')"   type="radio" class="join-item btn p-2" aria-label="🧁 Cupcake" {   }
                      input name="theme-buttons" onclick="changeTheme('halloween')" type="radio" class="join-item btn p-2"         aria-label="🎃 Halloween" { }
                      input name="theme-buttons" onclick="changeTheme('forest')"    type="radio" class="join-item btn p-2"         aria-label="🌲 Forect " {  }
                }

            }
        }
    }
}
