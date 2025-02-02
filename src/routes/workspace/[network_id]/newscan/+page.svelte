<script>
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { goto } from '$app/navigation';

    let machineName = '';
    let machineIP = '';

    async function addMachine() {
        const network_id_str = $page.params.network_id;
        const network_id = parseInt(network_id_str);
        try {
            await invoke('add_machine', { workspaceId: network_id, name: machineName, ip: machineIP });
            goto(`/workspace/${network_id}`);
        } catch (error) {
            console.error("Error adding machine:", error);
        }
    }
</script>

<style>
    .form-wrapper {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
        padding: 1rem;
        font-size: larger;
    }
    .form-container {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 600px;
        padding: 1rem;
        border: 1px solid black;
        border-radius: 1rem;
    }
    .form-container label {
        margin-bottom: 0.5rem;
    }
    .form-container input {
        margin-bottom: 1rem;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 0.5rem;
        height: 2rem;
        font-size: larger;
    }
    .form-container button {
        padding: 1rem;
        border: none;
        border-radius: 0.5rem;
        background-color: black;
        color: white;
        font-size: x-large;
        border: 2px solid white;
    }

    .form-container button:hover {
        background-color: white;
        color: black;
        border: 2px solid black;
    }
</style>

<div class="form-wrapper">
    <div class="form-container">
        <label for="machineName">Machine Name</label>
        <input type="text" id="machineName" bind:value={machineName} />

        <label for="machineIP">Machine IP</label>
        <input type="text" id="machineIP" bind:value={machineIP} />

        <button on:click={addMachine}>Add Machine</button>
    </div>
</div>
