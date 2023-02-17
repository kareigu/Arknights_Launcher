import { writable } from "svelte/store";

export enum View {
  Main,
  Options,
}

export const CurrentView = writable(View.Main);