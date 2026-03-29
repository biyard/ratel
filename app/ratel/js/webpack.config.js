const path = require("path");

module.exports = {
  entry: "./src/index.js",
  output: {
    filename: "main.js",
    chunkFilename: "ratel-chunk-[name]-[contenthash:8].js",
    path: path.resolve(__dirname, "dist"),
    // Chunks are loaded relative to the main script's location.
    // Dioxus serves assets under /assets/, so chunks must resolve there.
    publicPath: "/assets/",
    clean: true,
  },
  optimization: {
    splitChunks: {
      chunks: "async",
      minSize: 50000,
      maxAsyncRequests: 10,
    },
  },
};
