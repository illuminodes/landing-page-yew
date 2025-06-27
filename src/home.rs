use yew::prelude::*;

use crate::AppSticky;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
        <div
            class="flex flex-col items-center justify-center gap-8 z-20 max-w-sm sm:max-w-xl text-center mt-24 sm:mt-48 bg-img-fuzzy bg-cover bg-center">
            <h1>{"Integrity in Identity"}</h1>
            <p class="max-w-xs sm:max-w-sm md: max-w-md">{{crate::consts::WEBSITE_SUBTITLE}}</p>
            <a 
                target="_blank" 
                href={{crate::consts::MAIL_TO_LINK}}>
                <button
                    class="px-12 py-4 bg-nostr-light text-white font-bold text-lg lg:text-xl rounded-full hover:bg-nostr-dark">
                        {"Contact Us"}
                </button>
            </a>
            <AppSticky />
        </div>
        <div
            class="flex flex-col items-center justify-center gap-4 z-20 text-center  mt-4 mx-16 sm:mt-8 sm:mx-32 md:mt-16 md:mx-48">
            <h3 id="about-us">{"About Us"}</h3>
            <div class="flex flex-col items-center gap-8 text-center relative">
                <img src="/public/assets/purple-background.png" alt="Purple Background"
                    class="w-full sm:scale-50 z-10 absolute sm:-translate-y-64" />
                <div class="flex flex-col items-center gap-8 md:gap-16 text-center z-20">
                    <h2>{"illuminodes"}</h2>
                    <p class="max-w-lg">{{crate::consts::WEBSITE_ABOUT_US}}</p>
                    <div class="flex flex-col sm:flex-row  justify-center items-center gap-8 text-center">
                        <div class="flex flex-col max-w-sm items-center gap-8 text-center">
                            <img src="/public/assets/knowledge.png" alt="lightbulb Icon" class="h-24 sm:h-36" />
                            <h4>{"illuminate"}</h4>
                            <p class="align-top m-0">{{crate::consts::WEBSITE_ILLUMINATE}}</p>
                        </div>
                        <h5 class="text-5xl sm:text-6xl md:text-7xl lg:text-8xl font-bold text-nostr-light">{"+"}</h5>
                        <div class="flex flex-col max-w-sm items-center gap-8 text-center xl:-translate-y-4">
                            <img src="/public/assets/nodes.png" alt="Nodes Icon" class="h-24 sm:h-36" />
                            <h4>{"nodes"}</h4>
                            <p class="align-top m-0">{{crate::consts::WEBSITE_NODES}}</p>
                        </div>
                    </div>
                    <p>{{crate::consts::WEBSITE_FOUNDERS}}</p>
                </div>
            </div>
        </div>
        <div
            class="flex flex-col sm:flex-row items-center justify-center gap-16 z-30 text-center  mt-4 mx-16 sm:mt-8 sm:mx-32 md:mt-16 md:mx-48">
            <div
                class="bg-nostr-dark rounded-3xl p-8 aspect-square flex flex-col text-right gap-4 w-64 lg:w-96 items-center justify-center sm:mb-24">
                <h5 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-wrap font-nostr-bold">{"Our Vision"}</h5>
                <p class="text-base lg:text-lg text-wrap">{{crate::consts::WEBSITE_VISION}}</p>
            </div>
            <div
                class="bg-nostr-dark rounded-3xl p-8 aspect-square flex flex-col text-left gap-4 w-64 md:w-80 lg:w-96 items-center justify-center sm:mt-24">
                <h5 class="text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-wrap font-nostr-bold">{"Our Mission"}</h5>
                <p class="text-base lg:text-lg text-wrap">{{crate::consts::WEBSITE_MISSION}}</p>
            </div>
        </div>
        <div class="flex flex-col items-center gap-8 z-20 text-center  mt-4 mx-16 sm:mt-8 sm:mx-32 md:mt-16 md:mx-48">
            <div class="flex flex-col items-center gap-8 text-center relative">
                <img src="/public/assets/purple-background.png" alt="Purple Background"
                    class="w-full sm:scale-75 z-10 absolute sm:-translate-y-96" />
                <div class="flex flex-col items-center gap-8 text-center z-20">
                    <h2>{"Our Philosophy"}</h2>
                    <div class="flex flex-col md:flex-row  justify-between items-center gap-12">
                        <div class="flex flex-col gap-4">
                            <p class="text-center md:text-left">{{crate::consts::WEBSITE_PHILOSOPHY_1}}</p>
                            <p class="text-center md:text-left">{{crate::consts::WEBSITE_PHILOSOPHY_2}}</p>
                        </div>
                        <img src="/public/assets/integrity.png" alt="Nodes Icon" class="sm:w-1/2 scale-75" />
                    </div>
                </div>
            </div>
        </div>
        <div class="flex flex-col items-center gap-8 z-20 text-center  mt-4 mx-16 sm:mt-8 sm:mx-32 md:mt-16 md:mx-48">
            <h3 id="technology">{"Technology"}</h3>
            <h2>{"Our Technology"}</h2>
            <div class="flex flex-col md:flex-row items-center text-center">
                <div class="flex flex-col gap-4 flex-1 m-4">
                    <p class="text-center md:text-left">
                        {{crate::consts::WEBSITE_TECH_1}}
                    </p>
                    <p class="text-center md:text-left">
                        {{crate::consts::WEBSITE_TECH_2}}
                    </p>
                </div>
                <div class="relative sm:w-1/2">
                    <img src="/public/assets/purple-background.png" alt="Nodes Icon" class="w-full absolute" />
                    <img src="/public/assets/illumGray.png" alt="Nodes Icon" class="w-full scale-50" />
                </div>
            </div>
        </div>
        </>
    }
}
