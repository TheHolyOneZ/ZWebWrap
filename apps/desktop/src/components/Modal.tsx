import { useEffect, useRef, type ReactNode } from "react";
import { IconClose } from "./Icons";
import { useT } from "../i18n/useI18n";

interface Props {
  title: string;
  subtitle?: string;

  wide?: boolean;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
}

export function Modal({
  title,
  subtitle,
  wide,
  onClose,
  children,
  footer,
}: Props) {
  const t = useT();
  const panel = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    document.addEventListener("keydown", onKey);

    panel.current?.focus();
    return () => document.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div className="overlay" onMouseDown={onClose}>
      <div
        className={`modal ${wide ? "modal--wide" : ""}`}
        role="dialog"
        aria-modal="true"
        aria-label={title}
        tabIndex={-1}
        ref={panel}

        onMouseDown={(e) => e.stopPropagation()}
      >
        <header className="modal__head">
          <div>
            <h2 className="modal__title">{title}</h2>
            {subtitle && <p className="modal__subtitle">{subtitle}</p>}
          </div>
          <button
            className="iconbtn"
            onClick={onClose}
            aria-label={t("ui.close")}
          >
            <IconClose />
          </button>
        </header>

        <div className="modal__body">{children}</div>

        {footer && <footer className="modal__foot">{footer}</footer>}
      </div>
    </div>
  );
}
