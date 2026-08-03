import { useEffect, useState, type ReactNode } from "react";
import * as api from "../api";
import { errorMessage, formatAge, formatCount } from "../api";
import type { AdblockStatus, Capabilities, Preferences } from "../types";
import { applyTheme } from "../theme";
import { Modal } from "./Modal";
import { Select } from "./Select";
import { LOCALES } from "../i18n";
import { useI18n } from "../i18n/useI18n";
import {
  IconCheck,
  IconDatabase,
  IconFolder,
  IconRefresh,
  IconShield,
  IconSliders,
  IconInfo,
} from "./Icons";
import brandIcon from "../assets-icon.png";

interface Props {
  status: AdblockStatus | null;
  dataDir: string;
  onStatus: (status: AdblockStatus) => void;
  onError: (message: string) => void;
  onOk: (message: string) => void;

  onTour: () => void;
  onClose: () => void;
}

type Pane = "general" | "blocking" | "storage" | "about";

const PANE_ICONS: {
  id: Pane;
  icon: ReactNode;
  key:
    | "settings.general"
    | "settings.blocking"
    | "settings.storage"
    | "settings.about";
}[] = [
  { id: "general", icon: <IconSliders />, key: "settings.general" },
  { id: "blocking", icon: <IconShield />, key: "settings.blocking" },
  { id: "storage", icon: <IconDatabase />, key: "settings.storage" },
  { id: "about", icon: <IconInfo />, key: "settings.about" },
];

const LINKS = {
  author: "https://github.com/TheHolyOneZ",
  source: "https://github.com/TheHolyOneZ/ZWebWrap",
  projects: "https://zsync.eu",
  downloads: "https://zsync.eu/zwebwrap/",
} as const;

const TAGS = [
  "Rust",
  "Tauri 2",
  "React 19",
  "TypeScript",
  "WebKitGTK",
  "WebView2",
  "Vite",
  "no Electron",
] as const;

function Toggle({
  title,
  hint,
  checked,
  disabled,
  onChange,
}: {
  title: string;
  hint: ReactNode;
  checked: boolean;
  disabled?: boolean;
  onChange: (next: boolean) => void;
}) {
  return (
    <div className="pref">
      <div className="pref__text">
        <div className="pref__title">{title}</div>
        <div className="pref__hint">{hint}</div>
      </div>
      <label className="switch">
        <input
          type="checkbox"
          checked={checked}
          disabled={disabled}
          onChange={(e) => onChange(e.target.checked)}
        />
        <span className="switch__track" />
      </label>
    </div>
  );
}

export function Settings({
  status,
  dataDir,
  onStatus,
  onError,
  onOk,
  onTour,
  onClose,
}: Props) {
  const { t, preference, setLanguage } = useI18n();
  const [pane, setPane] = useState<Pane>("general");
  const [refreshing, setRefreshing] = useState(false);
  const [prefs, setPrefs] = useState<Preferences | null>(null);
  const [caps, setCaps] = useState<Capabilities | null>(null);
  const [hidden, setHidden] = useState(0);

  useEffect(() => {
    const read = () =>
      api
        .hiddenCounts()
        .then((c) => setHidden(c.total))
        .catch(() => {});
    read();
    const timer = setInterval(read, 3000);
    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    api
      .platformReport()
      .then(setCaps)
      .catch(() => {});
  }, []);

  useEffect(() => {
    (async () => {
      try {
        const loaded = await api.getPreferences();

        const active = await api
          .autostartActive()
          .catch(() => loaded.start_with_os);
        setPrefs({ ...loaded, start_with_os: active });
      } catch {}
    })();
  }, []);

  const updatePrefs = async (next: Preferences) => {
    const previous = prefs;
    setPrefs(next);

    applyTheme(next.theme);
    try {
      await api.setPreferences(next);
    } catch (e) {
      setPrefs(previous);
      onError(errorMessage(e));
    }
  };

  const refresh = async () => {
    setRefreshing(true);
    try {
      const next = await api.refreshFilters();
      onStatus(next);
      onOk(t("settings.updated", { count: formatCount(next.network_rules) }));
    } catch (e) {
      onError(errorMessage(e));
    } finally {
      setRefreshing(false);
    }
  };

  const set = <K extends keyof Preferences>(key: K, value: Preferences[K]) =>
    prefs && void updatePrefs({ ...prefs, [key]: value });

  return (
    <Modal
      title={t("settings.title")}
      subtitle={t("settings.subtitle")}
      wide
      onClose={onClose}
      footer={
        <>
          <span className="modal__note">{t("settings.autosaved")}</span>
          <div className="spacer" />
          <button className="btn btn--primary" onClick={onClose}>
            {t("settings.done")}
          </button>
        </>
      }
    >
      <div className="prefs">
        <nav className="prefs__nav" aria-label={t("settings.sections")}>
          {PANE_ICONS.map((p) => (
            <button
              key={p.id}
              className={`navitem ${pane === p.id ? "is-active" : ""}`}
              onClick={() => setPane(p.id)}
              aria-current={pane === p.id}
            >
              {p.icon}
              <span>{t(p.key)}</span>
            </button>
          ))}
        </nav>

        <div className="prefs__pane">
          {pane === "general" && (
            <>
              <section className="panel">
                <h3 className="panel__title">{t("settings.startClose")}</h3>
                <Toggle
                  title={t("settings.autostart")}
                  hint={t("settings.autostart.hint")}
                  checked={prefs?.start_with_os ?? false}
                  disabled={!prefs}
                  onChange={(v) => set("start_with_os", v)}
                />
                <Toggle
                  title={t("settings.tray")}
                  hint={t("settings.tray.hint")}
                  checked={prefs?.close_to_tray ?? false}
                  disabled={!prefs}
                  onChange={(v) => set("close_to_tray", v)}
                />
                {prefs && (prefs.close_to_tray || prefs.start_with_os) && (
                  <p className="panel__foot">{t("settings.tray.note")}</p>
                )}
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.language")}</h3>

                <div className="pref">
                  <div className="pref__text">
                    <div className="pref__lead">
                      {t("settings.language.hint")}
                    </div>
                  </div>
                  <Select
                    ariaLabel={t("settings.language")}
                    value={preference ?? ""}
                    onChange={(v) => void setLanguage(v || null)}
                    options={[
                      { value: "", label: t("settings.language.system") },

                      ...Object.entries(LOCALES).map(([code, l]) => ({
                        value: code,
                        label: l.label,
                      })),
                    ]}
                  />
                </div>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.theme")}</h3>
                <div className="pref">
                  <div className="pref__text">
                    <div className="pref__lead">{t("settings.theme.hint")}</div>
                  </div>
                  <Select
                    ariaLabel={t("settings.theme")}
                    value={prefs?.theme ?? ""}
                    onChange={(v) => set("theme", v || null)}
                    options={[
                      { value: "", label: t("settings.language.system") },
                      { value: "light", label: t("settings.theme.light") },
                      { value: "dark", label: t("settings.theme.dark") },
                    ]}
                  />
                </div>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.downloads")}</h3>
                <Toggle
                  title={t("settings.askWhereToSave")}
                  hint={t("settings.askWhereToSave.hint")}
                  checked={prefs?.ask_where_to_save ?? true}
                  disabled={!prefs}
                  onChange={(v) => set("ask_where_to_save", v)}
                />
              </section>

              <section className="panel">
                <div className="panel__head">
                  <div>
                    <h3 className="panel__title">{t("settings.tour")}</h3>
                    <p className="panel__sub">{t("settings.tour.hint")}</p>
                  </div>
                  <button className="btn" onClick={onTour}>
                    {t("settings.tour.button")}
                  </button>
                </div>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.windows")}</h3>
                <Toggle
                  title={t("settings.fullscreen")}
                  hint={t("settings.fullscreen.hint")}
                  checked={prefs?.contain_fullscreen ?? true}
                  disabled={!prefs}
                  onChange={(v) => set("contain_fullscreen", v)}
                />
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.newApps")}</h3>
                <Toggle
                  title={t("settings.desktopShortcut")}
                  hint={t("settings.desktopShortcut.hint")}
                  checked={prefs?.desktop_shortcut ?? true}
                  disabled={!prefs}
                  onChange={(v) => set("desktop_shortcut", v)}
                />
                <Toggle
                  title={t("settings.adblockDefault")}
                  hint={t("settings.adblockDefault.hint")}
                  checked={prefs?.adblock_by_default ?? true}
                  disabled={!prefs}
                  onChange={(v) => set("adblock_by_default", v)}
                />
              </section>
            </>
          )}

          {pane === "blocking" && (
            <>
              {caps && !caps.ad_blocking && (
                <div className="note note--warn">
                  {caps.limitations.find((l) =>
                    l.toLowerCase().includes("ad blocking"),
                  ) ??
                    "Network ad blocking is only available on Linux for now. Cosmetic hiding still applies here."}
                </div>
              )}

              <section className="metrics">
                <div className="metric">
                  <div className="metric__value">
                    {status ? formatCount(status.network_rules) : "—"}
                  </div>
                  <div className="metric__label">
                    {t("settings.metric.rules")}
                  </div>
                </div>
                <div className="metric">
                  <div className="metric__value">
                    {status ? status.lists_loaded : "—"}
                  </div>
                  <div className="metric__label">
                    {t("settings.metric.lists")}
                  </div>
                </div>
                <div className="metric">
                  <div className="metric__value">
                    {status ? formatAge(status.age_secs) : "—"}
                  </div>
                  <div className="metric__label">
                    {t("settings.metric.updated")}
                  </div>
                </div>

                <div className="metric">
                  <div className="metric__value">{formatCount(hidden)}</div>
                  <div className="metric__label">
                    {t("settings.metric.hidden")}
                  </div>
                </div>
              </section>

              {status && !status.ready && (
                <div className="note note--warn">{t("settings.notReady")}</div>
              )}

              {status?.seed_only && (
                <div className="note note--warn">{t("settings.seedOnly")}</div>
              )}

              <section className="panel">
                <div className="panel__head">
                  <div>
                    <h3 className="panel__title">{t("settings.lists")}</h3>
                    <p className="panel__sub">{t("settings.lists.hint")}</p>
                  </div>
                  <button
                    className="btn"
                    onClick={refresh}
                    disabled={refreshing}
                  >
                    <IconRefresh className={refreshing ? "spin" : undefined} />
                    {refreshing ? t("settings.updating") : t("settings.update")}
                  </button>
                </div>

                <ul className="sources">
                  {(status?.sources ?? []).map((source) => (
                    <li key={source.name} className="source">
                      <span
                        className={`source__dot ${source.cached ? "is-on" : ""}`}
                      >
                        {source.cached && <IconCheck />}
                      </span>
                      <span className="source__text">
                        <span className="source__name">{source.name}</span>
                        <span className="source__desc">
                          {source.description}
                        </span>
                      </span>
                    </li>
                  ))}
                </ul>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("settings.blocked")}</h3>
                <p className="panel__sub">{t("settings.blocked.body")}</p>
              </section>
            </>
          )}

          {pane === "storage" && (
            <section className="panel">
              <div className="panel__head">
                <div>
                  <h3 className="panel__title">{t("settings.dataFolder")}</h3>
                  <p className="panel__sub">{t("settings.dataFolder.hint")}</p>
                </div>
                <button
                  className="btn"
                  onClick={() => api.revealDataDir().catch(() => {})}
                >
                  <IconFolder />
                  {t("settings.open")}
                </button>
              </div>
              <input
                className="input input--mono"
                value={dataDir}
                readOnly
                tabIndex={-1}
              />
            </section>
          )}

          {pane === "about" && (
            <>
              <section className="panel about">
                <div className="about__head">
                  <img
                    className="about__mark"
                    src={brandIcon}
                    alt=""
                    width={54}
                    height={54}
                  />
                  <div>
                    <h3 className="about__name">ZWebWrap</h3>
                    <p className="about__version">
                      {t("about.version", { version: "0.1.0" })}
                    </p>
                  </div>
                </div>
                <p className="about__body">{t("about.description")}</p>
                <div className="about__tags">
                  {TAGS.map((tag) => (
                    <span key={tag} className="about__tag">
                      {tag}
                    </span>
                  ))}
                </div>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("about.author")}</h3>
                <p className="about__line">TheHolyOneZ</p>
                <h3 className="panel__title about__spaced">
                  {t("about.licence")}
                </h3>
                <p className="about__line">GPL-3.0-or-later</p>
                <p className="field__hint">{t("about.licence.hint")}</p>
              </section>

              <section className="panel">
                <h3 className="panel__title">{t("about.links")}</h3>
                <div className="about__links">
                  <button
                    className="btn"
                    onClick={() => void api.openLink(LINKS.downloads)}
                  >
                    {t("about.link.downloads")}
                  </button>
                  <button
                    className="btn"
                    onClick={() => void api.openLink(LINKS.source)}
                  >
                    {t("about.link.source")}
                  </button>
                  <button
                    className="btn"
                    onClick={() => void api.openLink(LINKS.projects)}
                  >
                    {t("about.link.projects")}
                  </button>
                  <button
                    className="btn"
                    onClick={() => void api.openLink(LINKS.author)}
                  >
                    {t("about.link.author")}
                  </button>
                </div>
                <p className="field__hint">{t("about.downloads.hint")}</p>
              </section>
            </>
          )}

          {pane === "storage" && caps && (
            <section className="panel">
              <h3 className="panel__title">{t("settings.platform")}</h3>
              <p className="panel__sub">
                {caps.os} · {t("settings.platform.session")}: {caps.session}
              </p>
              {caps.limitations.length === 0 ? (
                <p className="panel__sub">{t("settings.platform.fine")}</p>
              ) : (
                <ul className="limits">
                  {caps.limitations.map((note) => (
                    <li key={note}>{note}</li>
                  ))}
                </ul>
              )}
            </section>
          )}
        </div>
      </div>
    </Modal>
  );
}
