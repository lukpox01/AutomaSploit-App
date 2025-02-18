<script lang="ts">
    import CredentialsTable from './CredentialsTable.svelte';
    import { invoke } from "@tauri-apps/api/core";
    import SvelteMarkdown from 'svelte-markdown';
    export let data: Array<any> = [];
    export let details: Array<string> = [];
    export let workspaceId: number;
    export let machineId: number;
    export let portNumber: number;

    // Filter credentials and notes
    $: credentials = data.filter(note => 'Credentials' in note);
    $: pentestNotes = data.filter(note => note.PentestNote);
    $: nmapNotes = data.filter(note => 'NmapScan' in note);

    async function handleBlur(index, event) {
        const newContent = event.target.innerText;
        // Find the actual index in the full data array
        const noteIndex = data.findIndex(note => 
            note.PentestNote && note.PentestNote.content === pentestNotes[index].PentestNote.content
        );
        
        if (newContent !== pentestNotes[index].PentestNote.content) {
            try {
                await invoke('update_note_content', {
                    workspaceId,
                    machineId,
                    portNumber,
                    noteIndex,
                    newContent
                });
                // Don't modify the local state - let the parent component refresh the data
            } catch (error) {
                console.error(error);
                // Revert the content if there's an error
                event.target.innerText = pentestNotes[index].PentestNote.content;
                alert("Error updating note: " + error);
            }
        }
    }

    function formatTimestamp(timestamp: string) {
        return new Date(timestamp).toLocaleString();
    }

    function processNmapNote(note) {
        if (note.NmapScan[0] === "AI Security Analysis") {
            let content = note.NmapScan[1];
            try {
                // Try to parse JSON response
                const parsed = JSON.parse(content);
                content = parsed.text || content;
            } catch (e) {
                // Not JSON, use as is
                console.log('Not JSON format, using raw content');
            }
            return {
                title: "AI Analysis",
                content: content
            };
        }
        // Regular Nmap scan results
        return {
            title: "Scan Results",
            content: note.NmapScan.join('\n')
        };
    }
</script>

<div class="notes-container">
    {#if data && data.length > 0}
        {#if credentials.length > 0}
            <CredentialsTable {credentials} />
        {/if}

        <!-- Pentest Notes -->
        {#each pentestNotes as note, index}
            <div class="note pentest-note">
                <div class="note-header">
                    <h3>{note.PentestNote.stage}</h3>
                    <span class="timestamp">{formatTimestamp(note.PentestNote.timestamp)}</span>
                </div>
                <!-- Editable note content; blur will trigger a save -->
                <p contenteditable="true" on:blur={(e) => handleBlur(index, e)} class="note-content">
                    {note.PentestNote.content}
                </p>
            </div>
        {/each}

        <!-- Nmap and AI Analysis Results -->
        {#each nmapNotes as note}
            {@const processed = processNmapNote(note)}
            <div class="note nmap-note">
                <h3>{processed.title}</h3>
                <div class="note-content">
                    {#if processed.title === "AI Security Analysis"}
                        <svelte:component this={SvelteMarkdown} source={processed.content} />
                    {:else}
                        {#each processed.content.split('\n') as line}
                            <p>{line}</p>
                        {/each}
                    {/if}
                </div>
            </div>
        {/each}
    {:else}
        <p class="no-notes">No notes available</p>
    {/if}
</div>

<style>
    .notes-container {
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .note {
        padding: 1rem;
        border-radius: 0.5rem;
        border: 1px solid #e5e7eb;
    }

    .pentest-note {
        background-color: #f0fdf4;
        border-left: 4px solid #16a34a;
    }

    .nmap-note {
        background-color: #f1f5f9;
        border-left: 4px solid #475569;
        font-family: monospace;
    }

    .note-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
    }

    .timestamp {
        font-size: 0.875rem;
        color: #6b7280;
    }

    .note-content {
        white-space: pre-wrap;
        outline: none;
        padding: 0.5rem;
        border: 1px dashed transparent;
    }

    .note-content:focus {
        border-color: #16a34a;
        background-color: #fcfdfc;
    }

    .no-notes {
        text-align: center;
        color: #6b7280;
        font-style: italic;
    }

    .note-content :global(p) {
        margin: 0.5rem 0;
    }

    .note-content :global(ul), .note-content :global(ol) {
        margin: 0.5rem 0;
        padding-left: 1.5rem;
    }

    .note-content :global(li) {
        margin: 0.3rem 0;
    }

    .note-content :global(code) {
        background-color: #f1f1f1;
        padding: 0.2rem 0.4rem;
        border-radius: 0.3rem;
        font-family: monospace;
    }
</style>