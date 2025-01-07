<script>
// @ts-nocheck

  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { Monitor, FileWarning } from "lucide-svelte";
  import { onMount } from "svelte";


  let ports = [];

  let workspace = {};

  let machine = {};

  onMount(() => {
    const unsubscribe = page.subscribe(async ($page) => {
      try {
        const network_id_str = $page.params.network_id;
        const network_id = parseInt(network_id_str);
        const machine_id_str = $page.params.machine_id;
        const machine_id = parseInt(machine_id_str);
        console.log(network_id);
        console.log(machine_id);
        const databaseJson = await invoke("ports", {
          workspaceId: network_id,
          machineId: machine_id,
        });

        const machineJson = await invoke("get_machine", {
          workspaceId: network_id,
          machineId: machine_id,
        });

        machine = JSON.parse(machineJson);

        console.log(machine);

        const workspaceJson = await invoke("get_workspace", {
          workspaceId: network_id,
        });

        workspace = JSON.parse(workspaceJson);

        console.log(workspace);

        ports = JSON.parse(databaseJson);
        console.log(ports);
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
    <p>{workspace.ip_range}</p>
  </header>
  <div class="header">
    <p>{machine.hostname}</p>
    <p>{machine.ip}</p>
  </div>

  <div class="ports">
    {#each ports as port}
      <div class="component">
        <ul class="info">
          <li>{port.service}</li>
          <li>{port.number}</li>
        </ul>
        <div class="security-status-critical">Critical</div>
        <hr>
        <div class="no-notes">NO NOTES</div>
        <a href="/workspace/{$page.params.network_id}/machine/{$page.params.machine_id}/port/{port.number}">Notes</a>
      </div>
    {/each}
  </div>
</main>

<style>

.security-status-critical {
    color: orange;
    font-size: x-large;
    font-weight: bolder;
    margin-left: 1rem;
  }

  hr {
    border: 2px solid black;
    margin:1rem;
  }
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

  header {
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 1rem;
    background-color: black;
    border-radius: 1rem;
    color: white;
  }

  .component {
    margin-top: 2rem;
    border: black 2px solid;
    border-radius: 1rem;
    display: flex;
    flex-direction: column;
    padding: 0.5rem;

    width: 30%;
  }
  .info {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    padding: 1rem;
    justify-content: space-between;
    font-size: xx-large;
    font-weight: bolder;
    list-style: none;
  }
  .no-notes {
    padding-top: 4rem;
    padding-bottom: 4rem;
    font-size: x-large;
    text-align: center;
  }

  .ports {
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    gap: 3rem;
  }

  a {
    padding: 1rem;
    border: black 2px solid;
    border-radius: 1rem;
    background-color: black;
    color: white;
    font-size: xx-large;
    text-decoration: none;
    align-content: center;
    text-align: center;
    margin-left: 0.5rem;
    margin-right: 0.5rem;
  }
  a:hover {
    background-color: transparent;
    color: black;
  }
  .header {
    display: flex;
    justify-content: space-around;
    align-items: center;
    margin: 1rem;
    padding: 1rem;
    background-color: black;
    border-radius: 1rem;
    color: white;
    font-size: x-large;
  }
</style>
