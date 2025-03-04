<script>
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';

  export const theme = writable('light');

  onMount(() => {
    // Load theme from local storage on component mount
    const storedTheme = localStorage.getItem('theme') || 'light';
    theme.set(storedTheme);

    // Update data-theme attribute when the theme changes
    theme.subscribe(value => {
      document.documentElement.setAttribute('data-theme', value);
      localStorage.setItem('theme', value);
    });
  });
</script>

<slot />
