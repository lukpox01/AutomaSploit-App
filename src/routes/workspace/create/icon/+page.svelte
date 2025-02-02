<script>
    import { 
      Home, 
      User, 
      Settings 
    } from 'lucide-svelte';
  
    let selectedIcon = Home; // Default selected icon
  
    const icons = [
      { name: 'Home', component: Home },
      { name: 'User', component: User },
      { name: 'Settings', component: Settings }
    ];
  </script>
<style>
    .text {
        font-size: 5vw;
        margin: 1rem;
    }

    .input {
        font-size: 5vw;
        padding: 1rem;
        font-size: 1.5rem;
        border: none;
        border-bottom: 1px solid black;
        border-radius: 0;
        outline: none;
        text-align: center;
        background-color: transparent;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
    }
    option{
        color: blanchedalmond;
    }
</style>

<main class="container">
    <h1 class="text">
        Icon
    </h1>
    <div class="select-wrapper">
        <select bind:value={selectedIcon}>
            {#each icons as icon}
              <option value={icon.component}>
                <svelte:component this={icon.component} /> {icon.name} 
              </option>
            {/each}
          </select>

<p>Selected Icon:</p>
<svelte:component this={selectedIcon} size="32" /> 
    </div>
</main>
<!-- TODO implement this  -->