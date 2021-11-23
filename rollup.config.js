import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";
import common from "@rollup/plugin-commonjs";

const isWatch = !!process.env.ROLLUP_WATCH;

export default [
  {
    input: {
      dominator: "./Cargo.toml",
    },
    output: {
      name: "dominator",
      dir: "dist/js",
      format: "iife",
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
  },
  {
    input: {
      components: "stories/jig-label-ui/jig-label-ui.ts",
    },
    output: {
      name: "components",
      dir: "dist/js",
      format: "iife",
    },
    plugins: [resolve(), common(), typescript()],
  },
];
