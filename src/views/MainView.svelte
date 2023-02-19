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
    <div class="button-dots" />
  </button>
  <button
    class="options-button button-text button-border"
    on:click={() => CurrentView.set(View.Options)}
  >
    <span style="position: absolute; top: 1rem; left: 1.2rem;"> ⚙️ </span>
    <div class="button-dots" />
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

  .button-dots {
    width: 100%;
    height: 100%;
    --mask: linear-gradient(to bottom right, transparent 50%, 92%, black);
    mask-image: var(--mask);
    -webkit-mask-image: var(--mask);
  }

  .button-dots::after {
    content: "";
    display: block;
    height: 100%;
    width: 100%;
    position: absolute;
    z-index: -1;
    top: 0;
    left: 0;
    background-image: radial-gradient(
      circle at 0.2rem 0.2rem,
      grey 0.15rem,
      transparent 0
    );
    background-size: 0.5rem 0.5rem;
    transform: skew(15deg, -5deg) translateY(1rem);
    transform-origin: top left;
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
    position: relative;
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
