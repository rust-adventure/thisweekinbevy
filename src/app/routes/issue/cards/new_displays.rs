use leptos::prelude::*;

use crate::app::routes::issue::{
    ImgDataTransformed, PROSE,
};

#[component]
pub fn ZeroImage(
    title: String,
    type_name: String,
    description: String,
    posted_date: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-950 mt-24">
            <div class="relative mx-auto -mt-12 max-w-7xl px-4 pb-16 sm:px-6 sm:pb-24 lg:px-8">
                <div class="mx-auto max-w-2xl text-center lg:max-w-4xl">
                <h2 class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">{title}</h2>
                </div>
                <div
                    class=format!("mt-4 text-slate-400 md:max-w-xl mx-auto {}", PROSE)
                    inner_html=description
                />
            </div>
        </div>
    }
}

#[component]
pub fn SingleImage(
    title: String,
    type_name: String,
    image: ImgDataTransformed,
    description: String,
    posted_date: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-900 mt-32">
            <div aria-hidden="true" class="relative">
                <img src=image.url alt=image.description class="h-96 object-cover md:max-w-4xl mx-auto rounded shadow border border-slate-600" />
                <div class="absolute inset-0 bg-linear-to-t from-slate-900 from-5% to-50%"></div>
            </div>

            <div class="relative mx-auto -mt-12 max-w-7xl px-4 pb-16 sm:px-6 sm:pb-24 lg:px-8">
                <div class="mx-auto max-w-2xl text-center lg:max-w-4xl">
                <h2 class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">{title}</h2>
                </div>
                <div
                    class=format!("mt-4 text-slate-400 md:max-w-xl mx-auto {}", PROSE)
                    inner_html=description
                />
            </div>
        </div>
    }
}

#[component]
pub fn TwoImage(
    title: String,
    type_name: String,
    images: Vec<ImgDataTransformed>,
    description: String,
    posted_date: Option<String>,
) -> impl IntoView {
    view! {
        <div>// class="bg-sky-950">
        <div class="mx-auto max-w-2xl px-4 py-24 sm:px-6 sm:py-32 lg:max-w-7xl lg:px-8">
            <section aria-labelledby="details-heading">
            <div class="flex flex-col items-center text-center">
                <h2 id="details-heading" class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">{title}</h2>
                <div
                    class=format!("mt-3 max-w-3xl text-lg text-slate-400 {}", PROSE)
                    inner_html=description
                />
            </div>

            <div class="mt-16 grid grid-cols-1 gap-y-16 lg:grid-cols-2 lg:gap-x-8">
                <div>
                <img src=images[0].url.clone() alt=images[0].description.clone() class="aspect-3/2 w-full rounded-lg object-cover rounded border border-slate-600" />
                // <p class="mt-8 text-base text-gray-500">The 20L model has enough space for 370 candy bars, 6 cylinders of chips, 1220 standard gumballs, or any combination of on-the-go treats that your heart desires. Yes, we did the math.</p>
                </div>
                <div>
                <img src=images[1].url.clone() alt=images[1].description.clone() class="aspect-3/2 w-full rounded-lg object-cover rounded border border-slate-600" />
                // <p class="mt-8 text-base text-gray-500">Up your snack organization game with multiple compartment options. The quick-access stash pouch is ready for even the most unexpected snack attacks and sharing needs.</p>
                </div>
            </div>
            </section>
        </div>
        </div>
    }
}

#[component]
pub fn ThreePlusImage(
    title: String,
    type_name: String,
    images: Vec<ImgDataTransformed>,
    description: String,
    posted_date: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-950">
            <div class="mx-auto grid max-w-2xl grid-cols-1 items-center gap-x-8 gap-y-16 px-4 py-24 sm:px-6 sm:py-32 lg:max-w-7xl lg:grid-cols-2 lg:px-8">
                <div>
                <h2 class="text-3xl font-bold tracking-tight text-slate-50 sm:text-4xl">{title}</h2>
                <div
                    class=format!("mt-4 text-slate-400 {}", PROSE)
                    inner_html=description
                />
                </div>
                <div class="grid grid-cols-2 grid-rows-2 gap-4 sm:gap-6 lg:gap-8">
                // <img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-feature-03-detail-01.jpg" alt="Walnut card tray with white powder coated steel divider and 3 punchout holes." class="rounded-lg bg-gray-100" />
                // <img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-feature-03-detail-02.jpg" alt="Top down view of walnut card tray with embedded magnets and card groove." class="rounded-lg bg-gray-100" />
                // <img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-feature-03-detail-03.jpg" alt="Side of walnut card tray with card groove and recessed card area." class="rounded-lg bg-gray-100" />
                // <img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-feature-03-detail-04.jpg" alt="Walnut card tray filled with cards and card angled in dedicated groove." class="rounded-lg bg-gray-100" />
                </div>
            </div>
        </div>
    }
}
