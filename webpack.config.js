const path = require("path");
const distPath = path.resolve(__dirname, "dist");

const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = (env, argv) => {
  return {
    performance: {
      // Don't break compilation because of WASM file bigger than 244 KB.
      hints: false,
    },
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000,
      historyApiFallback: true,
    },
    entry: "./bootstrap.js",
    output: {
      path: distPath,
      filename: "[name].[contenthash].js",
      // webassemblyModuleFilename: "todomvc.wasm"
    },
    plugins: [
      new CleanWebpackPlugin(),
      // Extract CSS styles into a file.
      new MiniCssExtractPlugin({
        filename: "[name].[contenthash].css",
      }),
      // Add scripts, css, ... to html template.
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "./static/index.html"),
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
      new CopyWebpackPlugin({ patterns: [{ from: "./static", to: distPath }] }),
    ],
    // Webpack try to guess how to resolve imports in this order:
    resolve: {
      extensions: [".js", ".wasm"],
      alias: {
        crate: path.resolve(__dirname, "../crate"),
      },
    },
    module: {
      rules: [
        {
          test: /\.(jpg|jpeg|png|woff|woff2|eot|ttf|svg)$/,
          use: [
            {
              loader: "file-loader",
              options: {
                // Don't copy files to `dist`, we do it through `CopyWebpackPlugin` (see above)
                // - we only want to resolve urls to these files.
                emitFile: false,
                name: "[path][name].[ext]",
              },
            },
          ],
        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            { loader: "css-loader", options: { importLoaders: 1 } },
            {
              loader: "postcss-loader",
              options: {
                config: {
                  // Path to postcss.config.js.
                  path: __dirname,
                },
              },
            },
          ],
        },
      ],
    },
    watch: argv.mode !== "production",
  };
};
