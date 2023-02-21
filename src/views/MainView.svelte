<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { View, CurrentView, Log, format_log_message } from "../common";
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { Canvas, Layer, t, type Render } from "svelte-canvas";
  import type { User } from "../common";

  let user: User;

  let has_activity = false;
  let launch_button_text = "Launch";

  let message_log_element: HTMLElement;

  let date = new Date();
  $: hours = add_leading_zeroes(date.getHours(), 2);
  $: minutes = add_leading_zeroes(date.getMinutes(), 2);

  function add_leading_zeroes(n: number, digits: number): string {
    const as_string = n.toString();
    let placeholder = "";
    for (let i = 0; i < digits - as_string.length; i++) placeholder += "0";

    return placeholder + as_string;
  }

  async function launch() {
    launch_button_text = has_activity ? "Stopping" : "Launching";
    try {
      await invoke(has_activity ? "stop" : "launch", {});
      has_activity = !has_activity;
    } catch (e) {}
    launch_button_text = has_activity ? "Stop" : "Launch";
  }

  function onMouseOver(e: MouseEvent) {
    document.documentElement.style.setProperty(
      "--menu-skew-x",
      `${e.clientX / 340}deg`
    );
    document.documentElement.style.setProperty(
      "--menu-skew-y",
      `${e.clientY / 160}deg`
    );
  }

  $: render = ({
    context,
    width,
  }: {
    context: CanvasRenderingContext2D;
    width: number;
  }) => {
    const lineWidth = 5;
    const radius = (width - lineWidth) / 2;
    context.lineCap = "square";
    context.lineWidth = lineWidth;
    const drawCircle = (color: string, percent: number) => {
      context.beginPath();
      context.arc(
        width / 2,
        width / 2,
        radius,
        0,
        Math.PI * 2 * percent,
        false
      );
      context.strokeStyle = color;
      context.stroke();
    };
    drawCircle("#fefefe", 1);
    drawCircle("#f5de47", date.getSeconds() / 60);
  };

  $: {
    $Log;
    if (message_log_element)
      message_log_element.scrollTop = message_log_element.scrollHeight;
  }

  onMount(async () => {
    const data = await invoke("user", {});
    user = data;
    console.log(user);
    document.documentElement.style.setProperty("--menu-skew-x", "1.75deg");
    document.documentElement.style.setProperty("--menu-skew-y", "2.85deg");
  });

  const timeInterval = setInterval(() => (date = new Date()), 500);

  onDestroy(() => {
    clearInterval(timeInterval);
  });
</script>

<main class="main-view" in:fade>
  <div class="profile-info">
    <div class="timer-container">
      <Canvas width={100} height={100} style="transform: rotateZ(-90deg);">
        <Layer {render} />
      </Canvas>
      <span class="clock">{hours}:{minutes}</span>
      <span class="clock-undertext">HR</span>
    </div>
    {#if user}
      <div class="user-info-container">
        <span class="user-name"
          >Dr. {user.name} #{add_leading_zeroes(user.discriminator, 4)}</span
        >
        <!-- TODO: Implement showing important log messages here-->
        <div class="message-box">
          <svg
            height="10"
            width="16"
            class="message-box-arrow"
            on:click={() =>
              (message_log_element.scrollTop =
                message_log_element.scrollHeight)}
          >
            <polygon points="0,0 8,10 16,0" />
          </svg>
          <div class="message-box-log" bind:this={message_log_element}>
            {#each $Log as e}
              <span>{format_log_message(e)}</span>
            {/each}
          </div>
        </div>
      </div>
    {:else}
      <!-- TODO: Have a better looking solution for when no user data is available-->
      <!-- TODO: It shouldn't remove the log from view especially-->
      <div>
        <span>Loading</span>
      </div>
    {/if}
  </div>
  <div class="main-buttons skew-to-hover" on:mousemove={onMouseOver}>
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
  </div>
</main>

<style>
  .main-view {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .profile-info {
    display: flex;
    width: 100%;
    margin-bottom: 1rem;
    margin-top: 2rem;
  }

  .timer-container {
    position: relative;
    width: 100px;
    height: 100px;
    user-select: none;
  }

  .clock {
    font-size: 2rem;
    font-weight: bold;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .clock-undertext {
    position: absolute;
    font-weight: bold;
    font-size: 1.05rem;
    top: 78%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .user-info-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 0.5rem 1rem;
    width: 100%;
  }

  .user-name {
    user-select: none;
    font-size: 1.5rem;
  }

  .message-box {
    position: relative;
    width: 18rem;
    height: 3rem;
    background: rgba(0, 0, 0, 0.6);
    padding: 0.5rem 0.5rem;
    transform: skew(-1deg, -1deg);
  }

  .message-box::before {
    content: "VOICE";
    padding-left: 0.4rem;
    font-size: 0.5rem;
    line-height: 0.8rem;
    color: rgba(0, 0, 0, 0.4);
    display: block;
    position: absolute;
    z-index: 1;
    top: -0.15rem;
    left: -0.3rem;
    background: #666666;
    width: 30%;
    height: 0.75rem;
    box-shadow: 0.7rem 0.5rem 0.3rem rgba(0, 0, 0, 0.2);
  }

  .message-box-arrow {
    fill: var(--ak-white);
    position: absolute;
    right: 0.3rem;
    bottom: 0.4rem;
    cursor: pointer;
  }

  .message-box-arrow:hover {
    filter: brightness(0.5);
  }

  .message-box-arrow:active {
    filter: brightness(1.5);
  }

  .message-box-log {
    overflow-y: scroll;
    height: 100%;
  }

  .message-box-log > span {
    display: block;
    line-height: 1.15rem;
  }

  .message-box-log::-webkit-scrollbar {
    width: 0.4rem;
    background: rgba(0, 0, 0, 0.4);
  }

  .message-box-log::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }

  .message-box-log::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }

  .main-buttons {
    display: flex;
    padding: 0.5rem;
    gap: 0.2rem;
    /*transform: skew(1deg, 2deg);*/
    user-select: none;
  }

  .skew-to-hover {
    transform: skew(var(--menu-skew-x), var(--menu-skew-y));
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
