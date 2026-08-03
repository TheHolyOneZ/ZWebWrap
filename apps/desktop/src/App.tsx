import { useCallback, useEffect, useMemo, useState } from "react";
import * as api from "./api";
import { errorMessage, formatCount, hostOf } from "./api";
import type {
  AdblockStatus,
  Download,
  MediaSupport,
  WrappedApp,
} from "./types";
import { AddAppForm } from "./components/AddAppForm";
import { AppCard } from "./components/AppCard";
import { AppDetail } from "./components/AppDetail";
import { ImportBookmarks } from "./components/ImportBookmarks";
import { classifyIcon, type IconFit } from "./iconFit";
import { applyTheme } from "./theme";
import { useT } from "./i18n/useI18n";
import { Settings } from "./components/Settings";
import { Downloads } from "./components/Downloads";
import { ConfirmDialog } from "./components/ConfirmDialog";
import { ContextMenu, type MenuItem } from "./components/ContextMenu";
import { ToastStack, useToasts } from "./components/Toasts";
import { Tour } from "./components/Tour";
import {
  IconDownload,
  IconRefresh,
  IconSearch,
  IconSettings,
  IconShield,
} from "./components/Icons";
import brandIcon from "./assets-icon.png";

export function App() {
  const t = useT();
  const [apps, setApps] = useState<WrappedApp[] | null>(null);
  const [icons, setIcons] = useState<Record<string, string | null>>({});
  const [fits, setFits] = useState<Record<string, IconFit>>({});
  const [status, setStatus] = useState<AdblockStatus | null>(null);
  const [dataDir, setDataDir] = useState("");
  const [media, setMedia] = useState<MediaSupport | null>(null);
  const [query, setQuery] = useState("");
  const [editing, setEditing] = useState<WrappedApp | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [menu, setMenu] = useState<{
    x: number;
    y: number;
    items: MenuItem[];
  } | null>(null);
  const [deleting, setDeleting] = useState<WrappedApp | null>(null);
  const [importing, setImporting] = useState(false);
  const [downloads, setDownloads] = useState<Download[]>([]);
  const [showDownloads, setShowDownloads] = useState(false);
  const [tour, setTour] = useState(false);

  const toasts = useToasts();
  const { ok, error } = toasts;

  const loadIcons = useCallback(async (list: WrappedApp[]) => {
    const entries = await Promise.all(
      list.map(
        async (app) =>
          [app.id, await api.iconDataUrl(app.id).catch(() => null)] as const,
      ),
    );
    setIcons(Object.fromEntries(entries));

    const measured = await Promise.all(
      entries.map(
        async ([id, url]) =>
          [id, url ? await classifyIcon(url) : "contain"] as const,
      ),
    );
    setFits(Object.fromEntries(measured));
  }, []);

  const refresh = useCallback(async () => {
    try {
      const list = await api.listApps();
      setApps(list);
      void loadIcons(list);
    } catch (e) {
      error(errorMessage(e));
    }
  }, [error, loadIcons]);

  useEffect(() => {
    void refresh();
    api
      .dataLocation()
      .then(setDataDir)
      .catch(() => {});
    api
      .mediaSupport()
      .then(setMedia)
      .catch(() => {});

    api
      .getPreferences()
      .then((p) => {
        applyTheme(p.theme);
        if (!p.tour_done) setTour(true);
      })
      .catch(() => {});
  }, [refresh]);

  useEffect(() => {
    let cancelled = false;
    void api
      .listDownloads()
      .then((list) => !cancelled && setDownloads(list))
      .catch(() => {});

    const unlisten = api.onDownload((record) => {
      void api
        .listDownloads()
        .then((list) => !cancelled && setDownloads(list))
        .catch(() => {});

      if (cancelled) return;
      if (record.state === "done")
        ok(t("dl.toast.done", { name: record.file_name }));
      else if (record.state === "failed")
        error(t("dl.toast.failed", { name: record.file_name }));
    });

    return () => {
      cancelled = true;
      void unlisten.then((off) => off());
    };
  }, [ok, error, t]);

  useEffect(() => {
    const apply = () => {
      const hash = window.location.hash.replace(/^#/, "");

      if (hash === "settings") {
        setEditing(null);
        setShowSettings(true);
        return;
      }
      const match = hash.match(/^app\/(.+)$/);
      if (match && apps) {
        const found = apps.find((a) => a.id === match[1]);
        if (found) {
          setShowSettings(false);
          setEditing(found);
        }
      }
    };
    apply();
    window.addEventListener("hashchange", apply);
    return () => window.removeEventListener("hashchange", apply);
  }, [apps]);

  useEffect(() => {
    let cancelled = false;
    let attempts = 0;

    const poll = async () => {
      try {
        const next = await api.adblockStatus();
        if (cancelled) return;
        setStatus(next);
        if (!next.ready && attempts++ < 20) setTimeout(poll, 1500);
      } catch {}
    };

    void poll();
    return () => {
      cancelled = true;
    };
  }, []);

  const open = async (app: WrappedApp) => {
    try {
      await api.openApp(app.id);
    } catch (e) {
      error(errorMessage(e));
    }
  };

  const save = async (next: WrappedApp) => {
    try {
      await api.updateApp(next);
      setEditing(null);
      await refresh();
      ok(t("toast.saved", { name: next.title }));
    } catch (e) {
      error(errorMessage(e));
    }
  };

  const remove = async (app: WrappedApp) => {
    try {
      await api.deleteApp(app.id);
      setEditing(null);
      await refresh();
      ok(t("toast.deleted", { name: app.title }));
    } catch (e) {
      error(errorMessage(e));
    }
  };

  const openMenuFor = (app: WrappedApp, e: React.MouseEvent) => {
    e.preventDefault();
    setMenu({
      x: e.clientX,
      y: e.clientY,
      items: [
        { label: t("menu.open"), onSelect: () => void open(app) },
        { label: t("menu.settings"), onSelect: () => setEditing(app) },
        {
          label: t("menu.copyAddress"),
          separated: true,
          onSelect: () => {
            void api
              .copyText(app.url)
              .then((done) =>
                done ? ok(t("toast.copied")) : error(t("toast.copyFailed")),
              );
          },
        },
        {
          label: t(
            app.adblock !== "off" ? "menu.adblockOff" : "menu.adblockOn",
          ),
          onSelect: () =>
            void save({
              ...app,
              adblock: app.adblock !== "off" ? "off" : "full",
            }),
        },
        {
          label: t("menu.delete"),
          danger: true,
          separated: true,
          onSelect: () => setDeleting(app),
        },
      ],
    });
  };

  const visible = useMemo(() => {
    if (!apps) return [];
    const needle = query.trim().toLowerCase();
    if (!needle) return apps;
    return apps.filter(
      (a) =>
        a.title.toLowerCase().includes(needle) ||
        hostOf(a.url).toLowerCase().includes(needle),
    );
  }, [apps, query]);

  const groups = useMemo(() => {
    const seen: string[] = [];
    for (const app of apps ?? []) {
      const name = (app.group ?? "").trim();
      if (!name) continue;
      if (!seen.some((g) => g.toLowerCase() === name.toLowerCase()))
        seen.push(name);
    }
    return seen.sort((a, b) => a.toLowerCase().localeCompare(b.toLowerCase()));
  }, [apps]);

  const sections = useMemo(() => {
    const byGroup = new Map<string, WrappedApp[]>();
    const loose: WrappedApp[] = [];

    for (const app of visible) {
      const name = (app.group ?? "").trim();
      if (!name) {
        loose.push(app);
        continue;
      }

      const key =
        [...byGroup.keys()].find(
          (k) => k.toLowerCase() === name.toLowerCase(),
        ) ?? name;
      byGroup.set(key, [...(byGroup.get(key) ?? []), app]);
    }

    const named = [...byGroup.entries()]
      .sort(([a], [b]) => a.toLowerCase().localeCompare(b.toLowerCase()))
      .map(([name, items]) => ({ name, items }));

    return loose.length ? [...named, { name: "", items: loose }] : named;
  }, [visible]);

  const blockingLabel = !status
    ? t("app.blocking.loading")
    : !status.ready
      ? t("app.blocking.preparing")
      : status.seed_only
        ? t("app.blocking.basic")
        : t("app.blocking.ready", { count: formatCount(status.network_rules) });

  return (
    <div className="shell">
      <header className="topbar">
        <div className="brand">
          <img
            className="brand__mark"
            src={brandIcon}
            alt=""
            width={30}
            height={30}
          />
          <div>
            <h1 className="brand__name">ZWebWrap</h1>
            <p className="brand__tag">{t("app.tagline")}</p>
          </div>
        </div>

        <div className="topbar__actions">
          <button
            data-tour="blocking"
            className={`pill pill--button ${status?.ready ? "pill--ok" : "pill--muted"}`}
            onClick={() => setShowSettings(true)}
            title={t("app.blocking.status")}
          >
            <IconShield />
            {blockingLabel}
          </button>
          <button
            data-tour="import"
            className="btn btn--small"
            onClick={() => setImporting(true)}
          >
            {t("app.import")}
          </button>

          {downloads.length > 0 && (
            <div className="anchor">
              <button
                className={`iconbtn ${downloads.some((d) => d.state === "active") ? "is-busy" : ""}`}
                onClick={() => {
                  void api
                    .listDownloads()
                    .then(setDownloads)
                    .catch(() => {});
                  setShowDownloads((open) => !open);
                }}
                title={t("dl.title")}
                aria-label={t("dl.title")}
              >
                <IconDownload />
              </button>

              {showDownloads && (
                <Downloads
                  downloads={downloads}
                  onChanged={setDownloads}
                  onError={error}
                  onClose={() => setShowDownloads(false)}
                />
              )}
            </div>
          )}

          <button
            data-tour="settings"
            className="iconbtn"
            onClick={() => setShowSettings(true)}
            title={t("app.settings")}
            aria-label={t("app.settings")}
          >
            <IconSettings />
          </button>
        </div>
      </header>

      {media && !media.ok && (
        <div className="alert" role="alert">
          <div className="alert__title">{t("media.title")}</div>
          <p className="alert__body">
            {t("media.body", { missing: media.missing.join(", ") })}
          </p>
          {media.install_hint && (
            <>
              <p className="alert__body">{t("media.installHint")}</p>
              <code className="alert__code">{media.install_hint}</code>
            </>
          )}
          <div className="alert__actions">
            <button
              className="btn"
              onClick={async () => {
                const next = await api.mediaSupport().catch(() => null);
                if (!next) return;
                setMedia(next);
                if (next.ok) {
                  ok(t("media.found"));
                } else {
                  error(
                    t("media.stillMissing", {
                      missing: next.missing.join(", "),
                    }),
                  );
                }
              }}
            >
              <IconRefresh />
              {t("media.recheck")}
            </button>
          </div>
        </div>
      )}

      <AddAppForm
        onCreated={async (name) => {
          await refresh();
          ok(t("toast.created", { name }));
        }}
        onError={error}
      />

      <main className="library" data-tour="library">
        {apps !== null && apps.length > 0 && (
          <div className="library__head">
            <h2 className="library__title">{t("app.library")}</h2>

            {apps.length > 4 && (
              <div className="searchbar">
                <IconSearch />
                <input
                  className="searchbar__input"
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder={t("app.search")}
                  aria-label={t("app.search")}
                />
              </div>
            )}
          </div>
        )}

        {apps === null ? (
          <div className="grid">
            {[0, 1, 2, 3, 4, 5].map((i) => (
              <div key={i} className="app app--skeleton" />
            ))}
          </div>
        ) : apps.length === 0 ? (
          <div className="empty">
            <div className="empty__mark" aria-hidden="true" />
            <p className="empty__title">{t("app.empty.title")}</p>
            <p className="empty__body">{t("app.empty.body")}</p>
          </div>
        ) : visible.length === 0 ? (
          <div className="empty">
            <p className="empty__title">{t("app.noMatch", { query })}</p>
            <button className="btn" onClick={() => setQuery("")}>
              {t("app.clearSearch")}
            </button>
          </div>
        ) : (
          sections.map((section) => (
            <section key={section.name || "__loose"} className="group">
              {section.name && (
                <h3 className="group__title">
                  {section.name}
                  <span className="group__count">{section.items.length}</span>
                </h3>
              )}
              <div className="grid">
                {section.items.map((app) => (
                  <AppCard
                    key={app.id}
                    app={app}
                    icon={icons[app.id] ?? null}
                    fit={fits[app.id] ?? "contain"}
                    onOpen={() => open(app)}
                    onEdit={() => setEditing(app)}
                    onContextMenu={(e) => openMenuFor(app, e)}
                  />
                ))}
              </div>
            </section>
          ))
        )}
      </main>

      {editing && (
        <AppDetail
          app={editing}
          groups={groups}
          icon={icons[editing.id] ?? null}
          onIconChanged={(id, dataUrl) =>
            setIcons((prev) => ({ ...prev, [id]: dataUrl }))
          }
          onError={error}
          onSave={save}
          onDelete={() => remove(editing)}
          onClose={() => setEditing(null)}
        />
      )}

      {deleting && (
        <ConfirmDialog
          title={t("confirm.delete.title", { name: deleting.title })}
          body={t("confirm.delete.body")}
          confirmLabel={t("detail.delete")}
          onConfirm={async () => {
            const target = deleting;
            setDeleting(null);
            await remove(target);
          }}
          onCancel={() => setDeleting(null)}
        />
      )}

      {importing && (
        <ImportBookmarks
          onError={error}
          onClose={() => setImporting(false)}
          onDone={async (created) => {
            setImporting(false);
            await refresh();
            ok(t("import.done", { n: created }));
          }}
        />
      )}

      {tour && (
        <Tour
          onDone={() => {
            setTour(false);

            void api
              .getPreferences()
              .then((p) => api.setPreferences({ ...p, tour_done: true }))
              .catch(() => {});
          }}
        />
      )}

      {showSettings && (
        <Settings
          status={status}
          dataDir={dataDir}
          onStatus={setStatus}
          onError={error}
          onOk={ok}
          onTour={() => {
            setShowSettings(false);
            setTour(true);
          }}
          onClose={() => setShowSettings(false)}
        />
      )}

      {menu && (
        <ContextMenu
          x={menu.x}
          y={menu.y}
          items={menu.items}
          onClose={() => setMenu(null)}
        />
      )}

      <ToastStack toasts={toasts.toasts} onDismiss={toasts.dismiss} />
    </div>
  );
}
