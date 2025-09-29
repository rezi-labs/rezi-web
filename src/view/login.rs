use maud::{Markup, html};

pub fn login_page() -> Markup {
    html! {
        div class="flex min-h-screen items-center justify-center bg-base-200" {
            div class="w-full max-w-md p-8 space-y-6 bg-base-100 rounded-xl shadow-xl" {
                div class="text-center" {
                    h1 class="text-3xl font-bold text-base-content" { "Welcome to Rezi" }
                    p class="mt-2 text-sm text-base-content/70" {
                        "Please sign in to continue"
                    }
                }

                div class="mt-8" {
                    a href="/auth/login" class="btn btn-primary w-full" hx-boost="false" {
                        "Sign in with OpenID Connect"
                    }
                }

                div class="text-center mt-4" {
                    p class="text-xs text-base-content/50" {
                        "Secure authentication powered by OpenID Connect"
                    }
                }
            }
        }
    }
}
