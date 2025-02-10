<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { CheckCircle, XCircle, AlertTriangle, Activity, CheckCircle2, XOctagon } from 'lucide-svelte';
    import { goto } from '$app/navigation';

    let workspaces: any[] = [];
    let toolStatus: {
        rustscan: boolean;
        nmap: boolean;
        ollama: boolean;
    } | null = null;
    let loading = true;

    onMount(async () => {
        try {
            const [workspacesJson, toolsJson] = await Promise.all([
                invoke('workspaces'),
                invoke('check_tools')
            ]);

            workspaces = JSON.parse(workspacesJson as string);
            toolStatus = JSON.parse(toolsJson as string);
        } catch (error) {
            console.error('Error fetching data:', error);
        } finally {
            loading = false;
        }
    });

    const allToolsInstalled = () => {
        return toolStatus && Object.values(toolStatus).every(status => status);
    };

    function getServiceStatus(service: string, status: boolean) {
        if (status) {
            return 'Running';
        }
        return 'Not installed';
    }

    function getServiceIcon(status: boolean) {
        if (status) {
            return CheckCircle2;
        }
        return XOctagon;
    }
</script>

<main>
    <div class="header">
        <h1>Workspaces</h1>
        {#if toolStatus}
            <div class="tools-status">
                <div class="services-list">
                    {#each Object.entries(toolStatus) as [service, status]}
                        <div class="service-item {status ? 'running' : 'stopped'}">
                            <svelte:component 
                                this={getServiceIcon(status)} 
                                size={16}
                            />
                            <span class="service-name">{service}</span>
                            <span class="service-status">{getServiceStatus(service, status)}</span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>

    {#if loading}
        <p>Loading...</p>
    {:else if workspaces.length === 0}
        <div class="empty-state">
            <p>No workspaces found</p>
            <button on:click={() => goto('/workspace/create/name')}>Create Workspace</button>
        </div>
    {:else}
        <div class="workspace-grid">
            {#each workspaces as workspace}
                <a href="/workspace/{workspace.id}" class="workspace-card">
                    <h2>{workspace.name}</h2>
                    <p>{workspace.ip_range}</p>
                </a>
            {/each}
            <button class="new-workspace-card" on:click={() => goto('/workspace/new')}>
                <span class="plus">+</span>
                <span>New Workspace</span>
            </button>
        </div>
    {/if}
</main>

<style>
    main {
        padding: 2rem;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .tools-status {
        display: flex;
        gap: 1rem;
        min-width: 200px;
    }

    .status-badge {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        font-size: 0.9rem;
    }

    .status-badge.success {
        background-color: #dcfce7;
        color: #166534;
    }

    .status-badge.warning {
        background-color: #fef9c3;
        color: #854d0e;
    }

    .workspace-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 1rem;
    }

    .workspace-card, .new-workspace-card {
        padding: 1.5rem;
        border: 2px solid black;
        border-radius: 1rem;
        text-decoration: none;
        color: inherit;
        transition: all 0.2s;
    }

    .workspace-card:hover {
        background-color: black;
        color: white;
    }

    .new-workspace-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        background: transparent;
    }

    .new-workspace-card:hover {
        background-color: black;
        color: white;
    }

    .plus {
        font-size: 2rem;
        font-weight: bold;
        margin-bottom: 0.5rem;
    }

    .empty-state {
        text-align: center;
        margin: 3rem 0;
    }

    .empty-state button {
        margin-top: 1rem;
        padding: 0.75rem 1.5rem;
        background-color: black;
        color: white;
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
    }

    .services-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        padding: 0.5rem;
        border: 1px solid #e5e7eb;
        border-radius: 0.5rem;
        background-color: white;
    }

    .service-item {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.875rem;
    }

    .service-item.running {
        color: #059669;
        background-color: #ecfdf5;
    }

    .service-item.stopped {
        color: #dc2626;
        background-color: #fef2f2;
    }

    .service-name {
        text-transform: capitalize;
        font-weight: 500;
        min-width: 80px;
    }

    .service-status {
        color: #6b7280;
        font-size: 0.75rem;
    }
</style>