/** @type {import("tailwindcss").Config} */

module.exports = {
  mode: "jit",
  darkMode: "media",
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
    screens: {
      lg: "1175px",
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
