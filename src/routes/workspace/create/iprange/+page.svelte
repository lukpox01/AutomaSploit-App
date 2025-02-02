<script>
    import { invoke } from '@tauri-apps/api/core';
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let workspaceName = "";
    let ipRange = "";

    onMount(() => {
        const urlParams = new URLSearchParams(window.location.search);
        workspaceName = urlParams.get('name') || "";
    });

    async function createWorkspace() {
        try {
            const response = await invoke('add_workspace', { name: workspaceName, ipRange });
            console.log(response);
            goto('/');
        } catch (error) {
            console.error("Failed to create workspace:", error);
        }
    }
</script>

<style>
    .text {
        font-size: 5vw;
        margin: 1rem;
    }

    .input {
        font-size: 5vw;
        padding: 1rem;
        font-size: 1.5rem;
        border: none;
        border-bottom: 1px solid black;
        border-radius: 0;
        outline: none;
        text-align: center;
        background-color: transparent;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }

    .next-button {
        position: absolute;
        bottom: 1rem;
        right: 1rem;
        padding: 1rem 2rem;
        font-size: 1.5rem;
        background-color: #007BFF;
        color: white;
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        text-decoration: none;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>

<main class="container">
    <h1 class="text">
        Ip Range
    </h1>
    <input type="text" class="input" placeholder="Optional IP Range" bind:value={ipRange} maxlength="18" />
    <button class="next-button" on:click={createWorkspace}>Create</button>
</main>