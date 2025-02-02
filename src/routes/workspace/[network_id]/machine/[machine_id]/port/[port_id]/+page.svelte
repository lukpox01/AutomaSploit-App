<script>
// @ts-nocheck

  import { onMount } from "svelte";
  import { ChevronLeft, ChevronDown } from "lucide-svelte";
  import Note from "./Note.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/stores";
  import CredentialModal from './CredentialModal.svelte';

  let port = {};
  let showCredentialModal = false;

  onMount(() => {
    const unsubscribe = page.subscribe(async ($page) => {
      try {
        const network_id_str = $page.params.network_id;
        const network_id = parseInt(network_id_str);
        console.log(network_id);

        const port_id_str = $page.params.port_id;
        const port_id = parseInt(port_id_str);
        console.log(port_id);

        const machine_id_str = $page.params.machine_id;
        const machine_id = parseInt(machine_id_str);
        console.log(machine_id);

        const portJson = await invoke("get_port", {
          workspaceId: network_id,
            machineId: machine_id,
            portNumber: port_id
        });

        port = JSON.parse(portJson);

        console.log(port);
      } catch (error) {
        console.error("Error fetching ports:", error);
      }
    });

    return () => {
      unsubscribe();
    };
  });

  function openCredentialModal() {
    showCredentialModal = true;
  }

  function closeCredentialModal() {
    showCredentialModal = false;
  }

  async function handleSaveCredentials(creds) {
    try {
      const network_id = parseInt($page.params.network_id);
      const machine_id = parseInt($page.params.machine_id);
      const port_id = parseInt($page.params.port_id);

      const credNote = {
        Credentials: {
          name: creds.name || null,
          hash: creds.hash || null,
          password: creds.password || null
        }
      };
      
      const newData = [...port.data, credNote];
      
      await invoke('update_port_notes', {
        workspaceId: network_id,
        machineId: machine_id,
        portNumber: port_id,
        notes: newData
      });

      // Refresh port data
      const portJson = await invoke("get_port", {
        workspaceId: network_id,
        machineId: machine_id,
        portNumber: port_id
      });
      port = JSON.parse(portJson);
    } catch (error) {
      console.error("Error saving credentials:", error);
    }
  }
</script>

<main>
  <header>
    <div class="top-buttons">
      <u class="btn back-btn"
        ><span class="btn-align"
          ><ChevronLeft size="64" stroke-width="5" /></span
        ></u
      >
      <div class="right-buttons">
        <button type="button" class="btn btn-vulns">Vulns</button>
        <button type="button" class="btn btn-scan">Scan</button>
      </div>
    </div>

    <div class="port-info">
      <h1 class="service">{port.service}</h1>
      <h1 class="port-number">{port.number}</h1>
    </div>
    <div class="interact-buttons">
      <div class="select-wrapper">
        <select name="status" class="select">
          <option value="status-critical">Critical </option>
          <option value="status-high">High</option>
          <option value="status-medium">Medium</option>
          <option value="status-low">Low</option>
          <option value="status-info">Info</option>
        </select>

        <div class="select-icon"><ChevronDown size="32" /></div>
      </div>
      <button type="button" class="new-note-btn" on:click={openCredentialModal}>Add Credentials</button>
    </div>
  </header>
  <div class="content">

    <Note data={port.data} details={port.details}/>

  </div>
</main>

<CredentialModal 
    show={showCredentialModal}
    onClose={closeCredentialModal}
    onSave={handleSaveCredentials}
/>

<style>
  .btn {
    cursor: pointer;
    border: 3px solid black;
    border-radius: 1rem;
    background-color: transparent;
    height: 64px;
    font-size: x-large;
    font-weight: 600;
  }

  .back-btn {
    width: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .back-btn:hover {
    background-color: black;
    color: white;
    border: 3px solid white;
  }

  .btn-vulns {
    padding: 1rem;
    padding-inline: 4vw;
  }

  .btn-vulns:hover {
    background-color: black;
    color: white;
  }

  .btn-scan {
    padding: 1rem;
    color: white;
    background-color: black;
    padding-inline: 4vw;
  }

  .btn-scan:hover {
    background-color: white;
    color: black;
  }

  .btn-align {
    vertical-align: middle;
    position: relative;
    top: 2px;
    right: 2px;
  }

  .top-buttons {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
  }

  .right-buttons {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .port-info {
    display: flex;
    flex-direction: row;
    gap: 5vw;
    font-size: xx-large;
  }

  .select-wrapper {
    position: relative;
    display: inline-block;
  }

  .select {
    -webkit-appearance: none;
    -moz-appearance: none;
    appearance: none; /* Pre istotu */
    padding: 1rem;
    padding-right: 2rem;
    border: 2px solid black;
    border-radius: 1rem;
    font-size: x-large;
    background: none;
  }

  .select-icon {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-40%);
    pointer-events: none; /* Zabráňte interakcii s ikonou */
  }

  .new-note-btn {
    padding: 1rem;
    padding-inline: 4vw;
    background-color: transparent;
    border: 2px solid black;
    border-radius: 1rem;
    font-size: x-large;
    margin-top: 1rem;
  }

  .new-note-btn:hover {
    background-color: black;
    color: white;
    border: 2px solid white;
  }
</style>
