<script lang="ts">
    export let data: any[];

    function formatCredential(cred: any) {
        const name = cred.name || 'unknown';
        const hash = cred.hash || '';
        const password = cred.password ? cred.password : '';
        return { name, hash, password };
    }

    function groupNotesByType(notes: any[]) {
        const grouped = {
            nmap: [] as string[],
            credentials: [] as any[]
        };

        notes.forEach(note => {
            if (note.NmapScan) {
                grouped.nmap.push(...note.NmapScan);
            } else if (note.Credentials) {
                grouped.credentials.push(formatCredential(note.Credentials));
            }
        });

        return grouped;
    }

    $: groupedNotes = groupNotesByType(data || []);
</script>

<style>
    .content {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        padding: 1rem;
    }

    .note-section {
        background-color: rgb(239, 239, 239);
        border-radius: 1rem;
        padding: 1rem;
    }

    .note-header {
        font-size: larger;
        font-weight: bold;
        padding: 0.5rem 1rem;
        border-bottom: 2px solid #ddd;
        margin-bottom: 1rem;
    }

    .details-list {
        list-style-type: none;
        padding: 0.5rem 1rem;
    }

    .details-list li {
        margin-bottom: 0.5rem;
        font-family: monospace;
    }

    .credential-entry {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem;
        border-bottom: 1px solid #ddd;
    }

    .credential-entry:last-child {
        border-bottom: none;
    }

    .cred-left {
        color: #0066cc;
        font-weight: bold;
    }

    .cred-right {
        color: #009900;
        font-weight: bold;
    }

    .hash {
        color: #666;
        margin: 0 0.5rem;
    }

    .no-data {
        text-align: center;
        color: #666;
        font-style: italic;
    }
</style>

<div class="content">
    {#if groupedNotes.nmap.length > 0}
        <div class="note-section">
            <div class="note-header">Nmap Scan Details</div>
            <ul class="details-list">
                {#each groupedNotes.nmap as detail}
                    <li>{detail}</li>
                {/each}
            </ul>
        </div>
    {/if}

    {#if groupedNotes.credentials.length > 0}
        <div class="note-section">
            <div class="note-header">Credentials</div>
            {#each groupedNotes.credentials as cred}
                <div class="credential-entry">
                    <span class="cred-left">{cred.name}</span>
                    {#if cred.hash}
                        <span class="hash">{cred.hash}</span>
                    {/if}
                    {#if cred.password}
                        <span class="cred-right">{cred.password}</span>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}

    {#if !groupedNotes.nmap.length && !groupedNotes.credentials.length}
        <div class="note-section">
            <div class="no-data">No notes available</div>
        </div>
    {/if}
</div>