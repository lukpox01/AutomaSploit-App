<script>
// @ts-nocheck
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

    let workspace = {};
  onMount(() => {
    const unsubscribe = page.subscribe(async ($page) => {
      try {
        const network_id_str = $page.params.network_id;
        const network_id = parseInt(network_id_str);
        console.log(network_id);

        const workspaceJson = await invoke("get_workspace", {
          workspaceId: network_id,
        });

        workspace = JSON.parse(workspaceJson);

        console.log(workspace);
      } catch (error) {
        console.error("Error fetching ports:", error);
      }
    });

    return () => {
      unsubscribe();
    };
  });


</script>

<main>
  <header>
    <p>{workspace.name}</p>
    <p>10.5.9.0/16</p>
  </header>
</main>

<style>
  * {
    box-sizing: border-box;
    padding: 0;
    margin: 0;
  }
  header {
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 1rem;
    background-color: black;
    border-radius: 1rem;
    color: white;
  }
  header p {
    font-size: x-large;
  }

  .component {
    margin-top: 2rem;
    border: black 1px solid;
    border-radius: 1rem;
    display: flex;
    padding: 0.5rem;
    padding-left: 1rem;
  }
  .content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  .hostmane {
    display: flex;
    padding: 1rem;
    padding-top: 0.6rem;
    padding-bottom: 0.6rem;
    border: black 1px solid;
    border-radius: 1rem;
    font-size: x-large;
    gap: 2rem;
    background-color: rgb(212, 243, 253);
    width: fit-content;
  }
  .ports {
    display: flex;
    font-size: x-large;
    gap: 2rem;
  }
  .vertical-line {
    width: 2px;
    background-color: black;
    height: 5rem;
    align-self: center;
    margin-left: 1rem;
    margin-right: 1rem;
  }
  a {
    padding: 1rem;
    border: black 2px solid;
    border-radius: 1rem;
    background-color: black;
    color: white;
    font-size: xx-large;
    margin-left: auto;
    align-content: center;
  }
  a:hover {
    background-color: transparent;
    color: black;
  }
</style>
