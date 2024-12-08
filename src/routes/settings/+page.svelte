<script>
	let enableNotifications = false;
	let darkMode = false;

	function saveSettings() {
		// Save settings logic here
		console.log('Settings saved:', { enableNotifications, darkMode });
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
	<button type="submit">Save</button>
</form>
