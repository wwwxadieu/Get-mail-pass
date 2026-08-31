import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";

const CLEAR_AFTER_MS = 30_000;
let clearTimer: number | undefined;

/**
 * Chép vào clipboard rồi tự xoá sau 30 giây — nhưng chỉ xoá nếu nội dung
 * clipboard vẫn đúng là thứ ta đã chép (tránh xoá mất thứ người dùng copy sau đó).
 */
export async function copyWithAutoClear(value: string): Promise<void> {
  await writeText(value);
  if (clearTimer !== undefined) window.clearTimeout(clearTimer);
  clearTimer = window.setTimeout(async () => {
    try {
      const current = await readText();
      if (current === value) await writeText("");
    } catch {
      /* clipboard bị ứng dụng khác chiếm — bỏ qua */
    }
  }, CLEAR_AFTER_MS);
}

export const CLEAR_SECONDS = CLEAR_AFTER_MS / 1000;
