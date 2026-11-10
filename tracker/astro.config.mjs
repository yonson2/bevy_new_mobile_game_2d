// @ts-check
import { defineConfig } from 'astro/config';

// The build gives HTML and CSS only. No integrations, no client scripts.
export default defineConfig({
  markdown: {
    shikiConfig: { theme: 'github-dark' },
  },
});
