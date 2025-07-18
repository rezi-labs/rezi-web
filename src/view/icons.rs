use maud::{Markup, html};
use rand::Rng;

pub fn list_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" {
            }
        }
    }
}

pub fn chat_icon() -> Markup {
    html! {
        svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" {
            }
        }
    }
}

pub fn share_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 1 0 0 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186 9.566-5.314m-9.566 7.5 9.566 5.314m0 0a2.25 2.25 0 1 0 3.935 2.186 2.25 2.25 0 0 0-3.935-2.186Zm0-12.814a2.25 2.25 0 1 0 3.933-2.185 2.25 2.25 0 0 0-3.933 2.185Z" {
            }
        }
    }
}

pub fn add_icon() -> Markup {
    html! {
        svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" {
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

#[allow(unused)]
pub fn apple_icon() -> Markup {
    html! {
        svg aria-label="Apple logo" width="16" height="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1195 1195" {
            path fill="white" d="M1006.933 812.8c-32 153.6-115.2 211.2-147.2 249.6-32 25.6-121.6 25.6-153.6 6.4-38.4-25.6-134.4-25.6-166.4 0-44.8 32-115.2 19.2-128 12.8-256-179.2-352-716.8 12.8-774.4 64-12.8 134.4 32 134.4 32 51.2 25.6 70.4 12.8 115.2-6.4 96-44.8 243.2-44.8 313.6 76.8-147.2 96-153.6 294.4 19.2 403.2zM802.133 64c12.8 70.4-64 224-204.8 230.4-12.8-38.4 32-217.6 204.8-230.4z" {
            }
        }
    }
}

#[allow(unused)]
pub fn google_icon() -> Markup {
    html! {
        svg aria-label="Google logo" width="16" height="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" {
            g {
                path d="m0 0H512V512H0" fill="#fff" {
                }
                path fill="#34a853" d="M153 292c30 82 118 95 171 60h62v48A192 192 0 0190 341" {
                }
                path fill="#4285f4" d="m386 400a140 175 0 0053-179H260v74h102q-7 37-38 57" {
                }
                path fill="#fbbc02" d="m90 341a208 200 0 010-171l63 49q-12 37 0 73" {
                }
                path fill="#ea4335" d="m153 219c22-69 116-109 179-50l55-54c-78-75-230-72-297 55" {
                }
            }
        }
    }
}

#[allow(unused)]
pub fn house_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M8.25 21v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21m0 0h4.5V3.545M12.75 21h7.5V10.75M2.25 21h1.5m18 0h-18M2.25 9l4.5-1.636M18.75 3l-1.5.545m0 6.205 3 1m1.5.5-1.5-.5M6.75 7.364V3h-3v18m3-13.636 10.5-3.819" {
            }
        }
    }
}

pub fn potion_bottle_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M8 2v4l-2 2v12c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V8l-2-2V2M8 2h8M10 10h4M9 13h6M10 16h4" {
            }
        }
    }
}

pub fn cauldron_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M4 8h16l-1 10c0 1.1-.9 2-2 2H7c-1.1 0-2-.9-2-2L4 8zM2 8h20M8 4v4M16 4v4M12 2v2M9 11c0 1.5 1.5 3 3 3s3-1.5 3-3" {
            }
        }
    }
}

pub fn flask_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M9 2v6.5L5 14v5c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-5l-4-5.5V2M9 2h6M8 14h8" {
            }
        }
    }
}

pub fn magic_potion_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M10 2h4v3l3 3v12c0 1.1-.9 2-2 2H9c-1.1 0-2-.9-2-2V8l3-3V2zM8 10h8M9 13h6M10 16h4" {
            }
            circle cx="12" cy="11" r="1" fill="currentColor" {
            }
            circle cx="10" cy="14" r="0.5" fill="currentColor" {
            }
            circle cx="14" cy="17" r="0.5" fill="currentColor" {
            }
        }
    }
}

pub fn brew_bubbles_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M6 8h12l-1 12H7L6 8zM5 8h14M9 4v4M15 4v4" {
            }
            circle cx="10" cy="12" r="1" fill="none" {
            }
            circle cx="14" cy="14" r="1" fill="none" {
            }
            circle cx="12" cy="16" r="1" fill="none" {
            }
            path stroke-linecap="round" stroke-linejoin="round" d="M8 2c0 1 1 2 2 2s2-1 2-2M12 1c0 1 1 2 2 2s2-1 2-2M16 2c0 1 1 2 2 2s2-1 2-2" {
            }
        }
    }
}

pub fn elixir_bottle_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M9 2v2c0 1-1 2-2 2v12c0 2 1 4 3 4h4c2 0 3-2 3-4V6c-1 0-2-1-2-2V2M9 2h6M7 10h10M8 14h8" {
            }
            path stroke-linecap="round" stroke-linejoin="round" d="M10 6v2M14 6v2" {
            }
        }
    }
}

pub fn witch_brew_icon() -> Markup {
    html! {
        svg class="size-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" {
            path stroke-linecap="round" stroke-linejoin="round" d="M3 9h18l-2 11H5L3 9zM2 9h20M7 5v4M17 5v4M12 3v2" {
            }
            path stroke-linecap="round" stroke-linejoin="round" d="M9 12c2-1 4-1 6 0M8 15c3-1 5-1 8 0M10 18c2-1 3-1 4 0" {
            }
            path stroke-linecap="round" stroke-linejoin="round" d="M6 1l1 2M12 0l1 2M18 1l1 2" {
            }
        }
    }
}

#[allow(unused)]
pub fn wand_icon() -> Markup {
    html! {
        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
            line x1="15" y1="15" x2="21" y2="21" {
            }
            path d="M9 6L10 8L12 8L10.5 9.5L11 12L9 11L7 12L7.5 9.5L6 8L8 8L9 6z" fill="currentColor" {
            }
            path d="M16 8L17 9L16 10L15 9L16 8z" fill="currentColor" {
            }
            path d="M18 5L18.5 5.5L18 6L17.5 5.5L18 5z" fill="currentColor" {
            }
            path d="M4 17L4.5 17.5L4 18L3.5 17.5L4 17z" fill="currentColor" {
            }
        }
    }
}

pub fn random_potion_icon() -> Markup {
    let mut rng = rand::rng();
    let icon_index = rng.random_range(0..7);

    match icon_index {
        0 => potion_bottle_icon(),
        1 => cauldron_icon(),
        2 => flask_icon(),
        3 => magic_potion_icon(),
        4 => brew_bubbles_icon(),
        5 => elixir_bottle_icon(),
        _ => witch_brew_icon(),
    }
}
