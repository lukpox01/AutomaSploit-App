import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter(),
    prerender: {
      entries: [
        '/',
        '/roadmap',
        '/settings',
        '/workspace/create/name',
        '/workspace/create/icon',
        '/workspace/create/iprange',
        '/workspace/[network_id]',
        '/workspace/[network_id]/machine',
        '/workspace/[network_id]/machine/[machine_id]',
        '/workspace/[network_id]/machine/[machine_id]/port',
        '/workspace/[network_id]/machine/[machine_id]/port/[port_id]',
        '/workspace/[network_id]/newscan'
      ],
      handleHttpError: ({ path, referrer, message }) => {
        // Ignore certain dynamic routes during prerendering
        if (path.includes('[network_id]') || 
            path.includes('[machine_id]') || 
            path.includes('[port_id]')) {
          return;
        }
        // Otherwise, throw an error
        throw new Error(message);
      }
    }
  }
};

export default config;
