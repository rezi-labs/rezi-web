use crate::database::recipes::Recipe;
use crate::routes::random_html_safe_id;
use crate::view::icons::{self, add_icon, link_icon, spark_icon, wand_icon};
use maud::{Markup, html};

pub fn recipes(recipes: Vec<Recipe>) -> Markup {
    html! {
        div .p-2 {
            div class="card bg-base-100 shadow-xl" {
                div class="card-body" {

                    h2 class="card-title text-2xl mb-4" {
                        svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" {
                            }
                        }
                        "Recipes"
                        (add_modal())
                    }


                    div id="recipe-list" class="h-[600px] overflow-y-auto pr-2" {
                        div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4" {
                            @for recipe in recipes {
                                (recipe_row(&recipe))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn recipe_row(recipe: &Recipe) -> Markup {
    let recipe_id = recipe.id();
    let recipe_id_spinner = format!("indicator-{}", recipe_id);
    let recipe_spinner_target = format!("#indicator-{}", recipe_id);
    html! {
        div id=(format!("recipe-{}", recipe.id())) class="w-full" {
            div class="card bg-base-100 border border-base-300 shadow-lg hover:shadow-xl transition-shadow duration-200 h-full" {
                div class="card-body p-4" {
                    // Header with title
                    div class="card-title justify-between items-start mb-3" {
                        div class="flex-1" {
                            @if let Some(title) = recipe.title() {
                                h3 class="text-xl font-bold cursor-pointer hover:text-primary transition-colors"
                                    hx-get=(format!("/recipes/{}/edit", recipe.id()))
                                    hx-target=(format!("#recipe-{}", recipe.id()))
                                    hx-swap="outerHTML"
                                    title="Click to edit" {
                                    (title)
                                }
                            } @else {
                                h3 class="text-xl font-bold cursor-pointer hover:text-primary italic text-base-content/60 transition-colors"
                                    hx-get=(format!("/recipes/{}/edit", recipe.id()))
                                    hx-target=(format!("#recipe-{}", recipe.id()))
                                    hx-swap="outerHTML"
                                    title="Click to edit" {
                                    "Untitled Recipe"
                                }
                            }
                        }

                        // Quick action buttons in header
                        div class="flex gap-1" {
                            @if let Some(url) = recipe.url() {
                                @if !url.is_empty() {
                                    a class="btn btn-sm btn-ghost btn-circle"
                                        href={
                                            @if url.contains("://") {
                                                (url)
                                            } @else {
                                                (format!("https://{}", url))
                                            }
                                        }
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        title="Open URL" {
                                        (link_icon())
                                    }
                                }
                            }

                            button class="btn btn-sm btn-error btn-ghost btn-circle"
                                hx-delete=(format!("/recipes/{}", recipe.id()))
                                hx-target=(format!("#recipe-{}", recipe.id()))
                                hx-swap="outerHTML"
                                hx-confirm="Are you sure you want to delete this recipe?"
                                title="Delete recipe" {
                                (icons::delete_icon())
                            }
                        }
                    }

                    // URL section (if exists)
                    @if let Some(url) = recipe.url() {
                        @if !url.is_empty() {
                            div class="mb-3" {
                                div class="flex items-center gap-2 text-sm text-info" {
                                    (link_icon())
                                    a href={
                                        @if url.contains("://") {
                                            (url)
                                        } @else {
                                            (format!("https://{}", url))
                                        }
                                    } target="_blank" rel="noopener noreferrer" class="hover:underline truncate font-medium" {
                                        (url)
                                    }
                                }
                            }
                        }
                    }

                    // Content section
                    div class="mb-4" {
                        div class="bg-base-200 rounded-lg p-3" {
                            div class="text-base-content/90 whitespace-pre-wrap text-sm leading-relaxed"
                                style="display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical; overflow: hidden;" {
                                (recipe.content())
                            }
                        }
                    }

                    // Action buttons
                    div class="card-actions justify-between items-center pt-2 border-t border-base-300" {
                        div class="text-xs text-base-content/60" {
                            "Created: " (recipe.created_at.format("%b %d, %Y"))
                        }

                        div class="flex gap-2" {
                            @if let Some(url) = recipe.url() {
                                @if !url.is_empty() {
                                    form hx-post="/chat" hx-swap="none" class="inline" {
                                        input type="hidden" name="message" value=(url);
                                        input type="hidden" name="is_url" value="true";
                                        button class="btn btn-xs btn-primary btn-outline" type="submit" title="Use URL with AI" hx-indicator=(recipe_spinner_target) {
                                            (spark_icon())
                                            span id=(recipe_id_spinner) class="htmx-indicator loading loading-spinner loading-xs" {}
                                            "URL"
                                        }
                                    }
                                }
                            }

                            form hx-post="/chat" hx-swap="none" class="inline" {
                                input type="hidden" name="message" value=(recipe.content());
                                input type="hidden" name="is_content" value="true";
                                button class="btn btn-xs btn-secondary btn-outline" type="submit" title="Use content with AI" hx-indicator=(recipe_spinner_target) {
                                     (wand_icon())
                                     span id=(recipe_id_spinner) class="htmx-indicator loading loading-spinner loading-xs" {}
                                    "Content"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn recipe_edit_row(recipe: &Recipe) -> Markup {
    html! {
        div id=(format!("recipe-{}", recipe.id())) class="" {
            div class="card bg-base-200 shadow-lg border-2 border-primary" {
                div class="card-body p-4" {
                    form class="flex flex-col gap-4"
                        hx-patch=(format!("/recipes/{}", recipe.id()))
                        hx-target=(format!("#recipe-{}", recipe.id()))
                        hx-swap="outerHTML" {

                        div class="form-control" {
                            fieldset class="fieldset"{
                                legend class="fieldset-legend" { "Recipe Title" }
                                input class="input input-bordered w-full"
                                    type="text"
                                    name="title"
                                    value=(recipe.title().unwrap_or(""))
                                    placeholder="Enter recipe title (optional)"
                                    autofocus;
                            }
                        }

                        div class="form-control" {
                             fieldset class="fieldset"{
                                 legend class="fieldset-legend" {"Recipe URL" }
                                 input class="input input-bordered w-full"
                                     type="url"
                                     name="url"
                                     value=(recipe.url().unwrap_or(""))
                                     placeholder="Enter recipe URL (optional)";
                             }

                        }

                        div class="form-control" {
                             fieldset class="fieldset"{
                                 legend class="fieldset-legend" {"Recipe Content" }
                                 textarea class="textarea textarea-bordered h-32"
                                     name="content"
                                     placeholder="Enter recipe content, instructions, or notes..."
                                     required {
                                     (recipe.content())
                                 }
                             }

                        }

                        div class="card-actions gap-2 pt-2" {
                            button class="btn btn-primary" type="submit" {
                                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" {
                                    }
                                }
                                "Save Changes"
                            }

                            button class="btn btn-ghost" type="button"
                                hx-get=(format!("/recipes/{}/cancel", recipe.id()))
                                hx-target=(format!("#recipe-{}", recipe.id()))
                                hx-swap="outerHTML" {
                                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" {
                                    }
                                }
                                "Cancel"
                            }

                            button class="btn btn-error btn-outline" type="button"
                                hx-delete=(format!("/recipes/{}", recipe.id()))
                                hx-target=(format!("#recipe-{}", recipe.id()))
                                hx-swap="outerHTML"
                                hx-confirm="Are you sure you want to delete this recipe?" {
                                svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" {
                                    }
                                }
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn add_modal() -> Markup {
    let modal_id = format!("modal{}", random_html_safe_id());
    html! {
        button class="btn" onclick=(format!("{modal_id}.showModal()")) {
            (add_icon())
        }
        dialog id=(modal_id) class="modal" {
            div class="modal-box" {
                h3 class="text-lg font-bold" {
                    "Add a Recipe"
                }
                div class="py-4" {
                    "Manual recipes can be used like any other recipe."
                }
                form method="dialog" class="flex flex-col gap-2 mb-4" hx-post="/recipes" hx-target="#recipe-list > div" hx-swap="beforeend" hx-on--after-request=(format!("{modal_id}.close();this.reset();")) {
                    input class="input input-bordered" type="text" name="title" placeholder="Recipe title (optional)"{}
                    input class="input input-bordered" type="url" name="url" placeholder="Recipe URL (optional)"{}
                    textarea class="textarea textarea-bordered" name="content" placeholder="Recipe content or notes..." rows="3" required{}
                    button class="btn btn-primary" type="submit" {
                        (add_icon())
                        "Add Recipe"
                    }
                    button class="btn" type="button" onclick=(format!("{modal_id}.close()")) {
                        "Close"
                    }
                }
            }
            form method="dialog" class="modal-backdrop"{
                button{"close"}
            }
        }
    }
}
