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
  theme: {
    extend: {
      fontFamily: {
        festive: ["Festive"]
      }
    },
  },
  plugins: [],
}
