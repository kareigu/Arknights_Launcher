<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { View, CurrentView } from "../common";
  import { onMount, onDestroy } from "svelte";
  import { Canvas, Layer, t, type Render } from "svelte-canvas";
  import type { User } from "../common";

  let user: User;

  let has_activity = false;
  let launch_button_text = "Launch";

  let date = new Date();
  $: hours = add_leading_zero(date.getHours());
  $: minutes = add_leading_zero(date.getMinutes());

  function add_leading_zero(n: number): string {
    return n < 10 ? `0${n}` : n.toString();
  }

  function format_discriminator(n: number): string {
    const as_string = n.toString();
    let placeholder = "";
    for (let i = 0; i < 4 - as_string.length; i++) placeholder += "0";

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

  $: render = ({ context, width }) => {
    const lineWidth = 5;
    const radius = (width - lineWidth) / 2;
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
      context.lineCap = "square";
      context.lineWidth = lineWidth;
      context.stroke();
    };
    drawCircle("#fefefe", 1);
    drawCircle("#f5de47", date.getSeconds() / 60);
  };

  let getCanvas;
  onMount(async () => {
    getCanvas().style.setProperty("transform", "rotateZ(-90deg)");

    const data = await invoke("user", {});
    user = data;
    console.log(user);
  });

  const timeInterval = setInterval(() => (date = new Date()), 500);

  onDestroy(() => {
    clearInterval(timeInterval);
  });
</script>

<main class="main-view">
  <div class="profile-info">
    <div class="timer-container">
      <Canvas width={100} height={100} bind:getCanvas>
        <Layer {render} />
      </Canvas>
      <span class="clock">{hours}:{minutes}</span>
      <span class="clock-undertext">HR</span>
    </div>
    {#if user}
      <div class="user-info-container">
        <span class="user-name"
          >Dr. {user.name} #{format_discriminator(user.discriminator)}</span
        >
        <!-- TODO: Implement showing important log messages here-->
        <div class="message-box">
          <svg height="10" width="16" class="message-box-arrow">
            <polygon points="0,0 8,10 16,0" />
          </svg>
          <span>LOG: sample</span>
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
    width: 100%;
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
  }

  .main-buttons {
    display: flex;
    padding: 0.5rem;
    gap: 0.2rem;
    transform: skew(1deg, 2deg);
    user-select: none;
  }

  .skew-to-hover {
    transform: skew(var(--menu-skew-x), var(--menu-skew-y));
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
