<script>
  import { page } from '$app/stores';
  import {Monitor, FileWarning} from 'lucide-svelte';

  let computers = [
    {
        type: 'PC',
        hostname: 'PCSOSE11',
        ip: '10.10.20.68',
        ports: [
            {service: 'FTP', port: 21, favourite: false},
            {service: 'SSH', port: 22,favourite: true},
            {service: 'HTTP', port: 8080, favourite: true},
            {service: 'HTTPS', port: 443,favourite: false},
            {service: 'RDP', port: 3389,favourite: true}
        ]
    },
    {
        type: 'PC',
        hostname: 'PCSOSE1',
        ip: '10.10.5.8',
        ports: [
            {service: 'FTP', port: 21, favourite: true},
            {service: 'SSH', port: 22,favourite: false},
            {service: 'HTTP', port: 8000, favourite: true},
            {service: 'HTTPS', port: 443,favourite: true},
            {service: 'RDP', port: 3389,favourite: false}
        ]
    }
]
</script>

<style>
        * {
      box-sizing: border-box;
      padding: 0;
        margin: 0;
    }
    header {
        display: flex;
        justify-content:space-around;
        align-items: center;
        padding: 1rem;
        background-color: black;
        border-radius: 1rem;
        color: white;
    }
    header p {
        font-size: x-large;
    }

    .component{
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
    button {
        padding: 1rem;
        border: black 2px solid;
        border-radius: 1rem;
        background-color: black;
        color: white;
        font-size: xx-large;
        margin-left: auto;
    }
    button:hover {
        background-color: transparent;
        color: black;
    }
    
</style>

<main>
    <header>
        <p>{$page.params.id}</p>
        <p>10.5.9.0/16</p>
    </header>

    {#each computers as computer}

    <div class="component">
        {#if computer.type === 'PC'}
            <Monitor size="100" />
        {:else}    
            <FileWarning size="100" />
        {/if}
        <div class="vertical-line"></div>
        <div class="content">
            <div class="hostmane">
                <p>{computer.hostname}</p>
                <p>{computer.ip}</p>
            </div>
            <div class="ports">
                {#each computer.ports as port, index}
                    {#if port.favourite}
                        <p> {port.service} | {port.port}</p>
                    {/if}
                {/each}
            </div>
         </div>
         <button type="button">Details</button>
    </div>

    {/each}

</main>
