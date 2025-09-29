use lazy_static::lazy_static;
use markdown::{self, CompileOptions, Options};
use maud::{Markup, PreEscaped, html};

use crate::text_utils;

lazy_static! {
    static ref README_HTML: String = {
        let markdown_content = include_str!("../../README.md");
        let html_output = markdown::to_html_with_options(
            markdown_content,
            &Options {
                compile: CompileOptions {
                    ..CompileOptions::default()
                },
                ..Options::default()
            },
        )
        .unwrap();

        format!("<div class=\"space-y-6\">{}</div>", html_output)
    };
}

lazy_static! {
    static ref CHANGE_LOG: String = {
        let markdown_content = include_str!("../../CHANGELOG.md");

        let no_links = text_utils::remove_links(markdown_content);

        let html_output = markdown::to_html_with_options(
            &no_links,
            &Options {
                compile: CompileOptions {
                    ..CompileOptions::default()
                },
                ..Options::default()
            },
        )
        .unwrap();

        format!("<div class=\"space-y-6\">{}</div>", html_output)
    };
}

pub fn readme() -> Markup {
    html! {
         (PreEscaped(&*README_HTML))
    }
}

pub fn changelog() -> Markup {
    html! {
         (PreEscaped(&*CHANGE_LOG))
    }
}

pub fn about() -> Markup {
    html! {
        div class="space-x-4 mt-2" {
            button btn hx-get="/about/readme" hx-trigger="click, load" hx-target="#about-content" {
                "readme"
            }
            button btn hx-get="/about/changelog" hx-trigger="click" hx-target="#about-content"{
                "changelog"
            }
        }

        div class="max-w-6xl mx-auto h-full" {
            div class="rounded-lg h-full max-h-[calc(100vh-200px)] min-h-[400px] overflow-y-auto p-6" {
                div class="prose prose-lg max-w-none" {
                        div id="about-content"{}
                }
            }
        }
    }
}
