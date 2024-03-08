/** @type {import("tailwindcss").Config} */
const colors = require("tailwindcss/colors");

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
    colors: {
      light: colors.stone,
      accentlight: colors.amber,
      dark: colors.gray,
      accentdark: colors.blue,
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
