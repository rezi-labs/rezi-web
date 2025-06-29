use maud::{Markup, html};

use crate::user::User;

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
                    (user_icon())
                }
                ul class="dropdown-content menu-sm menu bg-base-100 w-44 rounded-box z-[1] p-2 shadow" tabindex="0" {
                    li { "email: "(user.email())}
                    li { "id: "(user.id())}
                    li { "initials: "(user.initials())}
                }
            }
            div class="dropdown dropdown-end" {
                div class="btn btn-ghost" tabindex="0" role="button" {
                  (theme_icon())
                }
                ul class="dropdown-content menu-sm menu bg-base-100 w-52 rounded-box z-1 mt-3 p-2 shadow" tabindex="0" {
                    li { a onclick="changeTheme('light')" { "🌞" span { "Light" } } }
                    li { a onclick="changeTheme('dark')" { "🌙 Dark" } }
                    li { a onclick="changeTheme('synthwave')" { "🌆 Synth" } }
                    li { a onclick="changeTheme('retro')" { "🕹️ Retro" } }
                    li { a onclick="changeTheme('cyberpunk')" { "🤖 Cyberpunk" } }
                    li { a onclick="changeTheme('valentine')" { "💝 Valentine" } }
                    li { a onclick="changeTheme('halloween')" { "🎃 Halloween" } }
                    li { a onclick="changeTheme('garden')" { "🌻 Garden" } }
                    li { a onclick="changeTheme('forest')" { "🌲 Forest" } }
                    li { a onclick="changeTheme('aqua')" { "🌊 Aqua" } }
                    li { a onclick="changeTheme('luxury')" { "💎 Luxury" } }
                    li { a onclick="changeTheme('dracula')" { "🧛 Dracula" } }
                    li { a onclick="changeTheme('corporate')" { "🏢 Corporate" } }
                    li { a onclick="changeTheme('business')" { "💼 Business" } }
                    li { a onclick="changeTheme('night')" { "🌃 Night" } }
                    li { a onclick="changeTheme('coffee')" { "☕ Coffee" } }
                    li { a onclick="changeTheme('winter')" { "❄️ Winter" } }
                    li { a onclick="changeTheme('dim')" { "🔅 Dim" } }
                    li { a onclick="changeTheme('nord')" { "🏔️ Nord" } }
                    li { a onclick="changeTheme('sunset')" { "🌅 Sunset" } }
                    div class="divider my-1" {}
                    li { a onclick="applyRandomTheme()" { "🎲 Random Theme" } }
                }
            }
        }
    }
    }
}

pub fn user_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" {
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
