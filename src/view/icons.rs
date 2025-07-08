use maud::{Markup, html};

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

pub fn apple_icon() -> Markup {
    html! {
        svg aria-label="Apple logo" width="16" height="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1195 1195" {
            path fill="white" d="M1006.933 812.8c-32 153.6-115.2 211.2-147.2 249.6-32 25.6-121.6 25.6-153.6 6.4-38.4-25.6-134.4-25.6-166.4 0-44.8 32-115.2 19.2-128 12.8-256-179.2-352-716.8 12.8-774.4 64-12.8 134.4 32 134.4 32 51.2 25.6 70.4 12.8 115.2-6.4 96-44.8 243.2-44.8 313.6 76.8-147.2 96-153.6 294.4 19.2 403.2zM802.133 64c12.8 70.4-64 224-204.8 230.4-12.8-38.4 32-217.6 204.8-230.4z" {
            }
        }
    }
}

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
