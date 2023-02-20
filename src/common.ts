import { writable } from "svelte/store";

export enum View {
  Main,
  Options,
}

export interface Options {
  executable_path: string,
  background: {
    Custom?: string,
    Default?: string,
  }
}

export interface User {
  discriminator: number,
  id: number,
  name: string,
}

export const CurrentView = writable(View.Main);