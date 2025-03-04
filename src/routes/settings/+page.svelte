<script>
  import { theme } from '../+layout.svelte';

  let enableNotifications = false;
  let selectedTheme;

  theme.subscribe(value => {
    selectedTheme = value;
  });

  function saveSettings() {
    // Save settings logic here
    console.log('Settings saved:', { enableNotifications });
  }

  function updateTheme() {
    theme.set(selectedTheme);
  }
</script>

<h1>Settings</h1>
<form on:submit|preventDefault={saveSettings}>
  <div>
    <label>
      <input type="checkbox" bind:checked={enableNotifications} />
      Enable Notifications
    </label>
  </div>

  <div>
    <label for="theme">Theme:</label>
    <select id="theme" bind:value={selectedTheme} on:change={updateTheme}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
    </select>
  </div>

  <button type="submit">Save</button>
</form>
