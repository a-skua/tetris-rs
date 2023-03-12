import * as esbuild from "https://deno.land/x/esbuild@v0.17.11/mod.js";

await esbuild.build({
  entryPoints: [
    "./main.ts",
  ],
  outfile: "../../docs/main.js",
  format: "esm",
  bundle: true,
});

esbuild.stop();
