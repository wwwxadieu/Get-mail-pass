import { useCallback, useEffect, useRef, useState } from "react";
import {
  isPermissionGranted,
  requestPermission,
} from "@tauri-apps/plugin-notification";
import TitleBar from "./components/TitleBar";
import PasswordTab from "./components/PasswordTab";
import MailTab from "./components/MailTab";
import HistoryPanel from "./components/HistoryPanel";
import Toasts, { type Toast } from "./components/Toasts";
import { copyWithAutoClear, CLEAR_SECONDS } from "./lib/clipboard";
import { errText } from "./lib/api";
import type { HistoryItem } from "./lib/types";

const HIST_KEY = "passmail.history";
const THEME_KEY = "passmail.theme";
const MAX_HISTORY = 60;

function loadHistory(): HistoryItem[] {
  try {
    const raw = localStorage.getItem(HIST_KEY);
    return raw ? (JSON.parse(raw) as HistoryItem[]) : [];
  } catch {
    return [];
  }
}

export default function App() {
  const [tab, setTab] = useState<"password" | "mail">("password");
  const [theme, setTheme] = useState<"light" | "dark">(
    () => (localStorage.getItem(THEME_KEY) as "light" | "dark") || "light",
  );
  const [history, setHistory] = useState<HistoryItem[]>(loadHistory);
  const [toasts, setToasts] = useState<Toast[]>([]);
  const toastId = useRef(0);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    localStorage.setItem(THEME_KEY, theme);
  }, [theme]);

  useEffect(() => {
    localStorage.setItem(HIST_KEY, JSON.stringify(history));
  }, [history]);

  // Xin quyền thông báo một lần khi mở app
  useEffect(() => {
    void (async () => {
      try {
        if (!(await isPermissionGranted())) await requestPermission();
      } catch {
        /* hệ thống không cho — bỏ qua, app vẫn chạy bình thường */
      }
    })();
  }, []);

  const notify = useCallback((text: string, kind: "ok" | "err" = "ok") => {
    const id = ++toastId.current;
    setToasts((t) => [...t, { id, text, kind }]);
    window.setTimeout(() => setToasts((t) => t.filter((x) => x.id !== id)), 2800);
  }, []);

  const addHistory = useCallback(
    (kind: HistoryItem["kind"], value: string, note: string) => {
      setHistory((prev) => {
        if (prev[0]?.value === value) return prev;
        const item: HistoryItem = {
          id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
          kind,
          value,
          note,
          at: Date.now(),
        };
        return [item, ...prev].slice(0, MAX_HISTORY);
      });
    },
    [],
  );

  const copy = useCallback(
    (value: string) => {
      void copyWithAutoClear(value)
        .then(() => notify(`Đã chép · clipboard tự xoá sau ${CLEAR_SECONDS}s`))
        .catch((e) => notify(errText(e), "err"));
    },
    [notify],
  );

  return (
    <>
      <div className="backdrop">
        <div className="blob b1" />
        <div className="blob b2" />
        <div className="blob b3" />
      </div>

      <div className="shell">
        <TitleBar
          theme={theme}
          onToggleTheme={() => setTheme((t) => (t === "dark" ? "light" : "dark"))}
        />

        <div className="tabs">
          <button className={tab === "password" ? "tab active" : "tab"} onClick={() => setTab("password")}>
            Mật khẩu
          </button>
          <button className={tab === "mail" ? "tab active" : "tab"} onClick={() => setTab("mail")}>
            Email tạm
          </button>
        </div>

        <div className="body">
          {tab === "password" ? (
            <PasswordTab onGenerated={addHistory} onCopy={copy} notify={notify} />
          ) : (
            <MailTab onGenerated={addHistory} onCopy={copy} notify={notify} />
          )}
          <HistoryPanel items={history} onCopy={copy} onClear={() => setHistory([])} />
        </div>
      </div>

      <Toasts items={toasts} />
    </>
  );
}
