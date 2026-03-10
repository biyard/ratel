const path = require("path");
const webpack = require("webpack");

module.exports = {
  entry: "./src/index.js",
  output: {
    filename: "main.js",
    path: path.resolve(__dirname, "dist"),
  },
  optimization: {
    splitChunks: false,
    runtimeChunk: false,
  },
  // FIXME: Forced single chunk to simplify deployment.
  // Bundle size around ~2.5MB — consider code splitting.
  plugins: [
    new webpack.optimize.LimitChunkCountPlugin({
      maxChunks: 1,
    }),
  ],
};
