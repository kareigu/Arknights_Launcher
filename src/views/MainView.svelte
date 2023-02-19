<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { View, CurrentView } from "../common";

  let has_activity = false;
  let launch_button_text = "Launch";

  let skew_to_hover = false;

  async function launch() {
    launch_button_text = has_activity ? "Stopping" : "Launching";
    try {
      await invoke(has_activity ? "stop" : "launch", {});
      has_activity = !has_activity;
    } catch (e) {}
    launch_button_text = has_activity ? "Stop" : "Launch";
  }
</script>

<main
  class="main-buttons"
  class:skew_to_hover
  on:mouseenter={() => (skew_to_hover = true)}
  on:mouseleave={() => (skew_to_hover = false)}
>
  <button class="launch-button button-text button-border" on:click={launch}>
    <span class="launch-button-text">{launch_button_text}</span>
  </button>
  <button
    class="options-button button-text button-border"
    on:click={() => CurrentView.set(View.Options)}
  >
    <span> ⚙️ </span>
  </button>
</main>

<style>
  .main-buttons {
    display: flex;
    padding: 0.5rem;
    gap: 0.2rem;
    transform: skew(1deg, 2deg);
  }

  .skew_to_hover {
    transform: skew(var(--mouse-x), var(--mouse-y));
  }

  .button-border {
    border-width: 0.15rem;
    border-color: rgba(0, 0, 0, 0.1);
    border-style: none outset outset none;
  }

  .button-border:hover {
    border-width: 0.18rem;
  }

  .button-border:active {
    border-width: 0.2rem;
  }

  .launch-button {
    height: var(--button-height);
    width: var(--primary-button-width);
    position: relative;
    color: var(--ak-black);
    background-color: var(--button-white);
    transition: all 80ms ease-in;
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
    color: var(--ak-white);
    background-color: var(--button-blue);
    transition: all 80ms ease-in;
  }

  .options-button:hover {
    background-color: var(--button-light-blue);
  }
</style>
