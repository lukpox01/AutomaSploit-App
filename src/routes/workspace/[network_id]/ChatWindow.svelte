<script lang="ts">
    import { MessageCircle, X, Send, ChevronDown } from 'lucide-svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { page } from '$app/stores';
    export let show = false;
    export let onClose: () => void;
    export let workspace: any;

    let messages: Array<{type: 'user' | 'ai', content: string}> = [];
    let currentMessage = '';
    let contextType = 'workspace';
    let machines = [];
    let selectedMachine: null = null;
    let selectedPort = null;
    let ports = [];

    async function loadMachines() {
        if (!workspace?.id) return;
        const machinesJson = await invoke('machines', { 
            workspaceId: workspace.id 
        });
        machines = JSON.parse(machinesJson);
    }

    async function loadPorts() {
        if (!selectedMachine) return;
        const portsJson = await invoke('ports', {
            workspaceId: workspace.id,
            machineId: selectedMachine
        });
        ports = JSON.parse(portsJson as string);
    }

    $: if (workspace?.id) {
        loadMachines();
    }

    $: if (selectedMachine) {
        loadPorts();
    }

    async function sendMessage() {
        if (!currentMessage.trim()) return;
        
        messages = [...messages, { type: 'user', content: currentMessage }];
        const userQuery = currentMessage;
        currentMessage = '';

        try {
            const contextData = {
                type_: contextType, // Changed from type to type_
                workspace_id: workspace.id,
                machine_id: selectedMachine,
                port_number: selectedPort
            };

            const response = await invoke('ask_question', {
                question: userQuery,
                context: contextData
            }) as string;

            const cleanResponse = response.replace(/<think>.*?<\/think>/gs, '').trim();
            
            messages = [...messages, {
                type: 'ai',
                content: cleanResponse
            }];
        } catch (error) {
            console.error('Error getting response:', error);
            messages = [...messages, {
                type: 'ai',
                content: 'Sorry, I encountered an error while processing your request.'
            }];
        }
    }

    function handleContextChange(event: Event) {
        contextType = (event.target as HTMLSelectElement).value;
        selectedMachine = null;
        selectedPort = null;
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        }
    }
</script>

<style>
    .chat-window {
        position: fixed;
        bottom: 5rem;
        right: 1rem;
        width: 400px;
        height: 500px;
        background-color: white;
        border: 2px solid black;
        border-radius: 1rem;
        display: flex;
        flex-direction: column;
        z-index: 1000;
    }

    .chat-header {
        padding: 1rem;
        border-bottom: 2px solid black;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .chat-header h3 {
        margin: 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .close-btn {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
    }

    .messages {
        flex: 1;
        overflow-y: auto;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .message {
        padding: 0.5rem 1rem;
        border-radius: 1rem;
        max-width: 80%;
    }

    .user-message {
        background-color: black;
        color: white;
        align-self: flex-end;
    }

    .ai-message {
        background-color: #f0f0f0;
        align-self: flex-start;
    }

    .input-area {
        padding: 1rem;
        border-top: 2px solid black;
        display: flex;
        gap: 0.5rem;
    }

    .message-input {
        flex: 1;
        padding: 0.5rem;
        border: 2px solid black;
        border-radius: 0.5rem;
        resize: none;
        font-family: inherit;
    }

    .send-btn {
        background: black;
        color: white;
        border: none;
        border-radius: 0.5rem;
        padding: 0.5rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .send-btn:hover {
        background: #333;
    }

    .context-selector {
        padding: 0.5rem;
        margin: 0.5rem;
        border: 1px solid black;
        border-radius: 0.5rem;
        display: flex;
        gap: 0.5rem;
        align-items: center;
        flex-wrap: wrap;
    }

    .context-select {
        padding: 0.3rem;
        border: 1px solid black;
        border-radius: 0.3rem;
        min-width: 120px;
    }

    .context-badge {
        background-color: #f0f0f0;
        padding: 0.2rem 0.5rem;
        border-radius: 0.3rem;
        font-size: 0.9em;
    }
</style>

{#if show}
    <div class="chat-window">
        <div class="chat-header">
            <h3>
                <MessageCircle size={20} />
                Workspace Assistant
            </h3>
            <button class="close-btn" on:click={onClose}>
                <X size={24} />
            </button>
        </div>

        <div class="context-selector">
            <select class="context-select" on:change={handleContextChange}>
                <option value="workspace">Entire Workspace</option>
                <option value="machine">Single Machine</option>
                <option value="port">Single Port</option>
            </select>

            {#if contextType === 'machine' || contextType === 'port'}
                <select 
                    class="context-select"
                    bind:value={selectedMachine}
                >
                    <option value={null}>Select Machine</option>
                    {#each machines as machine}
                        <option value={machine.id}>{machine.hostname}</option>
                    {/each}
                </select>
            {/if}

            {#if contextType === 'port' && selectedMachine}
                <select 
                    class="context-select"
                    bind:value={selectedPort}
                >
                    <option value={null}>Select Port</option>
                    {#each ports as port}
                        <option value={port.number}>{port.number} ({port.service})</option>
                    {/each}
                </select>
            {/if}
        </div>

        <div class="messages">
            {#each messages as message}
                <div class="message {message.type === 'user' ? 'user-message' : 'ai-message'}">
                    {message.content}
                </div>
            {/each}
        </div>
        <div class="input-area">
            <textarea
                class="message-input"
                placeholder="Ask about your workspace..."
                bind:value={currentMessage}
                on:keydown={handleKeyDown}
                rows="1"
            ></textarea>
            <button class="send-btn" on:click={sendMessage}>
                <Send size={20} />
            </button>
        </div>
    </div>
{/if}
