use maud::{Markup, html};

use crate::{
    message::spark_icon,
    unsafe_token_decode::User,
    view::icons::{self, list_icon},
};

pub fn render(user: &User) -> Markup {
    html! {

        div .drawer {
            input id="main-drawer" type="checkbox" class="drawer-toggle"{}
            div .drawer-content {
                label for="main-drawer" class="btn btn-primary drawer-button" {
                    (icons::house_icon())
                }
            }
            div class="drawer-side" {
                label for="main-drawer" aria-label="close sidebar" class="drawer-overlay" {}
                (navbar(user))
            }

        }

    }
}

fn navbar(user: &User) -> Markup {
    html! {
        div class="bg-base-200 min-h-full w-80 p-4" {
            div class="mb-6" {
                h1 class="text-xl font-bold mb-4" {
                    "Grocy"
                }
            }

            // Main navigation menu
            ul class="menu menu-vertical px-1 space-y-2" {
                li {
                    a href="/" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (spark_icon())
                        span { "Home" }
                    }
                }
                li {
                    a href="/items" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (list_icon())
                        span { "Items" }
                    }
                }
            }

            div class="divider my-4" {}

            // Secondary menu items
            ul class="menu menu-vertical px-1 space-y-2" {
                li {
                    details {
                        summary class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                            (icons::share_icon())
                            span { "Export" }
                        }
                        ul class="ml-6 mt-2 space-y-1" {
                            li {
                                a href="/items/csv" hx-swap="none" class="p-2 rounded hover:bg-base-300" {
                                    "CSV Export"
                                }
                            }
                        }
                    }
                }

                li {
                    details {
                        summary class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                            (icons::user_icon())
                            span { "Profile" }
                        }
                        ul class="ml-6 mt-2 space-y-1" {
                            li { div class="p-2 text-sm text-base-content/70" { "Email: " span{(user.email())} } }
                            li { div class="p-2 text-sm text-base-content/70" { "ID: " span{(user.id())} } }
                            li { div class="p-2 text-sm text-base-content/70" { "Initials: " span{(user.initials())} } }
                        }
                    }
                }

                li {
                    details {
                        summary class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                            (theme_icon())
                            span { "Theme" }
                        }
                        ul class="ml-6 mt-2 space-y-1" {
                            li { a onclick="changeTheme('light')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌞" span { "Light" } } }
                            li { a onclick="changeTheme('dark')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌙" span { "Dark" } } }
                            li { a onclick="changeTheme('synthwave')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌆" span { "Synth" } } }
                            li { a onclick="changeTheme('retro')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🕹️" span { "Retro" } } }
                            li { a onclick="changeTheme('cyberpunk')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🤖" span { "Cyberpunk" } } }
                            li { a onclick="changeTheme('valentine')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "💝" span { "Valentine" } } }
                            li { a onclick="changeTheme('halloween')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🎃" span { "Halloween" } } }
                            li { a onclick="changeTheme('garden')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌻" span { "Garden" } } }
                            li { a onclick="changeTheme('forest')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌲" span { "Forest" } } }
                            li { a onclick="changeTheme('aqua')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌊" span { "Aqua" } } }
                            li { a onclick="changeTheme('luxury')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "💎" span { "Luxury" } } }
                            li { a onclick="changeTheme('dracula')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🧛" span { "Dracula" } } }
                            li { a onclick="changeTheme('corporate')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🏢" span { "Corporate" } } }
                            li { a onclick="changeTheme('business')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "💼" span { "Business" } } }
                            li { a onclick="changeTheme('night')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌃" span { "Night" } } }
                            li { a onclick="changeTheme('coffee')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "☕" span { "Coffee" } } }
                            li { a onclick="changeTheme('winter')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "❄️" span { "Winter" } } }
                            li { a onclick="changeTheme('dim')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🔅" span { "Dim" } } }
                            li { a onclick="changeTheme('nord')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🏔️" span { "Nord" } } }
                            li { a onclick="changeTheme('sunset')" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🌅" span { "Sunset" } } }
                            div class="divider my-1" {}
                            li { a onclick="applyRandomTheme()" class="p-2 rounded hover:bg-base-300 flex items-center gap-2" { "🎲" span { "Random Theme" } } }
                        }
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
