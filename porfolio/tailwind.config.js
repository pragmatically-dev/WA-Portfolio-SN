module.exports = {
  content: ["./index.html","./src/**/*.{html,js,rs}"],
  theme: {
    extend: {
      extend: {
        fontFamily:{
          'poppins':['poppins']
        }
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    styled: true,
    themes: true,
    base: true,
    utils: true,
    logs: true,
    rtl: false,
    prefix: "",
    darkTheme: "black",
  },
}
