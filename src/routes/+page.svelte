<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Fa from "svelte-fa";
  import { faPlus, faFolder } from "@fortawesome/free-solid-svg-icons";

  import AddLinkDialog from "../lib/AddLinkDialog.svelte";

  let customMsg = $state("");
  
  async function my_custom_command(event: Event) {
    event.preventDefault();
    customMsg = await invoke("my_custom_command");
  }
</script>

<aside class="sidebar">
  <button aria-label="Add link button">
    <Fa icon={faPlus} />
  </button>
</aside>
<main class="main-content">
  <h1>Welcome to Sottr</h1>
  <p>Your downloads will appear here</p>
</main>
<footer>
  <form class="row" onsubmit={my_custom_command}>
    <button type="submit">Custom</button>
  </form>
  <p>{customMsg}</p>
</footer>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.main-content {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.sidebar {
  width: 64px; /* or 240px if expanded */
  background-color: #2f2f2f; /* distinct color from main */
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 0;
  border-right: 1px solid #444; /* visual separation */
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
}
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
