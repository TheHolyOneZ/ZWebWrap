export type IconFit = "cover" | "contain";

const SIZE = 64;

const CORNER = Math.round(SIZE * 0.16);

const MIN_ALPHA = 230;

const MAX_SPREAD = 14;

export function classifyIcon(dataUrl: string): Promise<IconFit> {
  return new Promise((resolve) => {
    const img = new Image();
    img.onload = () => {
      try {
        resolve(measure(img));
      } catch {
        resolve("contain");
      }
    };
    img.onerror = () => resolve("contain");
    img.src = dataUrl;
  });
}

function measure(img: HTMLImageElement): IconFit {
  const canvas = document.createElement("canvas");
  canvas.width = SIZE;
  canvas.height = SIZE;

  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) return "contain";

  ctx.drawImage(img, 0, 0, SIZE, SIZE);

  const corners: [number, number][] = [
    [0, 0],
    [SIZE - CORNER, 0],
    [0, SIZE - CORNER],
    [SIZE - CORNER, SIZE - CORNER],
  ];

  for (const [x, y] of corners) {
    if (!cornerIsFlatAndOpaque(ctx, x, y)) return "contain";
  }

  return "cover";
}

function cornerIsFlatAndOpaque(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
): boolean {
  const { data } = ctx.getImageData(x, y, CORNER, CORNER);
  const luma: number[] = [];

  for (let i = 0; i < data.length; i += 4) {
    if (data[i + 3] < MIN_ALPHA) return false;

    luma.push(0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2]);
  }

  if (luma.length === 0) return false;

  const mean = luma.reduce((a, b) => a + b, 0) / luma.length;
  const spread = luma.reduce((a, b) => a + Math.abs(b - mean), 0) / luma.length;
  return spread <= MAX_SPREAD;
}
