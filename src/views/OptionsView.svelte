<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { View, CurrentView } from "../common";
  import type { Options } from "../common";

  let options: Options = { executable_path: "", background: {} };
  let executablePath = "";

  async function executablePathBrowse() {
    const path = await open({
      defaultPath: executablePath,
      multiple: false,
      filters: [
        {
          name: "*",
          extensions: ["exe", "lnk", "exe.lnk"],
        },
      ],
    });

    executablePath = path;
  }

  async function save() {
    options.executable_path = executablePath;
    await invoke("set_options", { newOptions: options });
  }

  onMount(async () => {
    options = await invoke("options", {});
    await invoke("log", { msg: JSON.stringify(options) });
    executablePath = options.executable_path;
  });
</script>

<main class="options-view">
  <div class="header">
    <h1>Options</h1>
    <button class="close-button" on:click={() => CurrentView.set(View.Main)}>
      X
    </button>
  </div>
  <div class="options">
    <h2>Executable file path</h2>
    <div class="executable-path-select">
      <input
        type="text"
        placeholder="File path to the executable"
        class="text-input path-input"
        value={executablePath}
      />
      <button class="option-button" on:click={executablePathBrowse}
        >Browse</button
      >
    </div>

    <h2>Background</h2>

    <div class="save-buttons">
      <button class="option-button" on:click={save}>Save</button>
      <button class="option-button" disabled>Undo</button>
    </div>
  </div>
</main>

<style>
  .options-view {
    width: 80%;
    background-color: var(--ak-grey);
    border-radius: 0.2rem;
    box-shadow: 0.4rem 0.4rem 0.5rem rgba(0, 0, 0, 0.2);
  }

  .header {
    position: relative;
    text-align: center;
    width: 100%;
    user-select: none;
    font-weight: bold;
    text-transform: uppercase;
  }

  .header > h1 {
    font-weight: bold;
    font-variant-caps: unicase;
  }

  .close-button {
    position: absolute;
    top: -0.5rem;
    right: 0.8rem;

    width: 3rem;
    height: 3rem;

    border: 0.15rem solid var(--ak-white);
    border-radius: 100%;
    background-color: var(--ak-black);
    color: var(--ak-white);
    font-weight: 700;
    font-size: 2rem;
    box-shadow: inset 0.1rem 0.2rem 0.2rem rgba(255, 255, 255, 0.3);
    transition: all 80ms ease-in;
  }

  .close-button:hover {
    background-color: var(--button-grey);
    color: var(--ak-black);
    box-shadow: inset 0.01rem 0.05rem 0.4rem rgba(0, 0, 0, 0.8);
  }

  .close-button:active {
    background-color: var(--ak-black);
    color: var(--ak-grey);
    box-shadow: inset 0.01rem 0.05rem 0.4rem rgba(0, 0, 0, 1);
    border-color: var(--ak-black);
  }

  .options {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    padding-top: 0.2rem;
    padding-bottom: 2rem;
  }

  .option-button {
    background-color: var(--ak-black);
    border-radius: 5rem;
    border: 0.15rem solid rgba(255, 255, 255, 0.2);
    color: var(--ak-white);
    font-family: "PT Sans";
    font-weight: bold;
    font-size: 1rem;
    text-transform: uppercase;
    border-radius: 0.15rem;
    box-shadow: 0.4rem 0.4rem 0.5rem rgba(0, 0, 0, 0.2);
    user-select: none;
    transition: all 80ms ease-in;
  }

  .option-button:hover:not([disabled]) {
    border-color: rgba(255, 255, 255, 0.4);
    box-shadow: 0.1rem 0.1rem 0.4rem rgba(0, 0, 0, 0.2);
  }

  .option-button:active:not([disabled]) {
    border-color: rgba(255, 255, 255, 0.6);
    box-shadow: 0rem 0rem 0.2rem rgba(0, 0, 0, 0.2);
  }

  .option-button:disabled {
    color: var(--ak-grey);
  }

  .text-input {
    border-radius: 0.2rem;
    border: 0.18rem inset rgba(255, 255, 255, 0.15);
    box-shadow: inset 0.01rem 0.05rem 0.4rem rgba(0, 0, 0, 0.4);
    background-color: var(--ak-black);
    color: var(--ak-grey);
    padding: 0.5rem;
  }

  .path-input {
    width: 100%;
  }

  .executable-path-select {
    width: 90%;
    display: flex;
    justify-content: center;
    gap: 0.5rem;
  }

  .save-buttons {
    padding-top: 3rem;
    height: 2rem;
    display: flex;
    gap: 2rem;
  }

  .save-buttons > button {
    height: 100%;
  }
</style>
