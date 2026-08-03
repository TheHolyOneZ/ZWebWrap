const HOST = "com.zwebwrap.bridge";

export const PROTOCOL_VERSION = 1;

export interface AppSummary {
  id: string;
  title: string;
  url: string;
  icon: string | null;
  adblock: boolean;
}

type Request =
  | { type: "ping" }
  | { type: "list_apps" }
  | { type: "create_app"; url: string; title?: string; adblock?: boolean }
  | { type: "open_app"; id: string }
  | { type: "remove_app"; id: string };

type Response =
  | { type: "pong"; version: number; app_version: string }
  | { type: "apps"; apps: AppSummary[] }
  | { type: "created"; app: AppSummary }
  | { type: "ok" }
  | { type: "error"; message: string };

export class NotConnected extends Error {
  constructor(detail: string) {
    super(detail);
    this.name = "NotConnected";
  }
}

function timeoutFor(request: Request): number {
  return request.type === "create_app" ? 90_000 : 15_000;
}

function send(request: Request): Promise<Response> {
  return new Promise((resolve, reject) => {
    let settled = false;
    const done = (fn: () => void) => {
      if (settled) return;
      settled = true;
      fn();
    };

    const timer = setTimeout(
      () =>
        done(() => reject(new NotConnected("The desktop app did not answer."))),
      timeoutFor(request),
    );

    try {
      chrome.runtime.sendNativeMessage(HOST, request, (response) => {
        clearTimeout(timer);
        const failure = chrome.runtime.lastError;
        if (failure) {
          return done(() =>
            reject(new NotConnected(failure.message ?? "No connection.")),
          );
        }
        if (!response) {
          return done(() =>
            reject(new NotConnected("The desktop app sent no reply.")),
          );
        }
        done(() => resolve(response as Response));
      });
    } catch (e) {
      clearTimeout(timer);
      done(() =>
        reject(new NotConnected(e instanceof Error ? e.message : String(e))),
      );
    }
  });
}

async function expect<T extends Response["type"]>(
  request: Request,
  wanted: T,
): Promise<Extract<Response, { type: T }>> {
  const response = await send(request);
  if (response.type === "error") throw new Error(response.message);
  if (response.type !== wanted) {
    throw new Error(
      `The app replied with "${response.type}" instead of "${wanted}".`,
    );
  }
  return response as Extract<Response, { type: T }>;
}

export interface Connection {
  appVersion: string;

  mismatched: boolean;
}

export async function connect(): Promise<Connection> {
  const pong = await expect({ type: "ping" }, "pong");
  return {
    appVersion: pong.app_version,
    mismatched: pong.version !== PROTOCOL_VERSION,
  };
}

export async function listApps(): Promise<AppSummary[]> {
  return (await expect({ type: "list_apps" }, "apps")).apps;
}

export async function createApp(
  url: string,
  title?: string,
  adblock?: boolean,
): Promise<AppSummary> {
  return (await expect({ type: "create_app", url, title, adblock }, "created"))
    .app;
}

export async function openApp(id: string): Promise<void> {
  await expect({ type: "open_app", id }, "ok");
}

export async function removeApp(id: string): Promise<void> {
  await expect({ type: "remove_app", id }, "ok");
}

export function hostOf(url: string): string {
  try {
    return new URL(url).host.replace(/^www\./, "");
  } catch {
    return url;
  }
}

export function wrappable(url: string | undefined): boolean {
  if (!url) return false;
  return /^https?:\/\//i.test(url);
}
