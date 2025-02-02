<script>
// @ts-nocheck

  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { Monitor, FileWarning } from "lucide-svelte";
  import { onMount } from "svelte";


  let ports = [];

  let workspace = {};

  let machine = {};

  let scanning = false;

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

  async function scanMachine() {
    try {
      scanning = true;
      const workspaceId = parseInt($page.params.network_id);
      const machineId = parseInt($page.params.machine_id);
      console.log(`Starting scan for machine ${machineId} in workspace ${workspaceId}`);
      console.log(`Scanning machine: ${machine.hostname} (${machine.ip})`);
      
      const response = await invoke('scan_machine', { workspaceId, machineId });
      console.log("Scan completed:", response);
      
      console.log("Fetching updated port information...");
      const databaseJson = await invoke("ports", {
        workspaceId: workspaceId,
        machineId: machineId,
      });
      ports = JSON.parse(databaseJson);
      console.log(`Found ${ports.length} ports:`, ports);
    } catch (error) {
      console.error("Error during machine scan:", error);
    } finally {
      scanning = false;
      console.log("Scan process completed");
    }
  }
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
    {#if ports.length === 0}
        <div class="no-ports-message">No open ports found on this machine</div>
    {:else}
        {#each ports as port}
            <div class="component">
                <ul class="info">
                    <li>{port.service}</li>
                    <li>{port.number}</li>
                </ul>
                <div class="port-details">
                    <p>{port.protocol} - {port.state}</p>
                    {#if port.application}
                        <p>{port.application}</p>
                    {/if}
                    {#if port.details && port.details.length > 0}
                        <p class="details-available">Details Available</p>
                    {/if}
                </div>
                <div class="security-status-critical">Critical</div>
                <hr>
                <a href="/workspace/{$page.params.network_id}/machine/{$page.params.machine_id}/port/{port.number}">View Details</a>
            </div>
        {/each}
    {/if}
  </div>

  <div class="scan-btn-container">
    <button class="scan-btn" 
            on:click={scanMachine} 
            disabled={scanning}>
        {scanning ? 'Scanning...' : 'Scan Machine'}
    </button>
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

  .no-ports-message {
    width: 100%;
    text-align: center;
    font-size: x-large;
    padding: 2rem;
    border: 2px solid black;
    border-radius: 1rem;
    margin: 2rem 0;
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

  .scan-btn-container {
    position: fixed;
    bottom: 1rem;
    right: 1rem;
  }

  .scan-btn {
    padding: 1rem;
    border: black 3px solid;
    border-radius: 0.5rem;
    background-color: transparent;
    color: black;
    font-size: large;
    cursor: pointer;
  }
  .scan-btn:hover {
    background-color: black;
    color: white;
    border: white 3px solid;
  }
  .scan-btn[disabled] {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .port-details {
    padding: 0.5rem 1rem;
    font-size: medium;
  }
  .port-details p {
    margin: 0.25rem 0;
  }
  .details-available {
    color: green;
    font-weight: bold;
  }
</style>
