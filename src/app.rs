use crate::{
    app::{
        components::{AboutSection, TinyWaveFormIcon},
        routes::{
            admin::{self, AdminWrapper},
            custom,
            index::Home,
            issue,
        },
    },
    error_template::{AppError, ErrorTemplate},
    Username,
};
use leptos::{either::Either, logging::log, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};
mod components;
mod routes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // in --release, these must be provided
    #[cfg(not(debug_assertions))]
    let pkg_path: &'static str = std::env!("CDN_PKG_PATH");
    #[cfg(not(debug_assertions))]
    let cdn_path: &'static str = std::env!("CDN_PATH");

    // in debug, we can use the defaults or the env
    // vars
    #[cfg(debug_assertions)]
    let pkg_path: &'static str =
        std::option_env!("CDN_PKG_PATH").unwrap_or("/pkg");
    #[cfg(debug_assertions)]
    let cdn_path: &'static str =
        std::option_env!("CDN_PATH").unwrap_or("/");

    view! {
        <!DOCTYPE html>
        <html
            lang="en"
            class="h-full bg-ctp-base antialiased"
            >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="preconnect" href="https://cdn.thisweekinbevy.com"/>
                <style>
                    r#"@font-face {
                    font-family: "PP Neue Montreal";
                    src: url("https://cdn.thisweekinbevy.com/pp-neue-montreal/PPNeueMontreal-Variable.woff2")
                    format("woff2");
                    font-weight: 100 900;
                    font-display: swap;
                    font-style: normal;
                    }"#
                </style>
                <link
                    rel="preload"
                    as_="font"
                    type_="font/woff2"
                    crossorigin="anonymous"
                    href="https://cdn.thisweekinbevy.com/pp-neue-montreal/PPNeueMontreal-Variable.woff2"
                />
                <meta name="og:site_name" content="This Week in Bevy"/>

                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() root=cdn_path islands=true/>
                <HashedStylesheet options=options root=cdn_path/>
                // <link rel="stylesheet" id="leptos" href=format!("{pkg_path}/this-week-in-bevy.css")/>
                // <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <link
                    rel="apple-touch-icon"
                    sizes="180x180"
                    href=format!("{cdn_path}/apple-touch-icon.png")
                />
                <link
                    rel="icon"
                    type="image/png"
                    sizes="32x32"
                    href=format!("{cdn_path}/favicon-32x32.png")
                />
                <link
                    rel="icon"
                    type="image/png"
                    sizes="16x16"
                    href=format!("{cdn_path}/favicon-16x16.png")
                />
                <link rel="manifest" href=format!("{cdn_path}/site.webmanifest")/>
                <meta name="msapplication-TileColor" content="#cdd6f4"/>
                <meta name="theme-color" content="#cdd6f4"/>

                <MetaTags/>

            </head>
            <body class="flex min-h-full">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets,
    // titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Router>
            <Wrapper>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.
                insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/>
                }.into_view() }>
                    <Route path=path!("") view=Home/>
                    <Route path=path!("/issue/:slug") view=issue::Issue/>
                    <Route path=path!("/custom/:slug") view=custom::Issue/>
                    <Route path=path!("/login") view=Login/>
                    <ProtectedParentRoute
                        path=path!("/admin")
                        redirect_path=|| "/login"
                        condition=move || {
                            let logged_in_user = use_context::<Option<Username>>().flatten();
                            Some(logged_in_user == Some(Username("ChristopherBiscardi".to_string())))
                        }
                        view=AdminWrapper
                    >
                        <Route path=path!("") view=admin::AdminHomepage/>
                        <Route path=path!("/issue") view=admin::issues::Issues/>
                        <Route path=path!("/issue/:id") view=admin::issue::Issue/>
                        <Route path=path!("/showcase") view=admin::showcase::Showcase/>
                        <Route path=path!("/showcase/:id") view=admin::showcase::id::Showcase/>
                        <Route path=path!("/crate_release") view=admin::crate_release::CrateRelease/>
                        <Route
                            path=path!("/crate_release/:id")
                            view=admin::crate_release::id::CrateRelease
                        />
                        <Route path=path!("/devlog") view=admin::devlog::Devlog/>
                        <Route path=path!("/devlog/:id") view=admin::devlog::id::Devlog/>
                        <Route path=path!("/educational") view=admin::educational::Educational/>
                        <Route path=path!("/educational/:id") view=admin::educational::id::Educational/>
                        <Route path=path!("/images") view=admin::image::Image/>
                        <Route path=path!("/github") view=admin::github::GitHub/>
                    </ProtectedParentRoute>
                </Routes>
            </Wrapper>
        </Router>
    }
}

#[component]
fn Login(// Query(NextUrl { next }): Query<NextUrl>,
) -> impl IntoView {
    let message = Some("hello!");
    view! {
        <div>

            {message
                .map(|msg| {
                    view! {
                        <span>
                            <strong>{msg}</strong>
                        </span>
                    }
                })}
            <form method="post">
                <input type="submit" value="GitHub Login"/>

            // {% if let Some(next) = next %}
            // <input type="hidden" name="next" value="{{next}}" />
            // {% endif %}
            </form>
        </div>
    }
}

#[component]
fn PersonIcon(
    #[prop(into)] class: String,
) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 11 12" class=class>
            <path d="M5.019 5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Zm3.29 7c1.175 0 2.12-1.046 1.567-2.083A5.5 5.5 0 0 0 5.019 7 5.5 5.5 0 0 0 .162 9.917C-.39 10.954.554 12 1.73 12h6.578Z"></path>
        </svg>
    }
}

#[component]
fn YouTubeIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 28.57 20" class=class>
            <path
                fill="#FF0000"
                d="M27.973 3.123A3.578 3.578 0 0 0 25.447.597C23.22 0 14.285 0 14.285 0S5.35 0 3.123.597A3.578 3.578 0 0 0 .597 3.123C0 5.35 0 10 0 10s0 4.65.597 6.877a3.578 3.578 0 0 0 2.526 2.526C5.35 20 14.285 20 14.285 20s8.935 0 11.162-.597a3.578 3.578 0 0 0 2.526-2.526C28.57 14.65 28.57 10 28.57 10s-.002-4.65-.597-6.877Z"
            ></path>
            <path fill="#fff" d="M11.425 14.285 18.848 10l-7.423-4.285v8.57Z"></path>
        </svg>
    }
}

#[component]
fn GitHubIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" class=class fill="none">
            <path
                fill="var(--brand-github)"
                fill-rule="evenodd"
                d="M512 0C229.12 0 0 229.12 0 512c0 226.56 146.56 417.92 350.08 485.76 25.6 4.48 35.2-10.88 35.2-24.32 0-12.16-.64-52.48-.64-95.36-128.64 23.68-161.92-31.36-172.16-60.16-5.76-14.72-30.72-60.16-52.48-72.32-17.92-9.6-43.52-33.28-.64-33.92 40.32-.64 69.12 37.12 78.72 52.48 46.08 77.44 119.68 55.68 149.12 42.24 4.48-33.28 17.92-55.68 32.64-68.48-113.92-12.8-232.96-56.96-232.96-252.8 0-55.68 19.84-101.76 52.48-137.6-5.12-12.8-23.04-65.28 5.12-135.68 0 0 42.88-13.44 140.8 52.48 40.96-11.52 84.48-17.28 128-17.28 43.52 0 87.04 5.76 128 17.28 97.92-66.56 140.8-52.48 140.8-52.48 28.16 70.4 10.24 122.88 5.12 135.68 32.64 35.84 52.48 81.28 52.48 137.6 0 196.48-119.68 240-233.6 252.8 18.56 16 34.56 46.72 34.56 94.72 0 68.48-.64 123.52-.64 140.8 0 13.44 9.6 29.44 35.2 24.32C877.44 929.92 1024 737.92 1024 512 1024 229.12 794.88 0 512 0Z"
                clip-rule="evenodd"
            ></path>
        </svg>
    }
}

#[component]
fn RSSIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class=class
            width="256"
            height="256"
            viewBox="0 0 8 8"
        >
            <rect width="8" height="8" rx="1.5" style="stroke:none;fill:orange"></rect>
            <circle cx="2" cy="6" r="1" class="symbol"></circle>
            <path d="M1 4a3 3 0 0 1 3 3h1a4 4 0 0 0-4-4z" class="symbol"></path>
            <path d="M1 2a5 5 0 0 1 5 5h1a6 6 0 0 0-6-6z" class="symbol"></path>
        </svg>
    }
}

/// Renders the home page of your application.
#[component]
fn Wrapper(children: Children) -> impl IntoView {
    let maintainers =
        ["chris biscardi", "The Bevy Community"];
    let mntnrs = maintainers.iter().enumerate().map(|(i, cite)| {
        if i == 0 {
            Either::Left(view! {
                <>
                    <span>{cite.to_string()}</span>
                </>
            })
        } else {
            Either::Right(view! {
                <>
                    <span aria-hidden="true" class="text-ctp-text">
                        "/"
                    </span>
                    <span>{cite.to_string()}</span>
                </>
            })
        }
        }).collect_view();
    view! {
        <div class="w-full">
            <header class="bg-ctp-mantle lg:fixed lg:inset-y-0 lg:left-0 lg:flex lg:w-112 lg:items-start lg:overflow-y-auto xl:w-120">
                <div class="hidden lg:sticky lg:top-0 lg:flex lg:w-16 lg:flex-none lg:items-center lg:whitespace-nowrap lg:py-12 lg:text-sm lg:leading-7 lg:[writing-mode:vertical-rl]">
                    <span class="font-mono text-ctp-text">Curated by</span>
                    <span class="mt-6 flex gap-6 font-bold text-ctp-text">{mntnrs.clone()}</span>
                </div>
                <div class="relative z-10 mx-auto px-4 pb-4 pt-10 sm:px-6 md:max-w-2xl md:px-4 lg:min-h-full lg:flex-auto lg:border-x lg:border-ctp-crust lg:px-8 lg:py-12 xl:px-12">
                    <a
                        href="/"
                        class="relative mx-auto block w-48 overflow-hidden rounded-lg bg-ctp-crust shadow-xl shadow-ctp-crust sm:w-64 sm:rounded-xl lg:w-auto lg:rounded-2xl"
                        aria-label="Homepage"
                    >
                        <picture>
                            <source
                                srcset="https://res.cloudinary.com/dilgcuzda/image/upload/v1708481576/thisweekinbevy/this-week-in-bevydark_wdnm2d.avif"
                                media="(prefers-color-scheme: dark)"
                            />
                            <img
                                class="w-full aspect-square"
                                src="https://res.cloudinary.com/dilgcuzda/image/upload/v1708481576/thisweekinbevy/this-week-in-bevylight_uddwes.avif"
                                alt=""
                            />
                        </picture>
                        <div class="absolute inset-0 rounded-lg ring-1 ring-inset ring-black/10 sm:rounded-xl lg:rounded-2xl"></div>
                    </a>
                    <div class="mt-10 text-center lg:mt-12 lg:text-left">
                        <p class="text-xl font-bold text-ctp-text">
                            <a href="/">This Week in Bevy</a>
                        </p>
                        <p class="mt-3 text-lg font-medium leading-8 text-ctp-text">
                            What happened this week in the Bevy Engine ecosystem
                        </p>
                    </div>
                    <AboutSection class="mt-12 hidden lg:block"/>
                    <section class="mt-10 lg:mt-12">
                        <h2 class="sr-only flex items-center font-mono text-sm font-medium leading-7 text-ctp-text lg:not-sr-only">
                            <TinyWaveFormIcon
                                start_color="fill-pink-300"
                                end_color="fill-rose-300"
                                class="h-2.5 w-2.5"
                            />
                            <span class="ml-2.5">Links</span>
                        </h2>
                        <div class="h-px bg-gradient-to-r from-slate-200/0 via-slate-200 to-slate-200/0 lg:hidden"></div>
                        <ul
                            role="list"
                            class="mt-4 flex justify-center gap-10 text-base font-medium leading-7 text-ctp-text sm:gap-8 lg:flex-col lg:gap-4"
                        >
                            <li class="flex">
                                <a
                                    href="https://www.youtube.com/playlist?list=PLWtPciJ1UMuAyAER9ASVEDRIz0DUspOeZ"
                                    class="group flex items-center"
                                    aria-label="YouTube Playlist"
                                >
                                    <YouTubeIcon class="h-8 w-8 fill-slate-400 group-hover:fill-slate-600"/>
                                    <span class="hidden sm:ml-3 sm:block">YouTube Playlist</span>
                                </a>
                            </li>
                            <li class="flex">
                                <a
                                    href="https://github.com/rust-adventure/thisweekinbevy"
                                    class="group flex items-center"
                                    aria-label="GitHub Repo"
                                >
                                    <GitHubIcon class="h-8 w-8 fill-slate-400 group-hover:fill-slate-600"/>
                                    <span class="hidden sm:ml-3 sm:block">GitHub Repo</span>
                                </a>
                            </li>
                            <li class="flex">
                                <a
                                    href="https://thisweekinbevy.com/feed.xml"
                                    class="group flex items-center"
                                    aria-label="Atom Feed"
                                >
                                    <RSSIcon class="h-8 w-8 fill-white group-hover:fill-white"/>
                                    <span class="hidden sm:ml-3 sm:block">Atom Feed</span>
                                </a>
                            </li>
                        </ul>
                    </section>
                </div>
            </header>
            <main class="border-t border-ctp-crust lg:relative lg:mb-28 lg:ml-112 lg:border-t-0 xl:ml-120">
                // <Waveform class="absolute left-0 top-0 h-20 w-full" />
                <div class="relative">{children()}</div>
            </main>
            <footer class="border-t border-ctp-crust bg-ctp-mantle py-10 pb-40 sm:py-16 sm:pb-32 lg:hidden">
                <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4">
                    <AboutSection/>
                    <h2 class="mt-8 flex items-center font-mono text-sm font-medium leading-7 text-ctp-text">
                        <PersonIcon class="h-3 w-auto fill-slate-300"/>
                        <span class="ml-2.5">Curated by</span>
                    </h2>
                    <div class="mt-2 flex gap-6 text-sm font-bold leading-7 text-ctp-text">
                        {mntnrs}
                    </div>
                </div>
            </footer>
            // <AudioPlayer />
            <div class="fixed inset-x-0 bottom-0 z-10 lg:left-112 xl:left-120"></div>
        </div>
    }
}
