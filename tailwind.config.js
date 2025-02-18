/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./{source,sources}/**/*.{jinja,html}"],
    },
    theme: {
        extend: {},
    },
    plugins: [],
}