import { useEffect, useRef, useState, type ReactNode } from "react";
import * as api from "../api";
import { errorMessage, hostOf } from "../api";
import type { Permissions, Rule, UserAgentPreset, WrappedApp } from "../types";
import { classifyIcon, type IconFit } from "../iconFit";
import { Modal } from "./Modal";
import { Select } from "./Select";
import {
  IconEdit,
  IconExternal,
  IconShield,
  IconSliders,
  IconTrash,
} from "./Icons";
import { useT } from "../i18n/useI18n";

type Pane = "general" | "behaviour" | "appearance" | "advanced";

const PANE_ICONS: {
  id: Pane;
  icon: ReactNode;
  key:
    | "detail.general"
    | "detail.behaviour"
    | "detail.appearance"
    | "detail.advanced";
}[] = [
  { id: "general", icon: <IconEdit />, key: "detail.general" },
  { id: "behaviour", icon: <IconShield />, key: "detail.behaviour" },
  { id: "appearance", icon: <IconSliders />, key: "detail.appearance" },
  { id: "advanced", icon: <IconExternal />, key: "detail.advanced" },
];

interface Props {
  app: WrappedApp;

  groups: string[];
  icon: string | null;
  onIconChanged: (id: string, dataUrl: string | null) => void;
  onError: (message: string) => void;
  onSave: (app: WrappedApp) => Promise<void>;
  onDelete: () => Promise<void>;
  onClose: () => void;
}

export function AppDetail({
  app,
  groups,
  icon,
  onIconChanged,
  onError,
  onSave,
  onDelete,
  onClose,
}: Props) {
  const t = useT();
  const [pane, setPane] = useState<Pane>("general");
  const [draft, setDraft] = useState<WrappedApp>(app);
  const [busy, setBusy] = useState(false);
  const [confirmingDelete, setConfirmingDelete] = useState(false);

  const [login, setLogin] = useState({ username: "", password: "" });
  const [proxyError, setProxyError] = useState<string | null>(null);
  const [presets, setPresets] = useState<UserAgentPreset[]>([]);
  const [shownIcon, setShownIcon] = useState<string | null>(icon);

  const [onDesktop, setOnDesktop] = useState<boolean | null>(null);

  const [globalFullscreen, setGlobalFullscreen] = useState(true);
  const [recording, setRecording] = useState(false);
  const [hotkeyBad, setHotkeyBad] = useState(false);
  const [iconBusy, setIconBusy] = useState(false);

  const [fit, setFit] = useState<IconFit>("contain");
  const fileInput = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!shownIcon) return;
    let cancelled = false;
    void classifyIcon(shownIcon).then((next) => {
      if (!cancelled) setFit(next);
    });
    return () => {
      cancelled = true;
    };
  }, [shownIcon]);

  const pickIcon = async (file: File) => {
    setIconBusy(true);
    try {
      const dataUrl = await new Promise<string>((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result));
        reader.onerror = () => reject(new Error("that file could not be read"));
        reader.readAsDataURL(file);
      });

      const stored = await api.setAppIcon(app.id, dataUrl);
      setShownIcon(stored);
      onIconChanged(app.id, stored);
    } catch (e) {
      onError(errorMessage(e));
    } finally {
      setIconBusy(false);
    }
  };

  const restoreIcon = async () => {
    setIconBusy(true);
    try {
      const stored = await api.resetAppIcon(app.id);
      setShownIcon(stored);
      onIconChanged(app.id, stored);
    } catch (e) {
      onError(errorMessage(e));
    } finally {
      setIconBusy(false);
    }
  };

  useEffect(() => {
    api
      .userAgentPresets()
      .then(setPresets)
      .catch(() => setPresets([]));
    api
      .getPreferences()
      .then((p) => setGlobalFullscreen(p.contain_fullscreen))
      .catch(() => {});
  }, []);

  useEffect(() => {
    if (!app.has_auth) {
      setLogin({ username: "", password: "" });
      return;
    }
    api
      .appCredential(app.id)
      .then((stored) => setLogin(stored ?? { username: "", password: "" }))

      .catch(() => setLogin({ username: "", password: "" }));
  }, [app.id, app.has_auth]);

  useEffect(() => {
    api
      .hasDesktopShortcut(app.id)

      .then(setOnDesktop)
      .catch(() => setOnDesktop(false));
  }, [app.id]);

  const set = <K extends keyof WrappedApp>(key: K, value: WrappedApp[K]) =>
    setDraft((d) => ({ ...d, [key]: value }));

  const save = async () => {
    setBusy(true);
    try {
      const proxy = (draft.proxy ?? "").trim();

      if (proxy) {
        try {
          await api.validProxy(proxy);
        } catch (e) {
          setProxyError(errorMessage(e));
          setPane("behaviour");
          return;
        }
      }
      setProxyError(null);

      const hasAuth = login.username.trim() !== "";

      await onSave({ ...draft, proxy: proxy || null, has_auth: hasAuth });
      await api.setAppCredential(app.id, hasAuth ? login : null);
    } catch (e) {
      onError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  };

  const remove = async () => {
    setBusy(true);
    try {
      await onDelete();
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      title={app.title}
      subtitle={hostOf(app.url)}
      wide
      onClose={onClose}
      footer={
        <>
          {confirmingDelete ? (
            <div className="confirm">
              <span>{t("detail.delete.confirm")}</span>
              <button
                className="btn btn--danger"
                onClick={remove}
                disabled={busy}
              >
                {t("detail.delete")}
              </button>
              <button
                className="btn"
                onClick={() => setConfirmingDelete(false)}
                disabled={busy}
              >
                {t("detail.cancel")}
              </button>
            </div>
          ) : (
            <button
              className="btn btn--ghost-danger"
              onClick={() => setConfirmingDelete(true)}
              disabled={busy}
            >
              <IconTrash />
              {t("detail.delete")}
            </button>
          )}

          <div className="spacer" />

          <button className="btn" onClick={onClose} disabled={busy}>
            {t("detail.cancel")}
          </button>
          <button className="btn btn--primary" onClick={save} disabled={busy}>
            {busy ? t("detail.saving") : t("detail.save")}
          </button>
        </>
      }
    >
      <div className="prefs">
        <nav className="prefs__nav" aria-label={t("detail.sections")}>
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
            <section className="panel">
              <div className="detail__identity">
                <div className="iconpick">
                  {shownIcon ? (
                    <span className={`detail__icon detail__icon--${fit}`}>
                      <img src={shownIcon} alt="" draggable={false} />
                    </span>
                  ) : (
                    <span className="detail__icon detail__icon--contain detail__icon--fallback">
                      {app.title.slice(0, 1).toUpperCase()}
                    </span>
                  )}
                  <input
                    ref={fileInput}
                    type="file"
                    accept="image/png,image/jpeg,image/webp,image/x-icon,image/gif,image/svg+xml"
                    hidden
                    onChange={(e) => {
                      const file = e.target.files?.[0];
                      e.target.value = "";
                      if (file) void pickIcon(file);
                    }}
                  />
                  <div className="iconpick__actions">
                    <button
                      type="button"
                      className="btn btn--tiny"
                      onClick={() => fileInput.current?.click()}
                      disabled={iconBusy}
                    >
                      {iconBusy
                        ? t("detail.icon.saving")
                        : t("detail.icon.change")}
                    </button>
                    <button
                      type="button"
                      className="btn btn--tiny btn--quiet"
                      onClick={restoreIcon}
                      disabled={iconBusy}
                      title={t("detail.icon.reset.title")}
                    >
                      {t("detail.icon.reset")}
                    </button>
                  </div>
                </div>
                <label className="field field--grow">
                  <span className="field__label">{t("detail.name")}</span>
                  <input
                    className="input"
                    value={draft.title}
                    onChange={(e) => set("title", e.target.value)}
                    placeholder={t("detail.name")}
                  />
                  <span className="field__hint">{t("detail.name.hint")}</span>
                </label>
              </div>

              <label className="field">
                <span className="field__label">{t("detail.address")}</span>
                <input
                  className="input"
                  value={draft.url}
                  onChange={(e) => set("url", e.target.value)}
                  spellCheck={false}
                />
              </label>
            </section>
          )}

          {pane === "behaviour" && (
            <section className="panel">
              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.adblock")}</div>
                  <div className="pref__hint">
                    {t(
                      draft.adblock === "network_only"
                        ? "detail.adblock.hint.networkOnly"
                        : draft.adblock === "off"
                          ? "detail.adblock.hint.off"
                          : "detail.adblock.hint",
                    )}
                  </div>
                </div>
                <Select
                  ariaLabel={t("detail.adblock")}
                  value={draft.adblock}
                  onChange={(v) => set("adblock", v as WrappedApp["adblock"])}
                  options={[
                    { value: "full", label: t("detail.adblock.full") },
                    {
                      value: "network_only",
                      label: t("detail.adblock.networkOnly"),
                    },
                    { value: "off", label: t("detail.adblock.off") },
                  ]}
                />
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.shortcut")}</div>
                  <div className="pref__hint">
                    {t(
                      onDesktop === null
                        ? "detail.shortcut.checking"
                        : "detail.shortcut.hint",
                    )}
                  </div>
                </div>
                <label className="switch">
                  <input
                    type="checkbox"
                    checked={onDesktop ?? false}
                    disabled={onDesktop === null}
                    onChange={(e) => {
                      setOnDesktop(e.target.checked);
                      set("desktop_shortcut", e.target.checked);
                    }}
                  />
                  <span className="switch__track" />
                </label>
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.fullscreen")}</div>
                  <div className="pref__hint">
                    {t("detail.fullscreen.hint")}
                  </div>
                </div>
                <label className="switch">
                  <input
                    type="checkbox"
                    checked={draft.contain_fullscreen ?? globalFullscreen}
                    onChange={(e) =>
                      set("contain_fullscreen", e.target.checked)
                    }
                  />
                  <span className="switch__track" />
                </label>
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.frame")}</div>
                  <div className="pref__hint">{t("detail.frame.hint")}</div>
                </div>
                <Select
                  ariaLabel={t("detail.frame")}
                  value={draft.frame}
                  onChange={(v) => set("frame", v as WrappedApp["frame"])}
                  options={[
                    { value: "normal", label: t("detail.frame.normal") },
                    { value: "no_border", label: t("detail.frame.noBorder") },
                    { value: "frameless", label: t("detail.frame.frameless") },
                  ]}
                />
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">
                    {t("detail.offsite", { host: hostOf(app.url) })}
                  </div>
                  <div className="pref__hint">{t("detail.offsite.hint")}</div>
                </div>
                <Select
                  ariaLabel={t("detail.offsite.aria")}
                  value={draft.offsite}
                  onChange={(v) => set("offsite", v)}
                  options={[
                    { value: "browser", label: t("detail.offsite.browser") },
                    { value: "in_window", label: t("detail.offsite.inWindow") },
                  ]}
                />
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.session")}</div>
                  <div className="pref__hint">
                    {t(
                      draft.session === "isolated"
                        ? "detail.session.hint.isolated"
                        : "detail.session.hint",
                    )}
                  </div>
                </div>
                <Select
                  ariaLabel={t("detail.session")}
                  value={draft.session}
                  onChange={(v) => set("session", v as WrappedApp["session"])}
                  options={[
                    { value: "shared", label: t("detail.session.shared") },
                    { value: "isolated", label: t("detail.session.isolated") },
                  ]}
                />
              </div>

              <label className="field">
                <span className="field__label">{t("detail.proxy")}</span>
                <input
                  className="input input--mono"
                  value={draft.proxy ?? ""}
                  onChange={(e) => {
                    const value = e.target.value.trim();
                    setProxyError(null);
                    setDraft((d) => ({
                      ...d,
                      proxy: value || null,

                      session: value ? "isolated" : d.session,
                    }));
                  }}
                  placeholder="socks5://127.0.0.1:9050"
                  spellCheck={false}
                />
                <span className={`field__hint${proxyError ? " is-error" : ""}`}>
                  {proxyError ?? t("detail.proxy.hint")}
                </span>
              </label>

              <div className="field">
                <span className="field__label">{t("detail.auth")}</span>
                <div className="row">
                  <input
                    className="input"
                    value={login.username}
                    onChange={(e) =>
                      setLogin((l) => ({ ...l, username: e.target.value }))
                    }
                    placeholder={t("detail.auth.username")}
                    aria-label={t("detail.auth.username")}
                    autoComplete="off"
                    spellCheck={false}
                  />
                  <input
                    className="input"
                    type="password"
                    value={login.password}
                    onChange={(e) =>
                      setLogin((l) => ({ ...l, password: e.target.value }))
                    }
                    placeholder={t("detail.auth.password")}
                    aria-label={t("detail.auth.password")}
                    autoComplete="off"
                  />
                </div>
                <span className="field__hint">{t("detail.auth.hint")}</span>
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.hotkey")}</div>
                  <div className="pref__hint">
                    {hotkeyBad
                      ? t("detail.hotkey.taken")
                      : t("detail.hotkey.hint")}
                  </div>
                </div>
                <div className="hotkey">
                  <button
                    type="button"
                    className={`hotkey__field${recording ? " is-recording" : ""}`}
                    onClick={() => setRecording(true)}
                    onBlur={() => setRecording(false)}

                    onKeyDown={(e) => {
                      if (!recording) return;
                      e.preventDefault();
                      if (e.key === "Escape") {
                        setRecording(false);
                        return;
                      }
                      const parts: string[] = [];
                      if (e.ctrlKey) parts.push("Ctrl");
                      if (e.altKey) parts.push("Alt");
                      if (e.shiftKey) parts.push("Shift");
                      if (e.metaKey) parts.push("Super");

                      if (["Control", "Alt", "Shift", "Meta"].includes(e.key))
                        return;
                      parts.push(
                        e.key.length === 1 ? e.key.toUpperCase() : e.key,
                      );
                      const accelerator = parts.join("+");

                      void api
                        .validHotkey(accelerator)
                        .then((ok) => {
                          setHotkeyBad(!ok);
                          if (ok) {
                            set("hotkey", accelerator);
                            setRecording(false);
                          }
                        })
                        .catch(() => setHotkeyBad(true));
                    }}
                  >
                    {recording
                      ? t("detail.hotkey.record")
                      : (draft.hotkey ?? "—")}
                  </button>
                  {draft.hotkey && (
                    <button
                      type="button"
                      className="btn btn--tiny btn--quiet"
                      onClick={() => {
                        set("hotkey", null);
                        setHotkeyBad(false);
                      }}
                    >
                      {t("detail.hotkey.clear")}
                    </button>
                  )}
                </div>
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.badge")}</div>
                  <div className="pref__hint">{t("detail.badge.hint")}</div>
                </div>
                <label className="switch">
                  <input
                    type="checkbox"
                    checked={draft.title_badge}
                    onChange={(e) => set("title_badge", e.target.checked)}
                  />
                  <span className="switch__track" />
                </label>
              </div>

              <div className="pref">
                <div className="pref__text">
                  <div className="pref__title">{t("detail.permissions")}</div>
                  <div className="pref__hint">{t("detail.perm.hint")}</div>
                </div>
              </div>

              {(
                [
                  ["notifications", "detail.perm.notifications"],
                  ["microphone", "detail.perm.microphone"],
                  ["camera", "detail.perm.camera"],
                  ["location", "detail.perm.location"],
                ] as [keyof Permissions, "detail.perm.notifications"][]
              ).map(([which, label]) => (
                <div className="pref pref--tight" key={which}>
                  <div className="pref__text">
                    <div className="pref__title">{t(label)}</div>
                  </div>
                  <Select
                    ariaLabel={t(label)}
                    value={draft.permissions[which]}
                    onChange={(v) =>
                      set("permissions", {
                        ...draft.permissions,
                        [which]: v as Rule,
                      })
                    }
                    options={[
                      { value: "allow", label: t("rule.allow") },
                      { value: "ask", label: t("rule.ask") },
                      { value: "deny", label: t("rule.deny") },
                    ]}
                  />
                </div>
              ))}

              <label className="field">
                <span className="field__label">{t("detail.group")}</span>
                <input
                  className="input"
                  value={draft.group ?? ""}
                  onChange={(e) => set("group", e.target.value || null)}
                  placeholder={t("detail.group.placeholder")}
                  list="zww-groups"
                  spellCheck={false}
                />

                <datalist id="zww-groups">
                  {groups.map((g) => (
                    <option key={g} value={g} />
                  ))}
                </datalist>
                <span className="field__hint">{t("detail.group.hint")}</span>
              </label>

              <label className="field">
                <span className="field__label">{t("detail.scope")}</span>
                <input
                  className="input"
                  value={draft.extra_scope.join(", ")}
                  onChange={(e) =>
                    set(
                      "extra_scope",
                      e.target.value
                        .split(",")
                        .map((s) => s.trim())
                        .filter(Boolean),
                    )
                  }
                  placeholder="youtu.be, m.example.com"
                  spellCheck={false}
                />
                <span className="field__hint">{t("detail.scope.hint")}</span>
              </label>
            </section>
          )}

          {pane === "appearance" && (
            <section className="panel">
              <div className="grid2">
                <div className="field">
                  <span className="field__label">{t("detail.zoom")}</span>
                  <Select
                    ariaLabel={t("detail.zoom")}
                    value={String(draft.zoom)}
                    onChange={(v) => set("zoom", Number(v))}
                    options={[0.8, 0.9, 1, 1.1, 1.25, 1.5].map((z) => ({
                      value: String(z),
                      label: `${Math.round(z * 100)}%`,
                    }))}
                  />
                </div>

                <label className="field">
                  <span className="field__label">{t("detail.window")}</span>
                  <input
                    className="input"
                    value={`${Math.round(draft.window.width)} × ${Math.round(draft.window.height)}`}
                    readOnly
                    tabIndex={-1}
                  />
                  <span className="field__hint">{t("detail.window.hint")}</span>
                </label>
              </div>
            </section>
          )}

          {pane === "advanced" && (
            <section className="panel">
              <div className="field">
                <span className="field__label">{t("detail.userAgent")}</span>
                <div className="chips">
                  {presets.map((preset) => {
                    const active = (draft.user_agent ?? "") === preset.value;
                    return (
                      <button
                        key={preset.label}
                        type="button"
                        className={`chip${active ? " is-active" : ""}`}
                        title={preset.hint}
                        onClick={() => set("user_agent", preset.value || null)}
                      >
                        {preset.label}
                      </button>
                    );
                  })}
                </div>
                <input
                  className="input input--mono"
                  value={draft.user_agent ?? ""}
                  onChange={(e) =>
                    set("user_agent", e.target.value.trim() || null)
                  }
                  placeholder={t("detail.userAgent.placeholder")}
                  spellCheck={false}
                />
                <span className="field__hint">
                  {t("detail.userAgent.hint")}
                </span>
              </div>

              <label className="field">
                <span className="field__label">{t("detail.userCss")}</span>
                <textarea
                  className="input input--mono input--code"
                  rows={4}
                  value={draft.user_css ?? ""}
                  onChange={(e) => set("user_css", e.target.value || null)}
                  placeholder=".ad-slot { display: none !important; }"
                  spellCheck={false}
                />
                <span className="field__hint">{t("detail.userCss.hint")}</span>
              </label>

              <label className="field">
                <span className="field__label">{t("detail.userJs")}</span>
                <textarea
                  className="input input--mono input--code"
                  rows={4}
                  value={draft.user_js ?? ""}
                  onChange={(e) => set("user_js", e.target.value || null)}
                  placeholder="document.title = 'Focus';"
                  spellCheck={false}
                />
                <span className="field__hint">{t("detail.userJs.hint")}</span>
              </label>
            </section>
          )}
        </div>
      </div>
    </Modal>
  );
}
