import { useEffect, useLayoutEffect, useRef, useState } from "react";

export interface MenuItem {
  label: string;
  onSelect: () => void;

  danger?: boolean;

  separated?: boolean;
}

interface Props {
  x: number;
  y: number;
  items: MenuItem[];
  onClose: () => void;
}

export function ContextMenu({ x, y, items, onClose }: Props) {
  const menu = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState({ x, y });

  useLayoutEffect(() => {
    const el = menu.current;
    if (!el) return;
    const { width, height } = el.getBoundingClientRect();
    setPos({
      x: Math.min(x, window.innerWidth - width - 8),
      y: Math.min(y, window.innerHeight - height - 8),
    });
  }, [x, y]);

  useEffect(() => {
    const onPointerDown = (e: MouseEvent) => {
      if (menu.current?.contains(e.target as Node)) return;
      onClose();
    };
    const dismiss = () => onClose();
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };

    window.addEventListener("mousedown", onPointerDown, true);
    window.addEventListener("resize", dismiss);
    window.addEventListener("blur", dismiss);
    document.addEventListener("keydown", onKey);
    return () => {
      window.removeEventListener("mousedown", onPointerDown, true);
      window.removeEventListener("resize", dismiss);
      window.removeEventListener("blur", dismiss);
      document.removeEventListener("keydown", onKey);
    };
  }, [onClose]);

  return (
    <div
      className="ctxmenu"
      ref={menu}
      style={{ left: pos.x, top: pos.y }}
      role="menu"
      onMouseDown={(e) => e.stopPropagation()}
    >
      {items.map((item, i) => (
        <button
          key={i}
          role="menuitem"
          className={`ctxmenu__item${item.danger ? " is-danger" : ""}${
            item.separated ? " is-separated" : ""
          }`}

          onMouseUp={() => {
            onClose();
            item.onSelect();
          }}
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}
