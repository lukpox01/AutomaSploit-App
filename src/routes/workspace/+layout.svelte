<script>
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api/core';
    import {Settings, Map, Monitor, FileWarning, icons} from 'lucide-svelte';
    import { onMount } from 'svelte';

    /**
     * @type {any[]}
     */
    let machines = [];

  onMount(() => {
      const unsubscribe = page.subscribe(async ($page) => {
        try {
          const network_id_str = $page.params.network_id;
          const network_id = parseInt(network_id_str);
          console.log(network_id);
          const databaseJson = await invoke('machines', { workspaceId: network_id });
          machines = JSON.parse(databaseJson);
          console.log(machines);
        } catch (error) {
          console.error("Error fetching workspace:", error);
        }
      });
  
      return () => {
        unsubscribe();
      };
    });
  </script>
  
  <style>
    * {
      box-sizing: border-box;
      padding: 0;
        margin: 0;
        background-color: #f5faff;
    }
    :root{
        background-color: #f5faff;
    }
    .layout {
      display: flex;
      height: 98.5vh;
    }
    .sidebar {
      padding: 1rem;
      border: black 1px solid;
      border-radius: 1rem;
        width: 17rem;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
    }
    .sidebar ul {
      list-style-type: none;
      padding: 0;
    }

    .sidebar ul li  {
      margin-bottom:0.3rem;
      border-radius: 1rem;
    }
    .sidebar ul li a{
      text-decoration: none;
      color: black;
      font-size: larger;
      display: flex;
      align-items: center;
      padding: 1rem;
      border-radius: 1rem;
    }
    .sidebar ul li:hover *{
      background-color: black;
      color: white;
    }
    .content {
      flex: 1;
      padding: 1rem;
    }
    .logo {
      font-size: 1.5rem;
      font-weight: bold;
      margin-bottom: 0.5rem;
      font-size: xx-large;
    }
    hr {
      margin-top: 1rem;
      margin-bottom: 1rem;
    }
    
    .bottom-links {
      margin-top: auto;
    }
    .icon-align {
      vertical-align: middle;
      padding-right: 1rem;
      position: relative;
      top: 5px;
    }
    .newscan-btn {
      background-color: black;
      height: 4rem ;
      color: white;
      font-size: xx-large;
      border-radius: 1rem;
        border: none;
        margin-bottom: 2rem;
    }

  </style>
  
  <div class="layout">
    <nav class="sidebar">
        <h1 class="logo">AutomaSploit</h1>
        <button type="button" class="newscan-btn">+</button>
        <h3>Machines</h3>
        <hr/>
      <ul>

        {#each machines as machine}
          {#if machine.icon == "PC"}
            <li><a href="/workspace/{$page.params.network_id}/machine/{machine.id}"><span class="icon-align"><Monitor size="24" /></span>{machine.hostname}</a></li>
          {:else}
            <li><a href="/workspace/{$page.params.network_id}/machine/{machine.id}"><span class="icon-align"><FileWarning size="24" /></span>{machine.hostname}</a></li>
          {/if}
        {/each}

      </ul>
      <ul class="bottom-links">
        
        <li><a href="/settings"><span class="icon-align"><Settings size="24" /></span>Settings</a></li>
        <li><a href="/roadmap"><span class="icon-align"><Map size="24"/></span>Roadmap</a></li>
      </ul>
    </nav>
    <div class="content">
      <slot></slot>
    </div>
  </div>