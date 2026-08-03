import { useCallback, useEffect, useRef, useState } from "react";
import { useT } from "../i18n/useI18n";

export interface Toast {
  id: number;
  kind: "ok" | "error";
  message: string;
}

export function useToasts() {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const nextId = useRef(1);
  const timers = useRef<number[]>([]);

  const dismiss = useCallback((id: number) => {
    setToasts((current) => current.filter((t) => t.id !== id));
  }, []);

  const push = useCallback(
    (kind: Toast["kind"], message: string) => {
      const id = nextId.current++;
      setToasts((current) => [...current, { id, kind, message }]);

      const timer = window.setTimeout(
        () => dismiss(id),
        kind === "error" ? 7000 : 3000,
      );
      timers.current.push(timer);
    },
    [dismiss],
  );

  useEffect(() => () => timers.current.forEach(clearTimeout), []);

  return {
    toasts,
    dismiss,
    ok: useCallback((m: string) => push("ok", m), [push]),
    error: useCallback((m: string) => push("error", m), [push]),
  };
}

export function ToastStack({
  toasts,
  onDismiss,
}: {
  toasts: Toast[];
  onDismiss: (id: number) => void;
}) {
  const translate = useT();
  return (
    <div className="toasts" role="status" aria-live="polite">
      {toasts.map((t) => (
        <button
          key={t.id}
          className={`toast toast--${t.kind}`}
          onClick={() => onDismiss(t.id)}
          title={translate("ui.dismiss")}
        >
          {t.message}
        </button>
      ))}
    </div>
  );
}
