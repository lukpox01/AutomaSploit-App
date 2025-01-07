import adapter from '@sveltejs/adapter-static'; // Use the static adapter for Tauri


const prerenderEntries = [
  '/', // Prerender the homepage
  '/roadmap',
  '/settings',
  '/workspace/1',
  '/workspace/1/machine/1',
  '/workspace/1/machine/1/port/1',
];

const config = {
  kit: {
    adapter: adapter(),
    prerender: {
      entries: prerenderEntries
    }
  }
};

export default config;
