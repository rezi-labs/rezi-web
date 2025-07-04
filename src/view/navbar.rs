use maud::{Markup, html};

use crate::{unsafe_token_decode::User, view::icons};

pub fn render(user: &User) -> Markup {
    html! {
        div class="navbar bg-base-200 shadow-lg" {
        div class="navbar-start" {
            h1 class="text-xl font-bold" {
                "Grocy"
            }
        }
        div class="navbar-center"{
                ul class="menu menu-horizontal px-1" {
                    li {
                        a href="/" {
                            "Chat"
                        }
                    }
                    li {
                        a href="/items" {
                            "Groceries"
                        }
                    }
            }
        }
        div class="navbar-end" {
            div class="dropdown dropdown-end" {
                div class="btn btn-ghost" tabindex="0" role="button" {
                    (icons::share_icon())
                }
                ul class="dropdown-content menu-sm menu bg-base-100 w-52 rounded-box z-1 mt-3 p-2 shadow" tabindex="0" {
                    li { a class="btn btn-ghost bnt-xl" href="/items/ical" hx-swap="none" {
                        (icons::apple_icon())
                         span{
                             "IOS"
                         }
                    }}
                    li { a class="btn btn-ghost bnt-xl" href="/items/csv" hx-swap="none" {
                        (icons::google_icon())
                         span{
                             "Google"
                         }
                    }}
                }
            }

            div class="dropdown dropdown-end" {
                div class="btn btn-ghost" tabindex="0" role="button" {
                    (icons::user_icon())
                }
                ul class="dropdown-content menu-sm menu bg-base-100 w-52 rounded-box z-1 mt-3 p-2 shadow" tabindex="0" {
                    li { a {"email: " span{(user.email())}}}
                    li { a {"id: " span{(user.id())}}}
                    li { a {"initials: " span{(user.initials())}}}
                }
            }
            div class="dropdown dropdown-end" {
                div class="btn btn-ghost" tabindex="0" role="button" {
                  (theme_icon())
                }
                ul class="dropdown-content menu-sm menu bg-base-100 w-52 rounded-box z-1 mt-3 p-2 shadow" tabindex="0" {
                    li { a onclick="changeTheme('light')" { "🌞" span { "Light" } } }
                    li { a onclick="changeTheme('dark')" { "🌙" span { "Dark" } } }
                    li { a onclick="changeTheme('synthwave')" { "🌆" span { "Synth" } } }
                    li { a onclick="changeTheme('retro')" { "🕹️" span { "Retro" } } }
                    li { a onclick="changeTheme('cyberpunk')" { "🤖" span { "Cyberpunk" } } }
                    li { a onclick="changeTheme('valentine')" { "💝" span { "Valentine" } } }
                    li { a onclick="changeTheme('halloween')" { "🎃" span { "Halloween" } } }
                    li { a onclick="changeTheme('garden')" { "🌻" span { "Garden" } } }
                    li { a onclick="changeTheme('forest')" { "🌲" span { "Forest" } } }
                    li { a onclick="changeTheme('aqua')" { "🌊" span { "Aqua" } } }
                    li { a onclick="changeTheme('luxury')" { "💎" span { "Luxury" } } }
                    li { a onclick="changeTheme('dracula')" { "🧛" span { "Dracula" } } }
                    li { a onclick="changeTheme('corporate')" { "🏢" span { "Corporate" } } }
                    li { a onclick="changeTheme('business')" { "💼" span { "Business" } } }
                    li { a onclick="changeTheme('night')" { "🌃" span { "Night" } } }
                    li { a onclick="changeTheme('coffee')" { "☕" span { "Coffee" } } }
                    li { a onclick="changeTheme('winter')" { "❄️" span { "Winter" } } }
                    li { a onclick="changeTheme('dim')" { "🔅" span { "Dim" } } }
                    li { a onclick="changeTheme('nord')" { "🏔️" span { "Nord" } } }
                    li { a onclick="changeTheme('sunset')" { "🌅" span { "Sunset" } } }
                    div class="divider my-1" {}
                    li { a onclick="applyRandomTheme()" { "🎲" span { "Random Theme" } } }
                }
            }
        }
    }
    }
}

pub fn theme_icon() -> Markup {
    html! {
        svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" {
            path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" {
            }
        }
    }
}
