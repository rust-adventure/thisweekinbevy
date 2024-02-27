use leptos::*;

#[component]
pub fn AboutSection(
    #[prop(into, default = "".to_string())] class: String,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) =
        create_signal(false);

    view! {
        <section class=class>
            <h2 class="flex items-center font-mono text-sm font-medium leading-7 text-slate-900">
                <TinyWaveFormIcon
                    start_color="fill-violet-300"
                    end_color="fill-pink-300"
                    class="h-2.5 w-2.5"
                />
                <span class="ml-2.5">About</span>
            </h2>
            <p class=move || {
                format!(
                    "mt-2 text-base leading-7 text-slate-700 {}",
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
                                class="mt-2 hidden text-sm font-bold leading-6 text-pink-500 hover:text-pink-700 active:text-pink-900 lg:inline-block"
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
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("lg:px-8 {class}")>
            <div class="lg:max-w-4xl">
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
pub fn Divider(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <div class="relative mt-12 mb-6">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="w-full border-t border-gray-300"></div>
            </div>
            <div class="relative flex justify-center">
                <span class="bg-white px-3 text-base font-semibold leading-6 text-gray-900">
                    {title}
                </span>
            </div>
        </div>
    }
}