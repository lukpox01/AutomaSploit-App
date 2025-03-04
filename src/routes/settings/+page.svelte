<script>
	import { onMount } from 'svelte';

	let enableNotifications = false;
	let darkMode = false;
	let selectedTheme = 'light';

	onMount(() => {
		// Load theme from local storage on component mount
		const storedTheme = localStorage.getItem('theme') || 'light';
		selectedTheme = storedTheme;
		document.documentElement.setAttribute('data-theme', selectedTheme);
	});

	function saveSettings() {
		// Save settings logic here
		console.log('Settings saved:', { enableNotifications, darkMode });
	}

	function updateTheme() {
		localStorage.setItem('theme', selectedTheme);
		document.documentElement.setAttribute('data-theme', selectedTheme);
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
		<label>
			<input type="checkbox" bind:checked={darkMode} />
			Dark Mode
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
