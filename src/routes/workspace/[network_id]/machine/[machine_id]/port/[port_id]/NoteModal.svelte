<script lang="ts">
    export let show = false;
    export let onClose: () => void;
    export let onSave: (note: any) => void;

    let selectedStage = "Enumeration";
    let noteContent = "";

    const stages = [
        "Enumeration",
        "Information Gathering",
        "Vulnerability Analysis",
        "Exploitation",
        "Post-Exploitation",
        "Privilege Escalation",
        "Lateral Movement",
        "Persistence",
        "General Notes"
    ];

    function handleSubmit() {
        const note = {
            PentestNote: {
                stage: selectedStage,
                content: noteContent,
                timestamp: new Date().toISOString()
            }
        };
        
        onSave(note);
        noteContent = "";
        onClose();
    }
</script>

<div class="modal" class:show>
    <div class="modal-content">
        <h2>Add Note</h2>
        <form on:submit|preventDefault={handleSubmit}>
            <div class="form-group">
                <label for="stage">Stage:</label>
                <select id="stage" bind:value={selectedStage}>
                    {#each stages as stage}
                        <option value={stage}>{stage}</option>
                    {/each}
                </select>
            </div>
            <div class="form-group">
                <label for="content">Note:</label>
                <textarea
                    id="content"
                    bind:value={noteContent}
                    rows="4"
                    required
                ></textarea>
            </div>
            <div class="button-group">
                <button type="button" on:click={onClose}>Cancel</button>
                <button type="submit">Save</button>
            </div>
        </form>
    </div>
</div>

<style>
    .modal {
        display: none;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 1000;
    }

    .modal.show {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .modal-content {
        background: white;
        padding: 2rem;
        border-radius: 1rem;
        width: 90%;
        max-width: 500px;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
    }

    select, textarea {
        width: 100%;
        padding: 0.5rem;
        border: 2px solid black;
        border-radius: 0.5rem;
        font-family: inherit;
    }

    .button-group {
        display: flex;
        gap: 1rem;
        justify-content: flex-end;
    }

    button {
        padding: 0.5rem 1rem;
        border: 2px solid black;
        border-radius: 0.5rem;
        background: white;
        cursor: pointer;
    }

    button[type="submit"] {
        background: black;
        color: white;
    }
</style>
