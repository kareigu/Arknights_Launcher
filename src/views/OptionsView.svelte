<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { View, CurrentView, Options, BACKGROUND_BASE_URL } from "../common";
  import type { IOptions } from "../common";
  import backgrounds from "../backgrounds.json";

  let hidden = false;
  let dragging = false;
  let previous_mouse_pos: [number, number] = [0, 0];

  async function executablePathBrowse(defaultPath: string) {
    const path = await open({
      defaultPath,
      multiple: false,
      filters: [
        {
          name: "*",
          extensions: ["exe", "lnk", "exe.lnk"],
        },
      ],
    });

    Options.update((o: IOptions) => ({ ...o, executable_path: path }));
  }

  async function save() {
    await invoke("set_options", { newOptions: $Options });
  }

  async function close() {
    const options = await invoke("options", {});
    Options.set(options);
    CurrentView.set(View.Main);
  }

  onMount(async () => {
    const options = await invoke("options", {});
    Options.set(options);
    console.log(options);
  });
</script>

<main class="options-view" in:fade>
  <div class="header">
    <button class="close-button" on:click={close}>
      {"<"}
    </button>
    <div class="options-title">
      <img src="/icons/icons8-engineering-50.png" class="options-icon" />
      <span>Options</span>
    </div>
    <button class="toggle-hide-button">
      <img
        src={hidden ? "/icons/icons8-eye-58.png" : "/icons/icons8-eye-64.png"}
        width="30"
        on:click={() => (hidden = !hidden)}
      />
    </button>
  </div>
  {#if !hidden}
    <div class="options">
      <h2>Executable File Path</h2>
      <div class="executable-path-select">
        <input
          type="text"
          placeholder="File path to the executable"
          class="text-input path-input"
          value={$Options.executable_path}
          readonly
        />
        <button
          class="option-button"
          on:click={() => executablePathBrowse($Options.executable_path)}
          >Browse</button
        >
      </div>

      <h2>Background</h2>
      <div class="picture-select">
        <span class="picture-select-label">Background:</span>
        <div class="picture-select-options">
          {#each backgrounds.backgrounds as background}
            <span class="picture-select-option">
              <img
                class={`picture ${
                  $Options.background.Default.background === background
                    ? "selected"
                    : ""
                }`}
                src={`${BACKGROUND_BASE_URL}/${background}`}
                width="150rem"
                on:click={() =>
                  Options.update((o) => {
                    o.background.Default.background = background;
                    return o;
                  })}
              />
            </span>
          {/each}
        </div>
      </div>
      <div class="picture-select">
        <span class="picture-select-label">Character:</span>
        <div class="picture-select-options">
          {#each backgrounds.characters as character}
            <span class="picture-select-option">
              <img
                class={`picture ${
                  $Options.background.Default.character === character
                    ? "selected"
                    : ""
                }`}
                src={`${BACKGROUND_BASE_URL}/${character}`}
                width="150rem"
                on:click={() =>
                  Options.update((o) => {
                    o.background.Default.character = character;
                    return o;
                  })}
              />
            </span>
          {/each}
        </div>
      </div>

      <div class="save-button-container">
        <button class="save-button button-border" on:click={save}>
          <span class="save-button-text">Save</span>
          <img class="save-button-icon" src="/icons/icons8-save-50.png" />
          <div class="button-dots" />
        </button>
      </div>
    </div>
  {:else}
    <div class="character-position-options">
      <div
        class="drag-surface"
        on:mousedown={() => (dragging = true)}
        on:mouseup={() => (dragging = false)}
        on:mousemove={(e) => {
          if (!dragging) {
            previous_mouse_pos = [e.clientX, e.clientY];
            return;
          }
          const mouse_pos_difference = [
            e.clientX - previous_mouse_pos[0],
            e.clientY - previous_mouse_pos[1],
          ];
          Options.update((o) => {
            const offset = o.background.Default.offset;
            const multiplier = 1;
            o.background.Default.offset = [
              (offset[0] - mouse_pos_difference[0]) * multiplier,
              (offset[1] - mouse_pos_difference[1]) * multiplier,
            ];
            return o;
          });
          previous_mouse_pos = [e.clientX, e.clientY];
        }}
      />
      <div class="zoom-level-container">
        <span class="zoom-level-label">Zoom level</span>
        <input
          class="zoom-level-slider"
          type="range"
          min={50}
          max={300}
          value={$Options.background.Default.zoom}
          on:input={(e) => {
            const new_zoom = e.currentTarget.valueAsNumber;
            Options.update((o) => {
              o.background.Default.zoom = new_zoom;
              return o;
            });
          }}
        />
        <button
          class="reset-character-position-button"
          on:click={() =>
            Options.update((o) => {
              o.background.Default.zoom = 100;
              o.background.Default.offset = [0, 0];
              return o;
            })}>Reset position</button
        >
      </div>
    </div>
  {/if}
</main>

<style>
  .options-view {
    --background-blur: blur(3px);
    width: 100%;
    border-radius: 0.2rem;
    font-family: "PT Sans";
    box-shadow: 0.4rem 0.4rem 0.5rem rgba(0, 0, 0, 0.2);
    height: 100%;
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 2rem;
    position: relative;
    text-align: center;
    width: 100%;
    user-select: none;
    text-transform: uppercase;
    line-height: 1rem;
    background-color: rgba(0, 0, 0, 0.4);
    backdrop-filter: var(--background-blur);
    border-bottom: 0.5rem solid var(--ak-white);
    box-shadow: 0rem 1rem 0.1rem rgba(0, 0, 0, 0.5);
  }

  .options-title {
    background-color: var(--ak-white);
    color: var(--ak-black);
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: 400;
    font-size: 0.8rem;
    font-variant-caps: unicase;
    margin-block-start: 0;
    margin-block-end: 0;
    padding: 0.2rem 0.4rem;
    position: relative;
  }

  .options-icon {
    width: 1.8rem;
    margin-right: 0.4rem;
    filter: drop-shadow(0 1.5mm 1mm rgba(0, 0, 0, 0.4));
  }

  .close-button {
    width: 5rem;
    height: 1.5rem;
    margin-left: 1rem;

    border: 0;
    border-radius: 0.1rem;
    background-color: var(--ak-white);
    color: var(--ak-black);
    font-size: 1.5rem;
    text-align: left;
    line-height: 1rem;
    box-shadow: 0.2rem 0.3rem 0.4rem rgba(255, 255, 255, 0.3);
    transition: all 80ms ease-in;
  }

  .close-button:hover {
    background-color: var(--button-grey);
    color: var(--ak-black);
  }

  .close-button:active {
    background-color: var(--ak-black);
    color: var(--ak-grey);
    box-shadow: inset 0.01rem 0.05rem 0.4rem rgba(0, 0, 0, 1);
    border-color: var(--ak-black);
  }

  .toggle-hide-button {
    width: 3rem;
    height: 1.5rem;
    margin-left: auto;
    margin-right: 1rem;
    position: relative;

    display: flex;
    justify-content: center;
    align-items: center;

    border: 0;
    border-radius: 0.1rem;
    background-color: var(--ak-white);
    color: var(--ak-black);
    font-size: 1.5rem;
    text-align: left;
    line-height: 1rem;
    box-shadow: 0.2rem 0.3rem 0.4rem rgba(255, 255, 255, 0.3);
    transition: all 80ms ease-in;
  }

  .toggle-hide-button:hover {
    background-color: var(--button-grey);
    color: var(--ak-black);
  }

  .toggle-hide-button:active {
    background-color: var(--ak-black);
    color: var(--ak-grey);
    box-shadow: inset 0.01rem 0.05rem 0.4rem rgba(0, 0, 0, 1);
    border-color: var(--ak-black);
  }

  .options {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: var(--ak-white-translucent);
    backdrop-filter: var(--background-blur);
    width: 100%;
    height: 100%;
    overflow-x: hidden;
    overflow-y: scroll;
    animation: blur-in 80ms ease-in;
  }

  .options::-webkit-scrollbar {
    background: var(--ak-white-translucent);
    backdrop-filter: var(--background-blur);
    width: 0.4rem;
  }

  .options::-webkit-scrollbar-thumb {
    background: var(--ak-black);
    border-radius: 1rem;
  }

  .options > h2 {
    background-color: var(--ak-black);
    width: 90%;
    font-weight: normal;
    text-align: left;
    padding: 0.1rem 1rem;
    font-size: 1.2rem;
    border: 0.1rem white inset;
    user-select: none;
  }

  .option-button {
    background-color: var(--ak-black);
    border-radius: 5rem;
    border: 0.1rem solid var(--ak-white);
    color: var(--ak-white);
    font-family: "PT Sans";
    font-weight: normal;
    font-size: 0.7rem;
    border-radius: 0.1rem;
    padding: 0.2rem 2rem;
    box-shadow: 0.1rem 0.4rem 0.3rem rgba(0, 0, 0, 0.2);
    user-select: none;
    transition: all 80ms ease-in;
  }

  .option-button:hover:not([disabled]) {
    filter: brightness(0.8);
  }

  .option-button:active:not([disabled]) {
    filter: brightness(1.1);
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

  .picture-select {
    display: flex;
    align-items: center;
    gap: 1rem;
    color: var(--ak-black);
    width: 90%;
    margin: 0.5rem 0rem;
    user-select: none;
  }

  .picture-select-label {
    width: 7rem;
  }

  .picture-select-options {
    background-color: var(--ak-black);
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    border: 0.1rem white inset;
    width: 100%;
    padding: 0.2rem 0.5rem;
    overflow-x: scroll;
  }

  .picture-select-options::-webkit-scrollbar {
    background: var(--ak-black);
    height: 0.4rem;
  }

  .picture-select-options::-webkit-scrollbar-thumb {
    background: var(--ak-white-translucent);
  }

  .picture {
    cursor: pointer;
    border: 0.2rem inset var(--ak-grey);
    animation: blur-in 80ms ease-in;
    border-radius: 0.5rem;
  }

  .picture.selected {
    border: 0.2rem inset var(--button-light-blue);
  }

  .picture:hover {
    border: 0.2rem outset var(--button-blue);
  }

  .save-button-container {
    padding: 0.5rem 1rem;
    display: flex;
    justify-content: end;
    width: 95%;
  }

  .save-button {
    height: calc(var(--button-height) / 1.5);
    width: calc(var(--primary-button-width) / 2);
    position: relative;
    color: var(--ak-white);
    background-color: var(--button-blue);
    font-size: 1.7rem;
    font-weight: bold;
    transition: all 80ms ease-in;
  }

  .save-button:hover {
    background-color: var(--button-light-blue);
  }

  .save-button-text {
    position: absolute;
    bottom: 0.5rem;
    left: 1rem;
    z-index: 2;
  }

  .save-button-icon {
    position: absolute;
    font-size: 3rem;
    right: 0.9rem;
    top: 0.4rem;
    opacity: 0.5;
    color: white;
    filter: invert(1);
    z-index: 1;
  }

  .character-position-options {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
    animation: blur-in 80ms ease-in;
  }

  .drag-surface {
    width: 100%;
    height: 100%;
  }

  .zoom-level-container {
    position: absolute;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    top: 45%;
    right: 0rem;
    transform: translate(0, -50%);
  }

  .zoom-level-slider {
    -webkit-appearance: slider-vertical;
  }

  @keyframes blur-in {
    0% {
      filter: blur(10px);
    }
    100% {
      filter: blur(0);
    }
  }
</style>
