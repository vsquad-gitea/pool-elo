const { default: daisyui } = require("daisyui");

module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.rs",
      "./index.html",
      "./src/**/*.html",
      "./src/**/*.css",
    ],
  },
  theme: {},
  variants: {},
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["light", "dark", "lemonade", "autumn", "nord"],
  },
};
