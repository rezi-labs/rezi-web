use maud::{Markup, html};

use crate::view::icons::{self, house_icon, list_icon, spark_icon, user_icon};

pub fn render() -> Markup {
    html! {
       (navbar())
    }
}

fn navbar() -> Markup {
    html! {
        nav class="bg-base-100 border-b border-base-200 px-6 py-4" {
            div class="max-w-7xl mx-auto flex items-center justify-between" {
                div class="flex items-center" {
                    a href="/" class="flex items-center gap-3 hover:opacity-80 transition-opacity" {
                        span class="w-5 h-5 opacity-70" {
                            (spark_icon())
                        }
                        h1 class="text-lg font-light tracking-widest uppercase text-base-content" {
                            "Rezi"
                        }
                    }
                }

                // Main navigation - Clean links without backgrounds
                div class="hidden lg:flex items-center gap-8" {
                    a href="/" class="flex items-center gap-2 text-sm font-medium text-base-content/70 hover:text-base-content transition-colors py-2" {
                        span class="w-4 h-4 opacity-60" {
                            (spark_icon())
                        }
                        "Chat"
                    }
                    a href="/items" class="flex items-center gap-2 text-sm font-medium text-base-content/70 hover:text-base-content transition-colors py-2" {
                        span class="w-4 h-4 opacity-60" {
                            (list_icon())
                        }
                        "Items"
                    }
                    a href="/recipes" class="flex items-center gap-2 text-sm font-medium text-base-content/70 hover:text-base-content transition-colors py-2" {
                        span class="w-4 h-4 opacity-60" {
                            (house_icon())
                        }
                        "Recipes"
                    }
                    a href="/profile" class="flex items-center gap-2 text-sm font-medium text-base-content/70 hover:text-base-content transition-colors py-2" {
                        span class="w-4 h-4 opacity-60" {
                            (user_icon())
                        }
                        "Profile"
                    }
                }

                div class="flex items-center gap-4" {
                    div class="dropdown dropdown-end" {
                        div tabindex="0" role="button" class="flex items-center gap-2 text-sm font-medium text-base-content/70 hover:text-base-content transition-colors py-2 cursor-pointer" {
                            span class="w-4 h-4 opacity-60" {
                                (icons::export_icon())
                            }
                            "Export"
                            svg class="w-3 h-3 opacity-40" viewBox="0 0 12 12" fill="currentColor" {
                                path d="M6 9L2 5h8L6 9z" {}
                            }
                        }
                        ul tabindex="0" class="dropdown-content menu bg-base-100 z-[1] w-40 p-2 shadow-lg border border-base-200 rounded-lg mt-2" {
                            li {
                                a href="/items/csv" hx-swap="none" class="text-sm text-base-content/70 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    "CSV Export"
                                }
                            }
                        }
                    }

                    div class="dropdown dropdown-end lg:hidden" {
                        div tabindex="0" role="button" class="p-2 text-base-content/70 hover:text-base-content transition-colors" {
                            svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 6h16M4 12h16M4 18h16" {}
                            }
                        }
                        ul tabindex="0" class="dropdown-content menu bg-base-100 z-[1] w-52 p-3 shadow-lg border border-base-200 rounded-lg mt-2" {
                            li {
                                a href="/" class="flex items-center gap-3 text-sm text-base-content/70 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    span class="w-4 h-4 opacity-60" {
                                        (spark_icon())
                                    }
                                    "Chat"
                                }
                            }
                            li {
                                a href="/items" class="flex items-center gap-3 text-sm text-base-content/70 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    span class="w-4 h-4 opacity-60" {
                                        (list_icon())
                                    }
                                    "Items"
                                }
                            }
                            li {
                                a href="/recipes" class="flex items-center gap-3 text-sm text-base-content/70 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    span class="w-4 h-4 opacity-60" {
                                        (house_icon())
                                    }
                                    "Recipes"
                                }
                            }
                            li {
                                a href="/profile" class="flex items-center gap-3 text-sm text-base-content/70 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    span class="w-4 h-4 opacity-60" {
                                        (user_icon())
                                    }
                                    "Profile"
                                }
                            }
                            div class="border-t border-base-200 my-2" {}
                            li {
                                a href="/items/csv" hx-swap="none" class="flex items-center gap-3 text-xs text-base-content/60 hover:text-base-content hover:bg-base-200/50 px-3 py-2 rounded transition-colors" {
                                    span class="w-3 h-3 opacity-60" {
                                        (icons::export_icon())
                                    }
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
