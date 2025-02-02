<script lang="ts">
    export let show = false;
    export let onClose: () => void;
    export let onSave: (creds: {name: string, hash: string, password: string}) => void;

    let name = '';
    let hash = '';
    let password = '';

    function handleSave() {
        onSave({ name, hash, password });
        name = '';
        hash = '';
        password = '';
        onClose();
    }
</script>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .modal {
        background-color: white;
        padding: 2rem;
        border-radius: 1rem;
        width: 90%;
        max-width: 500px;
    }

    .modal-header {
        font-size: x-large;
        font-weight: bold;
        margin-bottom: 1rem;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        font-weight: bold;
    }

    input, textarea {
        width: 100%;
        padding: 0.5rem;
        border: 2px solid black;
        border-radius: 0.5rem;
        font-size: large;
    }

    textarea {
        min-height: 100px;
        resize: vertical;
    }

    .buttons {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
        margin-top: 1rem;
    }

    button {
        padding: 0.5rem 1rem;
        border: 2px solid black;
        border-radius: 0.5rem;
        font-size: large;
        cursor: pointer;
    }

    .save {
        background-color: black;
        color: white;
    }

    .save:hover {
        background-color: white;
        color: black;
    }

    .cancel {
        background-color: white;
        color: black;
    }

    .cancel:hover {
        background-color: black;
        color: white;
    }
</style>

{#if show}
    <div class="modal-backdrop">
        <div class="modal">
            <div class="modal-header">Add Credentials</div>
            <div class="form-group">
                <label for="name">Name/Username</label>
                <input type="text" id="name" bind:value={name} placeholder="admin/root/service"/>
            </div>
            <div class="form-group">
                <label for="hash">Hash</label>
                <input type="text" id="hash" bind:value={hash} placeholder="$1$hash..."/>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="text" id="password" bind:value={password} placeholder="Cracked password if available"/>
            </div>
            <div class="buttons">
                <button class="cancel" on:click={onClose}>Cancel</button>
                <button class="save" on:click={handleSave}>Save</button>
            </div>
        </div>
    </div>
{/if}
