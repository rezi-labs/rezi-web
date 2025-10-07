use crate::user;
use maud::{Markup, html};

pub fn export_page(user: &user::User) -> Markup {
    crate::view::index(Some(render()), false, Some(user))
}

pub fn render() -> Markup {
    html! {
        div .p-2 {
            div class="card bg-base-100 shadow-xl" {
                div class="card-body" {
                    h2 class="card-title text-2xl mb-4" {
                        svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" {}
                        }
                        "Export Data"
                    }

                    div class="space-y-4" {
                        div class="alert alert-info" {
                            svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6" {
                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" {}
                            }
                            span { "Export your data in various formats for backup or external use." }
                        }

                        div class="grid gap-4 md:grid-cols-2" {
                            div class="card bg-base-200 shadow-sm" {
                                div class="card-body" {
                                    h3 class="card-title text-lg mb-2" {
                                        svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" {}
                                        }
                                        "Items"
                                    }
                                    p class="text-sm text-base-content/70 mb-4" {
                                        "Export your items as CSV for spreadsheets or PDF for printing and sharing."
                                    }
                                    div class="card-actions justify-end gap-2" {
                                        a href="/export/items/csv"
                                        target="_blank"
                                          class="btn btn-outline btn-sm"
                                          hx-swap="none" {
                                            svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" {}
                                            }
                                            "CSV"
                                        }
                                        a href="/export/items/pdf"
                                          target="_blank"
                                          class="btn btn-primary btn-sm"
                                          hx-swap="none" {
                                            svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                                path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" {}
                                            }
                                            "PDF"
                                        }
                                    }
                                }
                            }

                            div class="card bg-base-200 shadow-sm opacity-50" {
                                div class="card-body" {
                                    h3 class="card-title text-lg mb-2" {
                                        svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" {}
                                        }
                                        "Messages"
                                    }
                                    p class="text-sm text-base-content/70 mb-4" {
                                        "Export your chat messages and conversations."
                                    }
                                    div class="card-actions justify-end" {
                                        button class="btn btn-disabled btn-sm" disabled {
                                            "Coming Soon"
                                        }
                                    }
                                }
                            }

                            div class="card bg-base-200 shadow-sm opacity-50" {
                                div class="card-body" {
                                    h3 class="card-title text-lg mb-2" {
                                        svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" {}
                                        }
                                        "Recipes"
                                    }
                                    p class="text-sm text-base-content/70 mb-4" {
                                        "Export your saved recipes and cooking notes."
                                    }
                                    div class="card-actions justify-end" {
                                        button class="btn btn-disabled btn-sm" disabled {
                                            "Coming Soon"
                                        }
                                    }
                                }
                            }

                            div class="card bg-base-200 shadow-sm opacity-50" {
                                div class="card-body" {
                                    h3 class="card-title text-lg mb-2" {
                                        svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" {}
                                        }
                                        "Full Backup"
                                    }
                                    p class="text-sm text-base-content/70 mb-4" {
                                        "Export all your data as a complete backup archive."
                                    }
                                    div class="card-actions justify-end" {
                                        button class="btn btn-disabled btn-sm" disabled {
                                            "Coming Soon"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
