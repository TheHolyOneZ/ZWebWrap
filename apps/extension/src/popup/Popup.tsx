import { useCallback, useEffect, useState } from "react";
import * as bridge from "../bridge";
import type { AppSummary } from "../bridge";

type Stage =
  | { at: "checking" }
  | { at: "no-app"; detail: string }
  | { at: "ready"; version: string; mismatched: boolean };

interface Tab {
  url: string;
  title: string;
}

export function Popup() {
  const [stage, setStage] = useState<Stage>({ at: "checking" });
  const [tab, setTab] = useState<Tab | null>(null);
  const [apps, setApps] = useState<AppSummary[]>([]);
  const [name, setName] = useState("");
  const [adblock, setAdblock] = useState(true);
  const [busy, setBusy] = useState(false);
  const [note, setNote] = useState<{ kind: "ok" | "bad"; text: string } | null>(
    null,
  );

  const [faviconFailed, setFaviconFailed] = useState(false);

  const refresh = useCallback(async () => {
    try {
      setApps(await bridge.listApps());
    } catch {}
  }, []);

  useEffect(() => {
    chrome.tabs.query({ active: true, currentWindow: true }, ([current]) => {
      if (current?.url) {
        setTab({ url: current.url, title: current.title ?? "" });
        setName(current.title ?? "");
      } else {
        setNote({
          kind: "bad",
          text: "Could not read the current tab. Reload the extension and try again.",
        });
      }
    });

    bridge
      .connect()
      .then((c) => {
        setStage({
          at: "ready",
          version: c.appVersion,
          mismatched: c.mismatched,
        });
        void refresh();
      })
      .catch((e) => setStage({ at: "no-app", detail: e.message }));
  }, [refresh]);

  const alreadyWrapped = tab
    ? apps.find((a) => bridge.hostOf(a.url) === bridge.hostOf(tab.url))
    : undefined;

  const create = async () => {
    if (!tab || busy) return;
    setBusy(true);
    setNote(null);
    try {
      const app = await bridge.createApp(
        tab.url,
        name.trim() || undefined,
        adblock,
      );
      setNote({
        kind: "ok",
        text: `Created ${app.title} — it's in your app menu and on your desktop.`,
      });
      await refresh();
    } catch (e) {
      setNote({
        kind: "bad",
        text: e instanceof Error ? e.message : String(e),
      });
    } finally {
      setBusy(false);
    }
  };

  const launch = async (app: AppSummary) => {
    try {
      await bridge.openApp(app.id);
      window.close();
    } catch (e) {
      setNote({
        kind: "bad",
        text: e instanceof Error ? e.message : String(e),
      });
    }
  };

  if (stage.at === "checking") {
    return (
      <div className="pop">
        <div className="skeleton" />
        <div className="skeleton" />
      </div>
    );
  }

  if (stage.at === "no-app") {
    return (
      <div className="pop">
        <header className="head">
          <span className="mark" />
          <span className="wordmark">ZWebWrap</span>
        </header>
        <div className="empty">
          <p className="empty__title">The desktop app isn&apos;t running</p>
          <p className="empty__body">
            This extension is a shortcut to ZWebWrap — it needs the app
            installed to create anything.
          </p>
          <code className="detail">{stage.detail}</code>
          <a
            className="btn btn--primary"
            href="https://github.com/theholyonez/ZWebWrap#building-and-running"
            target="_blank"
            rel="noreferrer"
          >
            Get ZWebWrap
          </a>
          <button className="btn btn--quiet" onClick={() => location.reload()}>
            Check again
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="pop">
      <header className="head">
        <span className="mark" />
        <span className="wordmark">ZWebWrap</span>
        <span className="spacer" />
        <span className="version">v{stage.version}</span>
      </header>

      {stage.mismatched && (
        <div className="warn">
          The app and this extension are different versions. Update whichever is
          older.
        </div>
      )}

      <section className="current">
        {!tab || !bridge.wrappable(tab.url) ? (
          <p className="empty__body">
            This page can&apos;t become an app. Open an ordinary website and try
            again.
          </p>
        ) : (
          <>
            <div className="site">
              {faviconFailed ? (
                <span className="favicon favicon--blank" aria-hidden="true">
                  {bridge.hostOf(tab.url).slice(0, 1).toUpperCase()}
                </span>
              ) : (
                <img
                  className="favicon"
                  src={`https://${bridge.hostOf(tab.url)}/favicon.ico`}
                  alt=""
                  width={28}
                  height={28}
                  onError={() => setFaviconFailed(true)}
                />
              )}
              <div className="site__text">
                <input
                  className="site__name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="App name"
                  aria-label="App name"
                />
                <span className="site__host">{bridge.hostOf(tab.url)}</span>
              </div>
            </div>

            <button
              className="btn btn--primary"
              onClick={create}
              disabled={busy}
            >
              {busy ? "Creating…" : "Create desktop app"}
            </button>

            {alreadyWrapped && (
              <button className="btn" onClick={() => launch(alreadyWrapped)}>
                Open {alreadyWrapped.title}
              </button>
            )}

            <label className="check">
              <input
                type="checkbox"
                checked={adblock}
                onChange={(e) => setAdblock(e.target.checked)}
              />
              <span>Block ads and trackers</span>
            </label>
          </>
        )}
      </section>

      {note && <div className={`note note--${note.kind}`}>{note.text}</div>}

      {apps.length > 0 && (
        <section className="library">
          <div className="library__label">Your apps</div>
          <ul className="list">
            {apps.map((app) => (
              <li key={app.id}>
                <button
                  className="row"
                  onClick={() => launch(app)}
                  title={`Open ${app.title}`}
                >
                  {app.icon ? (
                    <img
                      className="row__icon"
                      src={app.icon}
                      alt=""
                      width={20}
                      height={20}
                    />
                  ) : (
                    <span className="row__icon row__icon--blank">
                      {app.title.slice(0, 1).toUpperCase()}
                    </span>
                  )}
                  <span className="row__title">{app.title}</span>
                  <span className="row__host">{bridge.hostOf(app.url)}</span>
                  <span className="row__go">↗</span>
                </button>
              </li>
            ))}
          </ul>
        </section>
      )}
    </div>
  );
}
