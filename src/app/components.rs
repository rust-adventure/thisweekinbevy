use leptos::*;

#[component]
pub fn AboutSection(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) =
        create_signal(false);

    view! {
        <section class=class>
            <h2 class="flex items-center font-mono text-sm font-medium leading-7 text-ctp-text">
                <TinyWaveFormIcon
                    start_color="fill-violet-300"
                    end_color="fill-pink-300"
                    class="h-2.5 w-2.5"
                />
                <span class="ml-2.5">About</span>
            </h2>
            <p class=move || {
                format!(
                    "mt-2 text-base leading-7 text-ctp-text {}",
                    if is_expanded.get() { "lg:line-clamp-4" } else { "" },
                )
            }>

                This Week in Bevy is a curated roundup covering week-to-week activity in the Bevy ecosystem
            </p>
            {move || {
                is_expanded
                    .get()
                    .then_some(
                        view! {
                            <button
                                type="button"
                                class="mt-2 hidden text-sm font-bold leading-6 text-ctp-pink hover:text-pink-700 active:text-pink-900 lg:inline-block"
                                on:click=move |_| set_is_expanded(true)
                            >
                                Show more
                            </button>
                        },
                    )
            }}

        </section>
    }
}

#[component]
pub fn Container(
    #[prop(into, default = "".to_string())] class: String,
    #[prop(optional, default = false)] center: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("lg:px-8 {class}")>
            <div class=format!("lg:max-w-4xl {}", if center {"mx-auto"} else {""})>
                <div class="mx-auto px-4 sm:px-6 md:max-w-2xl md:px-4 lg:px-0">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn TinyWaveFormIcon(
    #[prop(into, default = "".to_string())]
    start_color: String,
    #[prop(into, default = "".to_string())]
    end_color: String,
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    view! {
        <svg aria-hidden="true" viewBox="0 0 10 10" class=class>
            <path
                d="M0 5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V5Z"
                class=start_color
            ></path>
            <path
                d="M6 1a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H7a1 1 0 0 1-1-1V1Z"
                class=end_color
            ></path>
        </svg>
    }
}

#[component]
pub fn Divider(
    #[prop(into)] title: String,
) -> impl IntoView {
    view! {
        <div class="relative mt-12 mb-6">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="w-full border-t border-ctp-surface1"></div>
            </div>
            <div class="relative flex justify-center">
                <span class="bg-ctp-base px-3 text-base font-semibold leading-6 text-ctp-text">
                    {title}
                </span>
            </div>
        </div>
    }
}

pub enum DescriptionColor {
    Blue,
    Pink,
    Lavender,
    Teal,
}
impl DescriptionColor {
    fn border_y(&self) -> &str {
        match self {
            DescriptionColor::Blue => "border-y-ctp-blue",
            DescriptionColor::Pink => "border-y-ctp-pink",
            DescriptionColor::Lavender => {
                "border-y-ctp-lavender"
            }
            DescriptionColor::Teal => "border-y-ctp-teal",
        }
    }
    fn border_t(&self) -> &str {
        match self {
            DescriptionColor::Blue => "border-t-ctp-blue",
            DescriptionColor::Pink => "border-t-ctp-pink",
            DescriptionColor::Lavender => {
                "border-t-ctp-lavender"
            }
            DescriptionColor::Teal => "border-t-ctp-teal",
        }
    }
}
#[component]
pub fn DividerWithDescription(
    #[prop(into)] color: DescriptionColor,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "mt-8 relative isolate overflow-hidden bg-ctp-base px-6 py-24 sm:py-32 lg:px-8 border-y-8 {}",
            color.border_y(),
        )>
            <img
                src="https://res.cloudinary.com/dilgcuzda/image/upload/c_scale,w_600/thisweekinbevy/01HTAXBQ8H38BYVD3VEXCKVDS4.avif"
                loading="lazy"
                alt=""
                class="absolute inset-0 -z-10 w-full aspect-h-7 aspect-w-10 object-cover opacity-30 blur"
            />
            <div class="mx-auto max-w-2xl text-center">
                <h2 class="text-4xl py-4 font-black tracking-tight text-ctp-text sm:text-6xl rounded-t-xl bg-gradient-to-bl from-50% from-ctp-base to-ctp-crust -skew-y-3 border border-t-ctp-surface0 border-r-ctp-surface0 border-b-ctp-crust border-l-ctp-crust">
                    {title}
                </h2>
                <p class=format!(
                    "-mt-[20px] p-4 text-lg leading-8 text-ctp-text bg-gradient-to-bl from-50% from-ctp-base to-ctp-crust border-t-[40px] {}",
                    color.border_t(),
                )>{description}</p>
            </div>
        </div>
    }
}
