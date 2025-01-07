<script>
    /** @type {{ data: import('./$types').PageData }} */
    let { data } = $props();
    import { onMount } from 'svelte';
    import { ArrowBigLeft } from "lucide-svelte";
</script>

<main>
    <header>
        <ArrowBigLeft />
    </header>
    <div class="content">

    </div>
</main>