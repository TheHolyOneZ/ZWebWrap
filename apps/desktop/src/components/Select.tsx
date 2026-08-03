import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

export interface Option<T extends string> {
  value: T;
  label: string;
  hint?: string;
}

interface Props<T extends string> {
  value: T;
  options: Option<T>[];
  onChange: (value: T) => void;
  ariaLabel?: string;
}

export function Select<T extends string>({
  value,
  options,
  onChange,
  ariaLabel,
}: Props<T>) {
  const [open, setOpen] = useState(false);
  const [active, setActive] = useState(0);
  const root = useRef<HTMLDivElement>(null);
  const list = useRef<HTMLDivElement>(null);

  const [box, setBox] = useState({ left: 0, top: 0, width: 0 });

  const selected = options.find((o) => o.value === value) ?? options[0];

  useLayoutEffect(() => {
    if (!open || !root.current) return;
    const rect = root.current.getBoundingClientRect();
    const needed = Math.min(options.length * 38 + 10, 260);

    const flip = window.innerHeight - rect.bottom < needed && rect.top > needed;
    setBox({
      left: rect.left,
      top: flip ? rect.top - needed - 5 : rect.bottom + 5,
      width: rect.width,
    });
    setActive(
      Math.max(
        0,
        options.findIndex((o) => o.value === value),
      ),
    );
  }, [open, options, value]);

  useEffect(() => {
    if (!open) return;
    const onPointerDown = (e: MouseEvent) => {
      const target = e.target as Node;
      if (root.current?.contains(target) || list.current?.contains(target))
        return;
      setOpen(false);
    };

    const close = () => setOpen(false);
    window.addEventListener("mousedown", onPointerDown, true);
    window.addEventListener("resize", close);
    return () => {
      window.removeEventListener("mousedown", onPointerDown, true);
      window.removeEventListener("resize", close);
    };
  }, [open]);

  const choose = (option: Option<T>) => {
    onChange(option.value);
    setOpen(false);
    root.current?.querySelector<HTMLButtonElement>(".select__trigger")?.focus();
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (!open) {
      if (e.key === "Enter" || e.key === " " || e.key === "ArrowDown") {
        e.preventDefault();
        setOpen(true);
      }
      return;
    }
    if (e.key === "Escape") {
      e.preventDefault();
      setOpen(false);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      setActive((i) => Math.min(i + 1, options.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setActive((i) => Math.max(i - 1, 0));
    } else if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      choose(options[active]);
    }
  };

  return (
    <div className="select" ref={root} onKeyDown={onKeyDown}>
      <button
        type="button"
        className={`select__trigger${open ? " is-open" : ""}`}
        onClick={() => setOpen((v) => !v)}
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-label={ariaLabel}
      >
        <span className="select__value">{selected?.label ?? ""}</span>
        <svg className="select__caret" viewBox="0 0 24 24" aria-hidden="true">
          <path
            d="M6 9l6 6 6-6"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      </button>

      {open &&
        createPortal(
          <div
            className="select__list"
            role="listbox"
            ref={list}
            style={{ left: box.left, top: box.top, minWidth: box.width }}
            onMouseDown={(e) => e.stopPropagation()}
          >
            {options.map((option, i) => (
              <button
                key={option.value}
                type="button"
                role="option"
                aria-selected={option.value === value}
                className={`select__option${i === active ? " is-active" : ""}${
                  option.value === value ? " is-selected" : ""
                }`}
                onMouseEnter={() => setActive(i)}
                onMouseUp={() => choose(option)}
              >
                <span className="select__option-label">{option.label}</span>
                {option.hint && (
                  <span className="select__option-hint">{option.hint}</span>
                )}
              </button>
            ))}
          </div>,
          document.body,
        )}
    </div>
  );
}
