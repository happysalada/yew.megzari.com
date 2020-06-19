// postcss.config.js
const purgecss = require("@fullhuman/postcss-purgecss")({
  // Specify the paths to all of the template files in your project
  content: [
    "./src/**/*.rs",
    // etc.
  ],

  // Include any special characters you're using in this regular expression
  defaultExtractor: (content) => content.match(/[\w-\/:]+(?<!:)/g) || [],
});

module.exports = ({ webpack: { mode } }) => ({
  plugins: [
    require("tailwindcss")("./tailwind.config.js"),
    require("autoprefixer"),
    ...(mode === "production" ? [purgecss] : []),
  ],
});
