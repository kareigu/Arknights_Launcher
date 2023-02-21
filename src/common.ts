import { writable } from "svelte/store";

export enum View {
  Main,
  Options,
}

export interface Options {
  executable_path: string,
  background: {
    Custom?: BackgroundComponents,
    Default?: BackgroundComponents,
  }
}

export interface BackgroundComponents {
  background: string,
  character: string,
}

export interface User {
  discriminator: number,
  id: number,
  name: string,
}

export interface LogMessage {
  msg_type: "Info" | "Warn" | "Error",
  msg: string,
}

export function format_log_message(msg: LogMessage): string {
  return `${msg.msg_type.toUpperCase()}: ${msg.msg}`;
}

export const CurrentView = writable(View.Main);
export const Log = writable([]);