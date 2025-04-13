// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { mdsvex } from "mdsvex";

/**
 * @typedef {'mdsvex'} NewType
 */

/** @type {import(NewType).MdsvexOptions} */
const MdsvexOptions = {
  extensions: ["md"],
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess(), mdsvex(MdsvexOptions)],
  kit: {
    adapter: adapter(),
  },
};

export default config;
