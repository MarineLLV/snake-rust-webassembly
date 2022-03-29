const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "bootstrap.js"
    },
    mode: "development",
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: "./index.html", to: "./" }
            ]
        })
    ]
}
