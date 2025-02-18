<script>
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api/core';
    import {Settings, Map, Monitor, FileWarning, icons, MessageCircle} from 'lucide-svelte';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import ChatWindow from './ChatWindow.svelte';

    /**
     * @type {any[]}
     */
    let machines = [];
    let scanning = false;
    let showChat = false;
    let currentWorkspace = {};

    onMount(() => {
        const unsubscribe = page.subscribe(async ($page) => {
            try {
                const network_id_str = $page.params.network_id;
                const network_id = parseInt(network_id_str);
                console.log(network_id);
                const databaseJson = await invoke('machines', { workspaceId: network_id });
                machines = JSON.parse(databaseJson);
                console.log(machines);
                const workspaceJson = await invoke('get_workspace', { 
                    workspaceId: parseInt($page.params.network_id) 
                });
                currentWorkspace = JSON.parse(workspaceJson);
            } catch (error) {
                console.error("Error fetching workspace:", error);
            }
        });

        return () => {
            unsubscribe();
        };
    });

    async function scanNetwork() {
        try {
            scanning = true;
            const workspaceId = parseInt($page.params.network_id);
            console.log(`Starting network scan for workspace: ${workspaceId}`);
            console.log("Initiating discovery process...");
            
            const result = await invoke('discover_hosts', { workspaceId });
            console.log("Network scan result:", result);
            
            console.log("Refreshing machine list...");
            const databaseJson = await invoke('machines', { workspaceId });
            machines = JSON.parse(databaseJson);
            console.log(`Updated machine list. Found ${machines.length} machines:`, machines);
        } catch (error) {
            console.error("Network scan failed:", error);
        } finally {
            scanning = false;
            console.log("Network scan process completed");
        }
    }
</script>

<style>
    * {
        box-sizing: border-box;
        padding: 0;
        margin: 0;
        background-color: #f5faff;
    }
    :root{
        background-color: #f5faff;
    }
    .layout {
        position: fixed;
        bottom: 1rem;
        top: 1rem;
        left: 1rem;
        right: 1rem;
        display: flex;
        background-color: #f5faff;
        gap: 1rem; /* Add gap between sidebar and content */
    }
    .sidebar {
        height: 100%;
        padding: 1rem;
        border: black 1px solid;
        border-radius: 1rem;
        width: 17rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        overflow-y: auto;
    }
    .sidebar ul {
        list-style-type: none;
        padding: 0;
    }

    .sidebar ul li  {
        margin-bottom:0.3rem;
        border-radius: 1rem;
    }
    .sidebar ul li a{
        text-decoration: none;
        color: black;
        font-size: larger;
        display: flex;
        align-items: center;
        padding: 1rem;
        border-radius: 1rem;
    }
    .sidebar ul li:hover *{
        background-color: black;
        color: white;
    }
    .content {
        flex: 1;
        padding: 1rem;
        overflow-y: auto; /* Allow content to scroll */
        margin-right: 1rem; /* Add some space for potential scrollbar */
        border: 1px solid #e5e7eb; /* Optional: adds a subtle border */
        border-radius: 1rem; /* Optional: matches sidebar border radius */
    }
    .logo {
        font-size: 1.5rem;
        font-weight: bold;
        margin-bottom: 0.5rem;
        font-size: xx-large;
        margin-left: 2rem;
    }
    hr {
        margin-top: 1rem;
        margin-bottom: 1rem;
    }
    
    .bottom-links {
        margin-top: auto;
    }
    .icon-align {
        vertical-align: middle;
        padding-right: 1rem;
        position: relative;
        top: 5px;
    }
    .newscan-btn {
        background-color: black;
        height: 4rem ;
        color: white;
        font-size: xx-large;
        border-radius: 1rem;
        border: white 3px solid;
        margin-bottom: 2rem;
        cursor: pointer;
    }

    .newscan-btn:hover {
        background-color: transparent;
        color: black;
        border: black 3px solid;
    }
    .scan-network-btn {
        background-color: transparent;
        height: 4rem;
        color: black;
        font-size: large;
        border-radius: 1rem;
        border: black 3px solid;
        margin-bottom: 2rem;
        cursor: pointer;
    }

    .scan-network-btn:hover {
        background-color: black;
        color: white;
        border: white 3px solid;
    }
    .scan-network-btn[disabled] {
        opacity: 0.5;
        cursor: not-allowed;
    }
    .chat-btn {
        position: fixed;
        bottom: 1rem;
        left: 19rem;  /* Changed from right: 1rem to left: 19rem to position it next to sidebar */
        background-color: black;
        color: white;
        border: none;
        border-radius: 50%;
        width: 3.5rem;
        height: 3.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        z-index: 1000;
    }

    .chat-btn:hover {
        background-color: #333;
    }

    /* Make scrollbars look better */
    .sidebar::-webkit-scrollbar,
    .content::-webkit-scrollbar {
        width: 8px;
    }

    .sidebar::-webkit-scrollbar-track,
    .content::-webkit-scrollbar-track {
        background: #f1f1f1;
        border-radius: 4px;
    }

    .sidebar::-webkit-scrollbar-thumb,
    .content::-webkit-scrollbar-thumb {
        background: #888;
        border-radius: 4px;
    }

    .sidebar::-webkit-scrollbar-thumb:hover,
    .content::-webkit-scrollbar-thumb:hover {
        background: #555;
    }
</style>

<div class="layout">
    <nav class="sidebar">
        <h1 class="logo">NetVision</h1>
        <button class="newscan-btn" on:click={() => goto(`/workspace/${$page.params.network_id}/newscan`)}>+</button>
        <button class="scan-network-btn" 
                on:click={scanNetwork} 
                disabled={scanning}>
            {scanning ? 'Scanning...' : 'Scan Network'}
        </button>
        <h3>Machines</h3>
        <hr/>
        <ul>

            {#each machines as machine}
                {#if machine.icon == "PC"}
                    <li><a href="/workspace/{$page.params.network_id}/machine/{machine.id}"><span class="icon-align"><Monitor size="24" /></span>{machine.hostname}</a></li>
                {:else}
                    <li><a href="/workspace/{$page.params.network_id}/machine/{machine.id}"><span class="icon-align"><FileWarning size="24" /></span>{machine.hostname}</a></li>
                {/if}
            {/each}

        </ul>
        <ul class="bottom-links">
            
            <li><a href="/settings"><span class="icon-align"><Settings size="24" /></span>Settings</a></li>
            <li><a href="/roadmap"><span class="icon-align"><Map size="24"/></span>Roadmap</a></li>
        </ul>
    </nav>
    <div class="content">
        <slot></slot>
    </div>
</div>

<button class="chat-btn" on:click={() => showChat = !showChat}>
    <MessageCircle size={24} />
</button>

<ChatWindow 
    show={showChat}
    onClose={() => showChat = false}
    workspace={currentWorkspace}
/>