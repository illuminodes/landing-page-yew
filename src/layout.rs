use web_sys::{
    js_sys::Function,
    wasm_bindgen::{prelude::Closure, JsCast},
    Element, HtmlElement,
};
use yew::prelude::*;

use crate::{GITHUB_LINK, MEDIUM_LINK, PRIMAL_LINK, X_LINK, MAIL_TO_LINK};

#[function_component(AppLayout)]
pub fn app_layout(props: &html::ChildrenProps) -> Html {
    html! {
        <div class="flex flex-col items-center gap-16 h-full w-full relative font-nostr-regular">
            <img src="/public/assets/background-principal.png" alt="Illuminodes Logo"
                class="w-full mx-8 absolute md:-translate-y-48 lg:-translate-y-72 xl:-translate-y-96 sm:-mt-36 z-10" />
            <AppHeader />
                {props.children.clone()}
            <AppFooter />
        </div>
    }
}

#[function_component(AppSticky)]
pub fn app_header() -> Html {
    html! {
        <div
            class="flex flex-row sm:flex-col gap-6 items-center justify-evenly z-50 bg-nostr-light rounded-lg sm:fixed sm:top-1/2 sm:right-0 mx-8 p-2 sm:py-8 sm:px-2">
            <a target="_blank" href={MAIL_TO_LINK} class="text-white hover:text-nostr-dark">
                <lucide_yew::Mail class="size-8 font-bold" />
            </a>
            <a target="_blank" href={X_LINK} class="text-white hover:text-nostr-dark">
                <img src="/public/assets/x.png" alt="twitter link" class="h-8" />
            </a>
            <a target="_blank" href={PRIMAL_LINK} class="text-white hover:text-nostr-dark">
                <img src="/public/assets/nostr.png" alt="primal link" class="h-8" />
            </a>
            <a target="_blank" href={MEDIUM_LINK} class="text-white hover:text-nostr-dark">
                <img src="/public/assets/medium.png" alt="medium link" class="h-8" />
            </a>
            <a target="_blank" href={GITHUB_LINK}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 stroke-white" viewBox="0 0 24 24"
                    stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                    <path
                        d="M5.315 2.1c.791 -.113 1.9 .145 3.333 .966l.272 .161l.16 .1l.397 -.083a13.3 13.3 0 0 1 4.59 -.08l.456 .08l.396 .083l.161 -.1c1.385 -.84 2.487 -1.17 3.322 -1.148l.164 .008l.147 .017l.076 .014l.05 .011l.144 .047a1 1 0 0 1 .53 .514a5.2 5.2 0 0 1 .397 2.91l-.047 .267l-.046 .196l.123 .163c.574 .795 .93 1.728 1.03 2.707l.023 .295l.007 .272c0 3.855 -1.659 5.883 -4.644 6.68l-.245 .061l-.132 .029l.014 .161l.008 .157l.004 .365l-.002 .213l-.003 3.834a1 1 0 0 1 -.883 .993l-.117 .007h-6a1 1 0 0 1 -.993 -.883l-.007 -.117v-.734c-1.818 .26 -3.03 -.424 -4.11 -1.878l-.535 -.766c-.28 -.396 -.455 -.579 -.589 -.644l-.048 -.019a1 1 0 0 1 .564 -1.918c.642 .188 1.074 .568 1.57 1.239l.538 .769c.76 1.079 1.36 1.459 2.609 1.191l.001 -.678l-.018 -.168a5.03 5.03 0 0 1 -.021 -.824l.017 -.185l.019 -.12l-.108 -.024c-2.976 -.71 -4.703 -2.573 -4.875 -6.139l-.01 -.31l-.004 -.292a5.6 5.6 0 0 1 .908 -3.051l.152 -.222l.122 -.163l-.045 -.196a5.2 5.2 0 0 1 .145 -2.642l.1 -.282l.106 -.253a1 1 0 0 1 .529 -.514l.144 -.047l.154 -.03z"
                        stroke-width="0" fill="currentColor" />
                </svg>
            </a>

        </div>
    }
}

#[function_component(AppHeader)]
pub fn app_header() -> Html {
    use_effect_with((), move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let anchors = document.query_selector_all("a[href^='#']").unwrap();
        let for_each_function = {
            Closure::<dyn Fn(Element)>::new(Box::new(move |anchor: Element| {
                let listener: Function =
                    web_sys::wasm_bindgen::closure::Closure::<dyn Fn(Event)>::new(Box::new(
                        move |event: web_sys::Event| {
                            event.prevent_default();
                            let new_document = web_sys::window().unwrap().document().unwrap();
                            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
                            let id = target.get_attribute("href").unwrap();
                            let element = new_document.query_selector(&id).unwrap().unwrap();
                            element.scroll_into_view();
                        },
                    )
                        as Box<dyn Fn(web_sys::Event)>)
                    .into_js_value()
                    .unchecked_into();
                anchor
                    .add_event_listener_with_callback("click", &listener)
                    .unwrap();
            }) as Box<dyn Fn(web_sys::Element)>)
        };
        anchors
            .for_each(for_each_function.as_ref().unchecked_ref())
            .expect("Error");
        || {}
    });
    html! {
        <div class="w-full h-fit p-4 px-12 bg-nostr-black opacity-90 z-20 bg-gradient-to-b from-nostr-black to-transparent">
            <div class="w-full flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <img src="/public/assets/illumLogoGray.png" alt="Illuminodes Logo" class="h-12" />
                    <h6 class="text-2xl font-bold font-nostr-bold">{"illuminodes"}</h6>
                </div>
                <div class="hidden sm:flex items-center gap-4 font-bold text-base lg:text-lg">
                    <a href="#about-us" class="text-white hover:text-nostr-light">{"About Us"}</a>
                    <p class="text-white">{"|"}</p>
                    <a href="#technology" class="text-white hover:text-nostr-light">{"Technology"}</a>
                    <p class="text-white">{"|"}</p>
                    <a href="#contact-us" class="text-white hover:text-nostr-light">{"Contact Us"}</a>
                </div>
            </div>
        </div>
    }
}

#[function_component(AppFooter)]
pub fn app_footer() -> Html {
    html! {
        <div class="w-full flex flex-col items-center z-10 text-center rounded-t-full bg-nostr-dark py-12">
            <h2 id="contact-us">{"Follow Us"}</h2>
            <div class="flex flex-row items-center gap-8 text-center mt-4 text-base sm:text-lg flex-wrap max-w-64 sm:max-w-sm md:max-w-xl justify-evenly">
                <a target="_blank" href={MAIL_TO_LINK} class="hover:text-nostr-light flex flex-col items-center">
                    <lucide_yew::Mail class="size-6 sm:size-8 md:size-12" />
                    <p class="text-xs sm:text-sm md:text-base">{"Email"}</p>
                </a>
                <a target="_blank" href={X_LINK} class="hover:text-nostr-light flex flex-col items-center">
                    <img src="/public/assets/x.png" alt="Twitter Icon" class="size-6 sm:size-8 md:size-12" />
                    <p class="text-xs sm:text-sm md:text-base">{"@illuminodes"}</p>
                </a>
                <a target="_blank" href={PRIMAL_LINK} class="hover:text-nostr-light flex flex-col items-center">
                    <img src="/public/assets/nostr.png" alt="Nostr Icon" class="size-6 sm:size-8 md:size-12" />
                    <p class="text-xs sm:text-sm md:text-base">{"Nostr"}</p>
                </a>
                <a target="_blank" href={MEDIUM_LINK} class="hover:text-nostr-light flex flex-col items-center">
                    <img src="/public/assets/medium.png" alt="Medium Icon" class="size-6 sm:size-8 md:size-12" />
                    <p class="text-xs sm:text-sm md:text-base">{"Medium"}</p>
                </a>
                <a target="_blank" href={GITHUB_LINK} class="hover:text-nostr-light flex flex-col items-center">
                    <lucide_yew::Github class="size-6 sm:size-8 md:size-12 stroke-white" />
                    // <svg xmlns="http://www.w3.org/2000/svg" class="size-6 sm:size-8 md:size-12 stroke-white" viewBox="0 0 24 24"
                    //     stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                    //     <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                    //     <path
                    //         d="M5.315 2.1c.791 -.113 1.9 .145 3.333 .966l.272 .161l.16 .1l.397 -.083a13.3 13.3 0 0 1 4.59 -.08l.456 .08l.396 .083l.161 -.1c1.385 -.84 2.487 -1.17 3.322 -1.148l.164 .008l.147 .017l.076 .014l.05 .011l.144 .047a1 1 0 0 1 .53 .514a5.2 5.2 0 0 1 .397 2.91l-.047 .267l-.046 .196l.123 .163c.574 .795 .93 1.728 1.03 2.707l.023 .295l.007 .272c0 3.855 -1.659 5.883 -4.644 6.68l-.245 .061l-.132 .029l.014 .161l.008 .157l.004 .365l-.002 .213l-.003 3.834a1 1 0 0 1 -.883 .993l-.117 .007h-6a1 1 0 0 1 -.993 -.883l-.007 -.117v-.734c-1.818 .26 -3.03 -.424 -4.11 -1.878l-.535 -.766c-.28 -.396 -.455 -.579 -.589 -.644l-.048 -.019a1 1 0 0 1 .564 -1.918c.642 .188 1.074 .568 1.57 1.239l.538 .769c.76 1.079 1.36 1.459 2.609 1.191l.001 -.678l-.018 -.168a5.03 5.03 0 0 1 -.021 -.824l.017 -.185l.019 -.12l-.108 -.024c-2.976 -.71 -4.703 -2.573 -4.875 -6.139l-.01 -.31l-.004 -.292a5.6 5.6 0 0 1 .908 -3.051l.152 -.222l.122 -.163l-.045 -.196a5.2 5.2 0 0 1 .145 -2.642l.1 -.282l.106 -.253a1 1 0 0 1 .529 -.514l.144 -.047l.154 -.03z"
                    //         stroke-width="0" fill="currentColor" />
                    // </svg>
                    <p class="text-xs sm:text-sm md:text-base">{"GitHub"}</p>
                </a>
            </div>
        </div>
    }
}
