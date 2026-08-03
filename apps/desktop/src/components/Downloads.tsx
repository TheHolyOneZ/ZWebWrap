import * as api from "../api";
import { errorMessage, formatBytes } from "../api";
import type { Download } from "../types";
import { useT } from "../i18n/useI18n";
import { IconExternal, IconFolder } from "./Icons";

interface Props {
  downloads: Download[];
  onChanged: (next: Download[]) => void;
  onError: (message: string) => void;
  onClose: () => void;
}

export function Downloads({ downloads, onChanged, onError, onClose }: Props) {
  const t = useT();
  const anyFinished = downloads.some((d) => d.state !== "active");

  const act = async (run: Promise<unknown>) => {
    try {
      await run;
    } catch (e) {
      onError(errorMessage(e));
    }
  };

  return (
    <>
      <div className="popover__scrim" onClick={onClose} />

      <div
        className="popover downloads"
        role="dialog"
        aria-label={t("dl.title")}
      >
        <div className="popover__head">
          <span className="popover__title">{t("dl.title")}</span>
          {anyFinished && (
            <button
              className="btn btn--tiny btn--quiet"
              onClick={() =>
                void api
                  .clearDownloads()
                  .then(onChanged)
                  .catch((e) => onError(errorMessage(e)))
              }
            >
              {t("dl.clear")}
            </button>
          )}
        </div>

        {downloads.length === 0 ? (
          <p className="popover__empty">{t("dl.empty")}</p>
        ) : (
          <ul className="dl__list">
            {downloads.map((d) => (
              <li key={d.id} className={`dl__row dl__row--${d.state}`}>
                <span className="dl__text">
                  <span className="dl__name" title={d.path}>
                    {d.file_name}
                  </span>
                  <span className="dl__meta">
                    {d.state === "active"
                      ? t("dl.running", { size: formatBytes(d.bytes) })
                      : d.state === "failed"
                        ? t("dl.failed")
                        : formatBytes(d.bytes)}
                  </span>
                </span>

                {d.state === "done" && (
                  <span className="dl__actions">
                    <button
                      className="iconbtn"
                      title={t("dl.open")}
                      aria-label={t("dl.open")}
                      onClick={() => void act(api.openDownload(d.id))}
                    >
                      <IconExternal />
                    </button>
                    <button
                      className="iconbtn"
                      title={t("dl.reveal")}
                      aria-label={t("dl.reveal")}
                      onClick={() => void act(api.revealDownload(d.id))}
                    >
                      <IconFolder />
                    </button>
                  </span>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>
    </>
  );
}
