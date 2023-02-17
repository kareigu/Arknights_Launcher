<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { View, CurrentView } from "../common";

  async function launch() {
    const res = await invoke("launch", {});
    console.log(res);
  }
</script>

<main class="main-buttons">
  <button class="launch-button button-text" on:click={launch}>
    <span class="launch-button-text"> Launch </span>
  </button>
  <button
    class="options-button button-text"
    on:click={() => CurrentView.set(View.Options)}
  >
    <span> ⚙️ </span>
  </button>
</main>

<style>
  .main-buttons {
    display: flex;
    gap: 0.2rem;
  }

  .launch-button {
    height: var(--button-height);
    width: var(--primary-button-width);
    border: 0;
    position: relative;
    color: var(--ak-black);
    background-color: var(--button-white);
    transition: background-color 80ms ease-in;
  }

  .launch-button:hover {
    background-color: var(--button-grey);
  }

  .launch-button-text {
    position: absolute;
    top: 0.4rem;
    left: 0.6rem;
  }

  .options-button {
    height: var(--button-height);
    width: var(--secondary-button-width);
    border: 0;
    color: var(--ak-white);
    background-color: var(--button-blue);
    transition: background-color 80ms ease-in;
  }

  .options-button:hover {
    background-color: var(--button-light-blue);
  }
</style>
