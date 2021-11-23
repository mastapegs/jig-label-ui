import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";

const isWatch = !!process.env.ROLLUP_WATCH;

export default {
  input: {
    index: "./Cargo.toml",
  },
  output: {
    dir: "dist/js",
    format: "iife",
    sourcemap: true,
  },
  plugins: [
    rust({
      serverPath: "js/",
    }),

    isWatch &&
      serve({
        contentBase: "dist",
        open: true,
      }),

    isWatch && livereload("dist"),

    !isWatch && terser(),
  ],
};
