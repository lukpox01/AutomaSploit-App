<script>
  import { ChevronLeft } from "lucide-svelte";

    // Page-specific logic goes here
  </script>
  
  <main>
    <header>
        <a class="btn back-btn" href="/"
        ><span class="btn-align"
          ><ChevronLeft size="64" stroke-width="5" /></span
        ></a>
    </header>
    <slot></slot>

  </main>
  
  <style>
      .btn {
    cursor: pointer;
    border: 3px solid black;
    border-radius: 1rem;
    background-color: transparent;
    height: 64px;
    font-size: x-large;
    font-weight: 600;
    text-decoration: none;
    color: black;
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
  .btn-align {
    vertical-align: middle;
    position: relative;
    top: 2px;
    right: 2px;
  }
  </style>