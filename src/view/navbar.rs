use maud::{Markup, html};

use crate::{
    message::spark_icon,
    view::icons::{self, house_icon, info_icon, list_icon, user_icon},
};

pub fn render() -> Markup {
    html! {
       (navbar())
    }
}

fn navbar() -> Markup {
    html! {
        div class="bg-base-100 w-full" {
            div .mb-4{}

            // Main navigation menu
            ul class="menu menu-vertical px-1 space-y-2" {
                li {
                    a href="/" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (spark_icon())
                        span { "Grocy" }
                    }
                }
                li {
                    a href="/items" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (list_icon())
                        span { "Items" }
                    }
                }
                li {
                    a href="/recipes" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (house_icon())
                        span { "Recipes" }
                    }
                }
                li {
                    a href="/profile" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (user_icon())
                        span { "Profile" }
                    }
                }
                li {
                    a href="/info" class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                        (info_icon())
                        span { "Info" }
                    }
                }
            }

            div class="divider my-4" {}

            // Secondary menu items
            ul class="menu menu-vertical px-1 space-y-2" {
                li {
                    details {
                        summary class="flex items-center gap-3 p-3 rounded-lg hover:bg-base-300" {
                            (icons::export_icon())
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




            }
        }
    }
}

#[allow(unused)]
pub fn cog_icon() -> Markup {
    html! {
        svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" {
            path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" {
            }
        }
    }
}
