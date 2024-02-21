use crate::{
    app::{
        components::{AboutSection, TinyWaveFormIcon},
        routes::{
            admin::{self, AdminWrapper},
            index::Home,
            issue::Issue,
        },
    },
    error_template::{AppError, ErrorTemplate},
    Username,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod components;
mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets,
    // titles, meta tags, etc.
    provide_meta_context();

    let pkg_path: &'static str = std::option_env!("CDN_PKG_PATH").unwrap_or("/pkg");
    view! {
        <Html
            lang="en"
            // arbitrary additional attributes can be passed via `attr:`
            attr:class="h-full bg-white antialiased"
        />
        <Body class="flex min-h-full"/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href=format!("{pkg_path}/this-week-in-bevy.css")/>
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Archivo%20Black"/>
        // sets the document title
        <Title text="This Week in the Bevy Game Engine"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Wrapper>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/issue/:slug" view=Issue/>
                    <Route path="/draft/:slug" view=Issue/>
                    <Route path="/login" view=Login/>
                    <ProtectedRoute
                        path="/admin"
                        redirect_path="/login"
                        condition=|| {
                            use_context::<Option<Username>>().flatten()
                                == Some(Username("ChristopherBiscardi".to_string()))
                        }

                        view=AdminWrapper
                    >
                        <Route path="/" view=admin::AdminHomepage/>
                        <Route path="/issue" view=admin::issues::Issues/>
                        <Route path="/issue/:id" view=admin::issue::Issue/>
                        <Route path="/showcase" view=admin::Showcase/>
                        <Route path="/crate_release" view=admin::CrateRelease/>
                        <Route path="/devlog" view=admin::Devlog/>
                        <Route path="/educational" view=admin::Educational/>
                    </ProtectedRoute>
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
fn RSSIcon(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 32 32" class=class>
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M8.5 4h15A4.5 4.5 0 0 1 28 8.5v15a4.5 4.5 0 0 1-4.5 4.5h-15A4.5 4.5 0 0 1 4 23.5v-15A4.5 4.5 0 0 1 8.5 4ZM13 22a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm-6-6a9 9 0 0 1 9 9h3A12 12 0 0 0 7 13v3Zm5.74-4.858A15 15 0 0 0 7 10V7a18 18 0 0 1 18 18h-3a15 15 0 0 0-9.26-13.858Z"
            ></path>
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
            view! {
                <>
                    <span>{cite.to_string()}</span>
                </>
            }
        } else {
            view! {
                <>
                    <span aria-hidden="true" class="text-slate-400">
                        "/"
                    </span>
                    <span>{cite.to_string()}</span>
                </>
            }
        }
        }).collect::<Vec<_>>();
    view! {
        <div class="w-full">
            <header class="bg-slate-50 lg:fixed lg:inset-y-0 lg:left-0 lg:flex lg:w-112 lg:items-start lg:overflow-y-auto xl:w-120">
                <div class="hidden lg:sticky lg:top-0 lg:flex lg:w-16 lg:flex-none lg:items-center lg:whitespace-nowrap lg:py-12 lg:text-sm lg:leading-7 lg:[writing-mode:vertical-rl]">
                    <span class="font-mono text-slate-500">Curated by</span>
                    <span class="mt-6 flex gap-6 font-bold text-slate-900">{mntnrs.clone()}</span>
                </div>
                <div class="relative z-10 mx-auto px-4 pb-4 pt-10 sm:px-6 md:max-w-2xl md:px-4 lg:min-h-full lg:flex-auto lg:border-x lg:border-slate-200 lg:px-8 lg:py-12 xl:px-12">
                    <a
                        href="/"
                        class="relative mx-auto block w-48 overflow-hidden rounded-lg bg-slate-200 shadow-xl shadow-slate-200 sm:w-64 sm:rounded-xl lg:w-auto lg:rounded-2xl"
                        aria-label="Homepage"
                    >
                        <img
                            class="w-full"
                            src="https://res.cloudinary.com/dilgcuzda/image/upload/v1708481576/thisweekinbevy/this-week-in-bevylight_uddwes.avif"
                            alt=""
                            sizes="(min-width: 1024px) 20rem, (min-width: 640px) 16rem, 12rem"
                            priority
                        />
                        <div class="absolute inset-0 rounded-lg ring-1 ring-inset ring-black/10 sm:rounded-xl lg:rounded-2xl"></div>
                    </a>
                    <div class="mt-10 text-center lg:mt-12 lg:text-left">
                        <p class="text-xl font-bold text-slate-900">
                            <a href="/">This Week in Bevy</a>
                        </p>
                        <p class="mt-3 text-lg font-medium leading-8 text-slate-700">
                            What happened this week in the Bevy Engine ecosystem
                        </p>
                    </div>
                    <AboutSection class="mt-12 hidden lg:block"/>
                    <section class="mt-10 lg:mt-12">
                        <h2 class="sr-only flex items-center font-mono text-sm font-medium leading-7 text-slate-900 lg:not-sr-only">
                            <TinyWaveFormIcon
                                start_color="fill-pink-300"
                                end_color="fill-rose-300"
                                class="h-2.5 w-2.5"
                            />
                            <span class="ml-2.5">Listen</span>
                        </h2>
                        <div class="h-px bg-gradient-to-r from-slate-200/0 via-slate-200 to-slate-200/0 lg:hidden"></div>
                        <ul
                            role="list"
                            class="mt-4 flex justify-center gap-10 text-base font-medium leading-7 text-slate-700 sm:gap-8 lg:flex-col lg:gap-4"
                        >

                            {[("RSS Feed", RSSIcon)]
                                .map(|(label, Icon)| {
                                    view! {
                                        // ["YouTube", YouTubeIcon],
                                        <li class="flex">
                                            <a
                                                href="/"
                                                class="group flex items-center"
                                                aria-label=label
                                            >
                                                <Icon class="h-8 w-8 fill-slate-400 group-hover:fill-slate-600"/>
                                                <span class="hidden sm:ml-3 sm:block">{label}</span>
                                            </a>
                                        </li>
                                    }
                                })}

                        </ul>
                    </section>
                </div>
            </header>
            <main class="border-t border-slate-200 lg:relative lg:mb-28 lg:ml-112 lg:border-t-0 xl:ml-120">
                // <Waveform class="absolute left-0 top-0 h-20 w-full" />
                <div class="relative">{children()}</div>
            </main>
            <footer class="border-t border-slate-200 bg-slate-50 py-10 pb-40 sm:py-16 sm:pb-32 lg:hidden">
                <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4">
                    <AboutSection/>
                    <h2 class="mt-8 flex items-center font-mono text-sm font-medium leading-7 text-slate-900">
                        <PersonIcon class="h-3 w-auto fill-slate-300"/>
                        <span class="ml-2.5">Curated by</span>
                    </h2>
                    <div class="mt-2 flex gap-6 text-sm font-bold leading-7 text-slate-900">
                        {mntnrs}
                    </div>
                </div>
            </footer>
            <div class="fixed inset-x-0 bottom-0 z-10 lg:left-112 xl:left-120">// <AudioPlayer />
            </div>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! { <h1>"hello"</h1> }
}
