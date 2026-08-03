import { useCallback, useEffect, useLayoutEffect, useState } from "react";
import { useT } from "../i18n/useI18n";
import type { Key } from "../i18n";

interface Step {
  anchor: string | null;
  title: Key;
  body: Key;
}

const STEPS: Step[] = [
  { anchor: null, title: "tour.welcome.title", body: "tour.welcome.body" },
  { anchor: "composer", title: "tour.add.title", body: "tour.add.body" },
  { anchor: "import", title: "tour.import.title", body: "tour.import.body" },
  { anchor: "library", title: "tour.library.title", body: "tour.library.body" },
  { anchor: null, title: "tour.toolbar.title", body: "tour.toolbar.body" },
  {
    anchor: "blocking",
    title: "tour.blocking.title",
    body: "tour.blocking.body",
  },
  {
    anchor: "settings",
    title: "tour.settings.title",
    body: "tour.settings.body",
  },
];

interface Box {
  top: number;
  left: number;
  width: number;
  height: number;
}

function place(box: Box | null): { style: React.CSSProperties; arrow: string } {
  const CARD = 340;
  const GAP = 14;

  if (!box) {
    return {
      style: { top: "50%", left: "50%", transform: "translate(-50%, -50%)" },
      arrow: "none",
    };
  }

  const below = box.top + box.height + GAP;
  const roomBelow = window.innerHeight - below;

  const wanted = box.left + box.width / 2 - CARD / 2;
  const left = Math.max(16, Math.min(wanted, window.innerWidth - CARD - 16));

  if (roomBelow > 220) return { style: { top: below, left }, arrow: "up" };
  return {
    style: {
      top: Math.max(16, box.top - GAP),
      left,
      transform: "translateY(-100%)",
    },
    arrow: "down",
  };
}

interface Props {
  onDone: () => void;
}

export function Tour({ onDone }: Props) {
  const t = useT();
  const [index, setIndex] = useState(0);
  const [box, setBox] = useState<Box | null>(null);

  const step = STEPS[index];
  const last = index === STEPS.length - 1;

  useLayoutEffect(() => {
    const measure = () => {
      if (!step.anchor) {
        setBox(null);
        return;
      }
      const el = document.querySelector(`[data-tour="${step.anchor}"]`);
      if (!el) {
        setBox(null);
        return;
      }
      const rect = el.getBoundingClientRect();
      if (rect.width === 0 || rect.height === 0) {
        setBox(null);
        return;
      }
      setBox({
        top: rect.top - 6,
        left: rect.left - 6,
        width: rect.width + 12,
        height: rect.height + 12,
      });
    };

    measure();
    window.addEventListener("resize", measure);
    window.addEventListener("scroll", measure, true);
    return () => {
      window.removeEventListener("resize", measure);
      window.removeEventListener("scroll", measure, true);
    };
  }, [step]);

  const next = useCallback(() => {
    setIndex((i) => {
      if (i < STEPS.length - 1) return i + 1;
      onDone();
      return i;
    });
  }, [onDone]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        onDone();
      } else if (e.key === "ArrowRight" || e.key === "Enter") {
        e.preventDefault();
        next();
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        setIndex((i) => Math.max(0, i - 1));
      }
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [next, onDone]);

  const { style, arrow } = place(box);

  return (
    <div
      className="tour"
      role="dialog"
      aria-modal="true"
      aria-label={t("tour.title")}
    >
      {box ? (
        <div
          className="tour__hole"
          style={{
            top: box.top,
            left: box.left,
            width: box.width,
            height: box.height,
          }}
          onClick={onDone}
        />
      ) : (
        <div className="tour__dim" onClick={onDone} />
      )}

      <div className={`tour__card tour__card--${arrow}`} style={style}>
        <p className="tour__count">
          {t("tour.step", { n: index + 1, total: STEPS.length })}
        </p>
        <h2 className="tour__title">{t(step.title)}</h2>
        <p className="tour__body">{t(step.body)}</p>

        <div className="tour__foot">
          <button className="tour__skip" onClick={onDone}>
            {t("tour.skip")}
          </button>
          <div className="tour__nav">
            {index > 0 && (
              <button
                className="btn btn--small"
                onClick={() => setIndex((i) => Math.max(0, i - 1))}
              >
                {t("tour.back")}
              </button>
            )}
            <button className="btn btn--primary btn--small" onClick={next}>
              {last ? t("tour.done") : t("tour.next")}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
