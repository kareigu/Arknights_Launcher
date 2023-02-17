<script lang="ts">
  import { View, CurrentView } from "../common";
</script>

<main class="options-view">
  <div class="header">
    <h1>Options</h1>
    <button class="close-button" on:click={() => CurrentView.set(View.Main)}>
      X
    </button>
  </div>
</main>

<style>
  .options-view {
    width: 100%;
  }

  .header {
    position: relative;
    text-align: center;
    width: 100%;
  }

  .close-button {
    position: absolute;
    top: -0.5rem;
    right: 5rem;

    width: 3rem;
    height: 3rem;

    border: 0.15rem solid var(--ak-white);
    border-radius: 100%;
    background-color: var(--ak-black);
    color: var(--ak-white);
    font-weight: 700;
    font-size: 2rem;
    transition: all 80ms ease-in;
  }

  .close-button:hover {
    background-color: var(--button-grey);
    color: var(--ak-black);
  }
</style>
