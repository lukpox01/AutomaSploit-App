<script lang="ts">
    import { MessageCircle, X, Send, ChevronDown } from 'lucide-svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { page } from '$app/stores';
    import SvelteMarkdown from 'svelte-markdown';
    export let show = false;
    export let onClose: () => void;
    export let workspace: any;

    let initialMessage = {
        type: 'ai' as const,
        content: "Hello, I'm your Penetration Testing Assistant. I specialize in security analysis, vulnerability assessment, and exploit development. I'll help you identify security weaknesses and suggest mitigation strategies based on the scan results. What would you like to know about your target?",
        timestamp: formatTimestamp()
    };

    let messages = [initialMessage];
    let currentMessage = '';
    let contextType = 'workspace';
    let machines = [];
    let selectedMachine: null = null;
    let selectedPort = null;
    let ports = [];
    let isLoading = false;

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

    function parseResponse(jsonString: string): string {
        try {
            const response = JSON.parse(jsonString);
            return response.text || response.response || jsonString;
        } catch {
            return jsonString;
        }
    }

    function formatTimestamp(): string {
        const now = new Date();
        return now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    async function sendMessage() {
        if (!currentMessage.trim()) return;
        
        const timestamp = formatTimestamp();
        messages = [...messages, { 
            type: 'user', 
            content: currentMessage,
            timestamp 
        }];
        
        const userQuery = currentMessage;
        currentMessage = '';
        isLoading = true;

        try {
            const contextData = {
                type_: contextType,
                workspace_id: workspace.id,
                machine_id: selectedMachine,
                port_number: selectedPort
            };

            const response = await invoke('ask_question', {
                question: userQuery,
                context: contextData
            }) as string;


            const cleanResponse = parseResponse(response)
                .replace(/<think>[\s\S]*?<\/think>/g, '') // Strip thinking stage
                .trim();
            
            messages = [...messages, {
                type: 'ai',
                content: cleanResponse,
                timestamp: formatTimestamp()
            }];
        } catch (error) {
            console.error('Error getting response:', error);
            messages = [...messages, {
                type: 'ai',
                content: 'Sorry, I encountered an error while processing your request.',
                timestamp: formatTimestamp()
            }];
        } finally {
            isLoading = false;
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
        width: 450px;
        height: 600px;
        background-color: #1a1a1a;
        border: 1px solid #333;
        border-radius: 1rem;
        display: flex;
        flex-direction: column;
        z-index: 1000;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        color: #fff;
    }

    .chat-header {
        padding: 1rem;
        background-color: #2a2a2a;
        border-bottom: 1px solid #333;
        border-radius: 1rem 1rem 0 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .chat-header h3 {
        margin: 0;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #fff;
        font-size: 1.1rem;
    }

    .close-btn {
        background: none;
        border: none;
        color: #fff;
        opacity: 0.7;
        cursor: pointer;
        padding: 0.5rem;
        transition: opacity 0.2s;
    }

    .close-btn:hover {
        opacity: 1;
    }

    .messages {
        flex: 1;
        overflow-y: auto;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        scroll-behavior: smooth;
    }

    .message {
        position: relative;
        padding: 0.8rem 1rem;
        border-radius: 1rem;
        max-width: 85%;
        line-height: 1.4;
    }

    .message-timestamp {
        font-size: 0.7rem;
        opacity: 0.7;
        margin-top: 0.3rem;
    }

    .user-message {
        background-color: #2b5c8c;
        color: white;
        align-self: flex-end;
        border-bottom-right-radius: 0.3rem;
    }

    .ai-message {
        background-color: #2a2a2a;
        color: #fff;
        align-self: flex-start;
        border-bottom-left-radius: 0.3rem;
    }

    .ai-message :global(p) {
        margin: 0.5rem 0;
    }

    .ai-message :global(code) {
        background-color: #1a1a1a;
        padding: 0.2rem 0.4rem;
        border-radius: 0.3rem;
        font-family: monospace;
        font-size: 0.9em;
    }

    .ai-message :global(pre) {
        background-color: #1a1a1a;
        padding: 1rem;
        border-radius: 0.5rem;
        overflow-x: auto;
        margin: 0.5rem 0;
    }

    .input-area {
        padding: 1rem;
        background-color: #2a2a2a;
        border-top: 1px solid #333;
        border-radius: 0 0 1rem 1rem;
        display: flex;
        gap: 0.5rem;
    }

    .message-input {
        flex: 1;
        padding: 0.8rem;
        background-color: #1a1a1a;
        border: 1px solid #333;
        border-radius: 0.5rem;
        resize: none;
        color: #fff;
        font-family: inherit;
        transition: border-color 0.2s;
    }

    .message-input:focus {
        outline: none;
        border-color: #4a4a4a;
    }

    .send-btn {
        background: #2b5c8c;
        color: white;
        border: none;
        border-radius: 0.5rem;
        padding: 0.8rem;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background-color 0.2s;
    }

    .send-btn:hover {
        background: #366ea8;
    }

    .send-btn:disabled {
        background: #1a1a1a;
        cursor: not-allowed;
    }

    .context-selector {
        padding: 0.8rem;
        background-color: #2a2a2a;
        border-bottom: 1px solid #333;
        display: flex;
        gap: 0.5rem;
        align-items: center;
        flex-wrap: wrap;
    }

    .context-select {
        padding: 0.5rem;
        background-color: #1a1a1a;
        border: 1px solid #333;
        border-radius: 0.4rem;
        color: #fff;
        min-width: 120px;
        appearance: none;
        background-image: url("image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23ffffff' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
        background-repeat: no-repeat;
        background-position: right 0.5rem center;
        padding-right: 2rem;
    }

    .context-select:focus {
        outline: none;
        border-color: #4a4a4a;
    }

    .context-select option {
        background-color: #1a1a1a;
        color: #fff;
    }

    /* Make the select arrow white */
    .context-select::-ms-expand {
        display: none;
    }

    /* Darker background when opened */
    .context-select:focus {
        background-color: #222;
    }

    /* Style placeholder option */
    .context-select option[value="null"] {
        color: #666;
    }

    .loading-dots {
        display: flex;
        gap: 0.3rem;
        padding: 1rem;
        align-items: center;
    }

    .dot {
        width: 0.5rem;
        height: 0.5rem;
        background-color: #fff;
        border-radius: 50%;
        opacity: 0.6;
        animation: pulse 1.4s infinite;
    }

    .dot:nth-child(2) { animation-delay: 0.2s; }
    .dot:nth-child(3) { animation-delay: 0.4s; }

    @keyframes pulse {
        0%, 100% { opacity: 0.6; }
        50% { opacity: 0.2; }
    }

    /* Additional markdown styles */
    :global(.ai-message h1) {
        font-size: 1.5em;
        margin: 0.5em 0;
        color: #fff;
    }

    :global(.ai-message h2) {
        font-size: 1.3em;
        margin: 0.4em 0;
        color: #fff;
    }

    :global(.ai-message h3) {
        font-size: 1.1em;
        margin: 0.3em 0;
        color: #fff;
    }

    :global(.ai-message ul, .ai-message ol) {
        margin: 0.5em 0;
        padding-left: 1.5em;
    }

    :global(.ai-message li) {
        margin: 0.2em 0;
    }

    :global(.ai-message code) {
        background-color: #1a1a1a;
        padding: 0.2rem 0.4rem;
        border-radius: 0.3rem;
        font-family: monospace;
        font-size: 0.9em;
    }

    :global(.ai-message pre) {
        background-color: #1a1a1a;
        padding: 1rem;
        border-radius: 0.5rem;
        overflow-x: auto;
        margin: 0.5rem 0;
    }

    :global(.ai-message table) {
        border-collapse: collapse;
        margin: 0.5em 0;
        width: 100%;
    }

    :global(.ai-message th, .ai-message td) {
        border: 1px solid #333;
        padding: 0.4em;
        text-align: left;
    }

    :global(.ai-message th) {
        background-color: #2a2a2a;
    }

    :global(.ai-message blockquote) {
        border-left: 3px solid #2b5c8c;
        margin: 0.5em 0;
        padding-left: 1em;
        color: #aaa;
    }

    :global(.ai-message a) {
        color: #4a9eff;
        text-decoration: none;
    }

    :global(.ai-message a:hover) {
        text-decoration: underline;
    }

    /* Added styles for initial message */
    .ai-message.initial-message {
        background-color: #2d2d2d;
        border-left: 3px solid #2b5c8c;
    }
</style>

{#if show}
    <div class="chat-window">
        <div class="chat-header">
            <h3>
                <MessageCircle size={20} />
                Security Analysis Assistant
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
                <div class="message {message.type === 'user' ? 'user-message' : 'ai-message'} {message === initialMessage ? 'initial-message' : ''}">
                    {#if message.type === 'user'}
                        {message.content}
                    {:else}
                        <SvelteMarkdown source={message.content} />
                    {/if}
                    <div class="message-timestamp">{message.timestamp}</div>
                </div>
            {/each}
            {#if isLoading}
                <div class="message ai-message">
                    <div class="loading-dots">
                        <div class="dot"></div>
                        <div class="dot"></div>
                        <div class="dot"></div>
                    </div>
                </div>
            {/if}
        </div>
        <div class="input-area">
            <textarea
                class="message-input"
                placeholder="Ask about your workspace..."
                bind:value={currentMessage}
                on:keydown={handleKeyDown}
                rows="1"
                disabled={isLoading}
            ></textarea>
            <button class="send-btn" on:click={sendMessage} disabled={isLoading || !currentMessage.trim()}>
                <Send size={20} />
            </button>
        </div>
    </div>
{/if}
