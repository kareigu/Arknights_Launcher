<script lang="ts">
  import MainView from "./views/MainView.svelte";
  import OptionsView from "./views/OptionsView.svelte";
  import { View, CurrentView } from "./common";

  let view: View;
  CurrentView.subscribe((v: View) => (view = v));
</script>

<main class="main" on:mousemove={mouseMove}>
  <div>
    <img src="/images/Arknights_logo.webp" alt="Arknights logo" />
  </div>

  {#if view == View.Options}
    <OptionsView />
  {:else}
    <MainView />
  {/if}
</main>

<style>
  .main {
    background-color: var(--background-colour);
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
