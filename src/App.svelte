<script lang="ts">
  import MainView from "./views/MainView.svelte";
  import OptionsView from "./views/OptionsView.svelte";
  import {
    View,
    CurrentView,
    Log,
    Options,
    BACKGROUND_BASE_URL,
  } from "./common";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api";
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import type { LogMessage, IOptions } from "./common";

  let unlisten;
  const options_unsubscribe = Options.subscribe((o: IOptions) => {
    console.log("Options subscription: ", o);
    document.documentElement.style.setProperty(
      "--background-image",
      `url(${BACKGROUND_BASE_URL}/${o.background.Default.background})`
    );
    document.documentElement.style.setProperty(
      "--background-character",
      `url(${BACKGROUND_BASE_URL}/${o.background.Default.character})`
    );
    document.documentElement.style.setProperty(
      "--background-character-zoom",
      `${o.background.Default.zoom}%`
    );
    document.documentElement.style.setProperty(
      "--background-character-offset-x",
      `${o.background.Default.offset[0]}%`
    );
    document.documentElement.style.setProperty(
      "--background-character-offset-y",
      `${o.background.Default.offset[1]}%`
    );
  });

  onMount(async () => {
    unlisten = await listen<LogMessage>("log", (msg) => {
      console.log(msg);
      Log.update((log: LogMessage[]) => [...log, msg.payload]);
    });

    const options: IOptions = await invoke("options", {});
    Options.set(options);

    await invoke("initialise", {});
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    options_unsubscribe();
  });
</script>

<main class="main">
  <div class="character-bg">
    {#if $CurrentView == View.Options}
      <OptionsView />
    {:else}
      <div in:fade>
        <img src="/images/Arknights_logo.webp" alt="Arknights logo" />
      </div>
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
    background-size: var(--background-character-zoom);
    background-position: var(--background-character-offset-x)
      var(--background-character-offset-y);
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
  }
</style>
