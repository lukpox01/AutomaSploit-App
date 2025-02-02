<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

  /**
     * @type {string | any[]}
     */
  let workspaces = []; 

  onMount(async () => { // Use onMount to load data when the component mounts
    try {
      const databaseJson = await invoke('workspaces');
      workspaces = JSON.parse(databaseJson);
      console.log(workspaces); 
    } catch (error) {
      console.error("Error fetching workspaces:", error);
    }
  });
</script>

<main class="no-scan-message">
  <h1 class="app-name">AutomaSploit App</h1>
  {#if workspaces.length === 0}
    <p>You don't have any workspace created. Please create a new one.</p>
  {:else}
    <div class="workspace-list">
      {#each workspaces as workspace}
        <a href="/workspace/{workspace.id}" class="workspace-button">{workspace.name}</a>
      {/each}
    </div>
  {/if}
  <a class="create" href="/workspace/create/name">Create new workspace</a>
</main>

<style>


  .create {
    border-radius: 1rem;
    border: 3px solid black;
    padding: 1rem;
    text-decoration: none;
    color: black;
  }

  .create:hover {
    background-color: black;
    color: white;
  }

  .no-scan-message {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 98vh;
    font-size: 1.5rem;
    text-align: center;
    color: black;
    flex-direction: column;
    /* gap: 1rem; */
  }

  .app-name {
    font-size: 5rem;
    margin-bottom: 2rem;
  }

  .workspace-list {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .workspace-button {
    background: none;
    border: none;
    color: black;
    text-decoration: none;
    font-size: 2rem;
    margin: 1rem;
    padding: 1rem;
    cursor: pointer;
    border-radius: 5px;
    border: transparent 1px solid;
    text-align: center;
  }

  .workspace-button:hover {
    border: black 1px solid;
  }
</style>