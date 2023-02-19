<script lang="ts">
  import MainView from "./views/MainView.svelte";
  import OptionsView from "./views/OptionsView.svelte";
  import { View, CurrentView } from "./common";

  let view: View;
  CurrentView.subscribe((v: View) => (view = v));

  function mouseMove(e: MouseEvent) {
    document.documentElement.style.setProperty(
      "--mouse-x",
      `${e.clientX / 340}deg`
    );
    document.documentElement.style.setProperty(
      "--mouse-y",
      `${e.clientY / 160}deg`
    );
  }
</script>

<main class="main" on:mousemove={mouseMove}>
  <div class="character-bg">
    <div>
      <img src="/images/Arknights_logo.webp" alt="Arknights logo" />
    </div>

    {#if view == View.Options}
      <OptionsView />
    {:else}
      <MainView />
    {/if}
  </div>
</main>

<style>
  .main {
    background: var(--background-image);
    background-repeat: no-repeat;
    background-size: cover;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
  }

  .character-bg {
    background: var(--background-character);
    background-repeat: no-repeat;
    background-size: cover;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
  }
</style>
