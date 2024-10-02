export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],

  plugins: [require('flowbite/plugin')],

  darkMode: 'selector',

  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {"50":"#fdf2f8",
          "100":"#fce7f3",
          "200":"#fbcfe8",
          "300":"#f9a8d4",
          "400":"#f472b6",
          "500":"#ec4899",
          "600":"#ec4899",
          "700":"#ec4899",
          "800":"#9d174d",
          "900":"#831843"}
      }
    }
  }
}