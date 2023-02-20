<script lang="ts">
  import MainView from "./views/MainView.svelte";
  import OptionsView from "./views/OptionsView.svelte";
  import { View, CurrentView, Log } from "./common";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import type { LogMessage } from "./common";

  let unlisten;

  onMount(async () => {
    unlisten = await listen<LogMessage>("log", (msg) => {
      console.log(msg);
      Log.update((log) => [...log, msg.payload]);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<main class="main">
  <div class="character-bg">
    <div>
      <img src="/images/Arknights_logo.webp" alt="Arknights logo" />
    </div>

    {#if $CurrentView == View.Options}
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
