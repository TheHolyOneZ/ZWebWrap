import { useEffect, useRef, useState } from "react";
import { useT } from "../i18n/useI18n";
import * as api from "../api";
import { errorMessage, hostOf } from "../api";
import type { Preview } from "../types";
import { IconPlus } from "./Icons";

interface Props {
  onCreated: (name: string) => void | Promise<void>;
  onError: (message: string) => void;
}

export function AddAppForm({ onCreated, onError }: Props) {
  const t = useT();
  const [input, setInput] = useState("");
  const [preview, setPreview] = useState<Preview | null>(null);
  const [busy, setBusy] = useState(false);

  const requestId = useRef(0);

  useEffect(() => {
    const value = input.trim();
    if (value.length < 3 || busy) {
      setPreview(null);
      return;
    }
    const id = ++requestId.current;
    const timer = setTimeout(() => {
      api
        .previewUrl(value)
        .then((result) => {
          if (id === requestId.current) setPreview(result);
        })
        .catch(() => {
          if (id === requestId.current) setPreview(null);
        });
    }, 450);
    return () => clearTimeout(timer);
  }, [input, busy]);

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    const value = input.trim();
    if (busy || !value) return;

    setBusy(true);

    requestId.current++;
    try {
      const adblock = await api
        .getPreferences()
        .then((p) => p.adblock_by_default)
        .catch(() => true);
      const created = await api.createApp(value, preview?.title, adblock);
      setInput("");
      setPreview(null);
      await onCreated(created.title);
    } catch (err) {
      onError(errorMessage(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <form className="composer" data-tour="composer" onSubmit={submit}>
      <div className="composer__main">
        <span className="composer__avatar">
          {preview?.icon_url ? (
            <img src={preview.icon_url} alt="" width={22} height={22} />
          ) : (
            <IconPlus />
          )}
        </span>

        <input
          className="composer__input"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder={t("add.placeholder")}
          aria-label={t("add.placeholder")}
          spellCheck={false}
          autoFocus
        />

        <button
          className="btn btn--primary"
          type="submit"
          disabled={busy || !input.trim()}
        >
          {busy ? t("add.working") : t("add.submit")}
        </button>
      </div>

      <div className="composer__preview" aria-live="polite">
        {busy ? (
          <span className="muted">{t("add.busy")}</span>
        ) : preview ? (
          <span className="muted">
            <strong className="preview__name">{preview.title}</strong>
            <span className="preview__sep">·</span>
            {hostOf(preview.url)}
            <span className="preview__sep">·</span>
            {t("add.preview.suffix")}
          </span>
        ) : (
          <span className="muted">{t("add.hint")}</span>
        )}
      </div>
    </form>
  );
}
