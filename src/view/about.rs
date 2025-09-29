use lazy_static::lazy_static;
use markdown::{self, CompileOptions, Options};
use maud::{Markup, PreEscaped, html};

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

        format!(
            "<div class=\"space-y-6 prose prose-lg max-w-none \">{}</div>",
            html_output
        )
    };
}

pub fn about() -> Markup {
    html! {
        div class="max-w-6xl mx-auto h-full" {
            div class="bg-base-200 rounded-lg h-full max-h-[calc(100vh-200px)] min-h-[400px] overflow-y-auto p-6" {
                div class="prose prose-lg max-w-none" {
                    article class="bg-base-100 p-6 rounded-lg" {
                        (PreEscaped(&*README_HTML))
                    }
                }
            }
        }
    }
}
