interface Props {
  className?: string;
}

const base = {
  width: 16,
  height: 16,
  viewBox: "0 0 24 24",
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 2.25,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
};

export const IconShield = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M12 3l7 3v5c0 4.5-3 8.3-7 10-4-1.7-7-5.5-7-10V6l7-3z" />
  </svg>
);

export const IconSearch = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <circle cx="11" cy="11" r="7" />
    <path d="M20 20l-3.5-3.5" />
  </svg>
);

export const IconSettings = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.7 1.7 0 00.3 1.9l.1.1a2 2 0 11-2.8 2.8l-.1-.1a1.7 1.7 0 00-1.9-.3 1.7 1.7 0 00-1 1.5v.2a2 2 0 11-4 0v-.1a1.7 1.7 0 00-1.1-1.5 1.7 1.7 0 00-1.9.3l-.1.1a2 2 0 11-2.8-2.8l.1-.1a1.7 1.7 0 00.3-1.9 1.7 1.7 0 00-1.5-1H2a2 2 0 110-4h.1A1.7 1.7 0 003.6 9a1.7 1.7 0 00-.3-1.9l-.1-.1a2 2 0 112.8-2.8l.1.1a1.7 1.7 0 001.9.3H8a1.7 1.7 0 001-1.5V3a2 2 0 114 0v.1a1.7 1.7 0 001 1.5 1.7 1.7 0 001.9-.3l.1-.1a2 2 0 112.8 2.8l-.1.1a1.7 1.7 0 00-.3 1.9V9a1.7 1.7 0 001.5 1h.2a2 2 0 110 4h-.1a1.7 1.7 0 00-1.5 1z" />
  </svg>
);

export const IconPlus = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M12 5v14M5 12h14" />
  </svg>
);

export const IconTrash = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M4 7h16M10 11v6M14 11v6" />
    <path d="M5 7l1 12a2 2 0 002 2h8a2 2 0 002-2l1-12M9 7V4h6v3" />
  </svg>
);

export const IconEdit = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M12 20h9" />
    <path d="M16.5 3.5a2.1 2.1 0 013 3L7 19l-4 1 1-4z" />
  </svg>
);

export const IconExternal = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6" />
    <path d="M15 3h6v6M10 14L21 3" />
  </svg>
);

export const IconClose = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M18 6L6 18M6 6l12 12" />
  </svg>
);

export const IconRefresh = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M21 12a9 9 0 11-3-6.7L21 8" />
    <path d="M21 3v5h-5" />
  </svg>
);

export const IconFolder = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M3 7a2 2 0 012-2h4l2 2h8a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
  </svg>
);

export const IconDownload = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M12 3v12" />
    <path d="M7 11l5 5 5-5" />
    <path d="M4 20h16" />
  </svg>
);

export const IconCheck = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M20 6L9 17l-5-5" />
  </svg>
);

export const IconSliders = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M4 6h10M18 6h2M4 12h4M12 12h8M4 18h10M18 18h2" />
    <circle cx="16" cy="6" r="2" />
    <circle cx="10" cy="12" r="2" />
    <circle cx="16" cy="18" r="2" />
  </svg>
);

export const IconDatabase = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <ellipse cx="12" cy="6" rx="7" ry="3" />
    <path d="M5 6v6c0 1.7 3.1 3 7 3s7-1.3 7-3V6" />
    <path d="M5 12v6c0 1.7 3.1 3 7 3s7-1.3 7-3v-6" />
  </svg>
);

export const IconInfo = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <circle cx="12" cy="12" r="9" />
    <path d="M12 11v5" />
    <path d="M12 7.6v.6" />
  </svg>
);

export const IconPower = ({ className }: Props) => (
  <svg {...base} className={className} aria-hidden="true">
    <path d="M12 4v8" />
    <path d="M7.5 6.5a7 7 0 109 0" />
  </svg>
);
