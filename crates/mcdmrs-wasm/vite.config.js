import { defineConfig } from "vite";

const workerImportMetaUrlRE =
  /\bnew\s+(?:Worker|SharedWorker)\s*\(\s*(new\s+URL\s*\(\s*('[^']+'|"[^"]+"|`[^`]+`)\s*,\s*import\.meta\.url\s*\))/g;

export default defineConfig({
  root: "www",
  base: "",
  worker: {
    format: "es",
    // https://github.com/vitejs/vite/issues/7015
    // https://github.com/vitejs/vite/issues/14499#issuecomment-1740267849
    // https://github.com/vitejs/vite/issues/7015#issuecomment-1802213572 <- HERO.
    plugins: () => [
      {
        name: "Disable nested workers",
        enforce: "pre",
        transform(code, id) {
          if (
            code.includes("new Worker") &&
            code.includes("new URL") &&
            code.includes("import.meta.url")
          ) {
            return code.replace(
              workerImportMetaUrlRE,
              `((() => { throw new Error('Nested workers are disabled') })()`
            );
          }
        },
      },
    ],
  },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
});
