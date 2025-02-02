<script>
    let workspaceName = "";
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

    .next-button {
        position: absolute;
        bottom: 1rem;
        right: 1rem;
        padding: 1rem 2rem;
        font-size: 1.5rem;
        background-color: #007BFF;
        color: white;
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        text-decoration: none;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>

<main class="container">
    <h1 class="text">
        Workspace Name
    </h1>
    <input type="text" class="input" placeholder="Required" bind:value={workspaceName} maxlength="15" />
    <a href={`/workspace/create/iprange?name=${workspaceName}`} class="next-button">Next</a>
</main>