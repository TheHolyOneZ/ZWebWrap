import { useEffect, useState } from "react";
import * as api from "../api";
import { errorMessage, hostOf } from "../api";
import type { Bookmark } from "../types";
import { IconCheck } from "./Icons";
import { Modal } from "./Modal";
import { useT } from "../i18n/useI18n";

interface Props {
  onDone: (created: number) => void;
  onError: (message: string) => void;
  onClose: () => void;
}

function BookmarkIcon({ host, title }: { host: string; title: string }) {
  const [failed, setFailed] = useState(false);

  if (failed || !host) {
    return (
      <span className="pick__icon pick__icon--blank" aria-hidden="true">
        {title.slice(0, 1).toUpperCase()}
      </span>
    );
  }

  return (
    <img
      className="pick__icon"
      src={`https://${host}/favicon.ico`}
      alt=""
      width={16}
      height={16}
      loading="lazy"
      onError={() => setFailed(true)}
    />
  );
}

export function ImportBookmarks({ onDone, onError, onClose }: Props) {
  const t = useT();
  const [found, setFound] = useState<Bookmark[] | null>(null);
  const [chosen, setChosen] = useState<Set<string>>(new Set());
  const [busy, setBusy] = useState(false);
  const [query, setQuery] = useState("");

  useEffect(() => {
    api
      .discoverBookmarks()
      .then(setFound)
      .catch((e) => {
        setFound([]);
        onError(errorMessage(e));
      });
  }, [onError]);

  const visible = (found ?? []).filter((b) => {
    const needle = query.trim().toLowerCase();
    if (!needle) return true;
    return (
      b.title.toLowerCase().includes(needle) ||
      b.url.toLowerCase().includes(needle)
    );
  });

  const toggle = (url: string) =>
    setChosen((prev) => {
      const next = new Set(prev);
      if (next.has(url)) next.delete(url);
      else next.add(url);
      return next;
    });

  const importChosen = async () => {
    if (!found || chosen.size === 0) return;
    setBusy(true);
    try {
      const picked = found.filter((b) => chosen.has(b.url));
      const created = await api.importBookmarks(
        picked.map((b) => b.url),
        picked.map((b) => b.title),
      );
      onDone(created);
    } catch (e) {
      onError(errorMessage(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Modal
      title={t("import.title")}
      subtitle={t("import.subtitle")}
      wide
      onClose={onClose}
      footer={
        <>
          <span className="modal__note">
            {chosen.size > 0
              ? t("import.chosen", { n: chosen.size })
              : t("import.pick")}
          </span>
          <div className="spacer" />
          <button className="btn" onClick={onClose} disabled={busy}>
            {t("detail.cancel")}
          </button>
          <button
            className="btn btn--primary"
            onClick={importChosen}
            disabled={busy || chosen.size === 0}
          >
            {busy
              ? t("import.working")
              : t("import.submit", { n: chosen.size })}
          </button>
        </>
      }
    >
      <div className="importer">
        {found === null ? (
          <div className="importer__state">{t("import.reading")}</div>
        ) : found.length === 0 ? (
          <div className="importer__state">{t("import.none")}</div>
        ) : (
          <>
            <input
              className="input"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder={t("import.search")}
              aria-label={t("import.search")}
            />

            <ul className="picks">
              {visible.map((b) => (
                <li key={b.url}>
                  <label
                    className={`pick${chosen.has(b.url) ? " is-picked" : ""}`}
                  >
                    <input
                      className="check__input"
                      type="checkbox"
                      checked={chosen.has(b.url)}
                      onChange={() => toggle(b.url)}
                    />
                    <span className="check" aria-hidden="true">
                      <IconCheck />
                    </span>
                    <BookmarkIcon host={hostOf(b.url)} title={b.title} />
                    <span className="pick__text">
                      <span className="pick__title">{b.title}</span>
                      <span className="pick__host">{hostOf(b.url)}</span>
                    </span>
                    <span className="pick__source">{b.source}</span>
                  </label>
                </li>
              ))}
            </ul>
          </>
        )}
      </div>
    </Modal>
  );
}
