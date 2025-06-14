use maud::{Markup, html};

pub fn render() -> Markup {
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
                        a href="/ui" {
                            "Chat"
                        }
                    }
                    li {
                        a href="/ui/list" {
                            "List"
                        }
                    }
            }
        }
        div class="navbar-end" {
            div class="dropdown dropdown-end w-40" {
                div class="btn btn-ghost" tabindex="0" role="button" {
                    svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" {
                        path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" {
                        }
                    }
                    "Theme"
                }
                ul class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow" tabindex="0" {
                    li { a onclick="changeTheme('light')" { "🌞 Light" } }
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
