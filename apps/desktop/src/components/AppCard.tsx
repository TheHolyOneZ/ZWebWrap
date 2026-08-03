import { hostOf } from "../api";
import type { WrappedApp } from "../types";
import type { IconFit } from "../iconFit";
import { IconEdit, IconShield } from "./Icons";
import { useT } from "../i18n/useI18n";

interface Props {
  app: WrappedApp;
  icon: string | null;

  fit: IconFit;
  onOpen: () => void;
  onEdit: () => void;
  onContextMenu: (e: React.MouseEvent) => void;
}

export function AppCard({
  app,
  icon,
  fit,
  onOpen,
  onEdit,
  onContextMenu,
}: Props) {
  const t = useT();
  const blocking = app.adblock !== "off";

  return (
    <article className="app" onContextMenu={onContextMenu}>
      <button
        className="app__open"
        onClick={onOpen}
        title={t("card.open", { name: app.title })}
      >
        <span className="app__icon">
          {icon ? (
            <span className={`app__art app__art--${fit}`}>
              <img src={icon} alt="" draggable={false} />
            </span>
          ) : (
            <span className="app__art app__art--contain app__initial">
              {app.title.slice(0, 1).toUpperCase()}
            </span>
          )}

          <span
            className={`app__shield ${blocking ? "is-on" : ""}`}
            title={t(
              blocking ? "card.blockingOn.title" : "card.blockingOff.title",
            )}
          >
            <IconShield />
            <span className="sr-only">
              {t(blocking ? "card.blocking" : "card.blockingOff")}
            </span>
          </span>
        </span>

        <span className="app__text">
          <span className="app__name">{app.title}</span>
          <span className="app__host">{hostOf(app.url)}</span>
        </span>
      </button>

      <button
        className="iconbtn app__edit"
        onClick={onEdit}
        title={t("card.settings", { name: app.title })}
        aria-label={t("card.settings", { name: app.title })}
      >
        <IconEdit />
      </button>
    </article>
  );
}
