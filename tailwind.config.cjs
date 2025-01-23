/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,rs}", "./index.html"],
    theme: {
        extend: {
            fontFamily: {
                'nostr-regular': ["SF-Pro", "sans-serif"],
                'nostr-bold': ["SF-Pro-Bold", "sans-serif"],
            },
            colors: {
                'nostr-dark': '#4B1862',
                'nostr-light': '#A334D5',
                'nostr-black': '#000000',
                'nostr-white': '#FFFFFF',
            },
            backgroundImage: {
                'img-logo': "url('/public/assets/background-principal.png')",
                'img-fuzzy': "url('/public/assets/purple-background.png')",
            },
        },
    },
    plugins: [require("@tailwindcss/forms")],
}

