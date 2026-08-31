import { getCurrentWindow } from "@tauri-apps/api/window";

type Props = {
  theme: "light" | "dark";
  onToggleTheme: () => void;
};

export default function TitleBar({ theme, onToggleTheme }: Props) {
  const win = getCurrentWindow();
  return (
    <div className="titlebar" data-tauri-drag-region>
      <div className="brand" data-tauri-drag-region>
        <span className="dot" />
        PassMail
      </div>
      <div className="spacer" data-tauri-drag-region />
      <button
        className="icon-btn"
        onClick={onToggleTheme}
        title={theme === "dark" ? "Chuyển sang nền sáng" : "Chuyển sang nền tối"}
      >
        {theme === "dark" ? "☀" : "☾"}
      </button>
      <div className="win-btns">
        <button className="win-btn" onClick={() => win.minimize()} title="Thu nhỏ">
          ―
        </button>
        <button className="win-btn" onClick={() => win.toggleMaximize()} title="Phóng to">
          ▢
        </button>
        <button className="win-btn close" onClick={() => win.close()} title="Đóng">
          ✕
        </button>
      </div>
    </div>
  );
}
