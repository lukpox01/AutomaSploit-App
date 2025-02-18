<script lang="ts">
    export let credentials: Array<any> = [];
</script>

<div class="note">
    <h3>Credentials Found</h3>
    <div class="table-container">
        <table>
            <thead>
                <tr>
                    <th>Username</th>
                    <th>Password</th>
                    <th>Hash</th>
                </tr>
            </thead>
            <tbody>
                {#if credentials.length > 0}
                    {#each credentials as cred}
                        <tr>
                            <td>
                                <span class="username-text">
                                    {cred.Credentials.name || '—'}
                                </span>
                            </td>
                            <td>
                                <span class="password-text">
                                    {cred.Credentials.password || '—'}
                                </span>
                            </td>
                            <td>
                                <span class="hash-text">
                                    {cred.Credentials.hash || '—'}
                                </span>
                            </td>
                        </tr>
                    {/each}
                {:else}
                    <tr>
                        <td colspan="3" class="no-creds">No credentials found</td>
                    </tr>
                {/if}
            </tbody>
        </table>
    </div>
</div>

<style>
    .note {
        padding: 1rem;
        border-radius: 0.5rem;
        background-color: white;
        margin-bottom: 1rem;
        border: 1px solid #e5e7eb;
    }

    h3 {
        margin: 0;
        margin-bottom: 1rem;
        font-size: 1.1rem;
        color: #1f2937;
    }

    .table-container {
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        background: white;
    }

    th, td {
        padding: 0.75rem 1rem;
        text-align: left;
        border-bottom: 1px solid #e5e7eb;
    }

    th {
        font-weight: 600;
        color: #374151;
        font-size: 0.875rem;
    }

    .username-text {
        color: #dc2626;  /* red */
        font-family: monospace;
    }

    .password-text {
        color: #2563eb;  /* blue */
        font-family: monospace;
    }

    .hash-text {
        color: #059669;  /* green */
        font-family: monospace;
        word-break: break-all;
    }

    tr:hover {
        background-color: #f9fafb;
    }

    .no-creds {
        text-align: center;
        color: #6b7280;
        font-style: italic;
        padding: 2rem;
    }
</style>
