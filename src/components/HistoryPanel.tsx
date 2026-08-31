import type { HistoryItem } from "../lib/types";

type Props = {
  items: HistoryItem[];
  onCopy: (value: string) => void;
  onClear: () => void;
};

const KIND_LABEL: Record<HistoryItem["kind"], string> = {
  password: "mật khẩu",
  passphrase: "cụm từ",
  email: "email",
};

function clock(ts: number) {
  return new Date(ts).toLocaleTimeString("vi-VN", { hour: "2-digit", minute: "2-digit" });
}

export default function HistoryPanel({ items, onCopy, onClear }: Props) {
  return (
    <div className="panel">
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2>Lịch sử</h2>
        {items.length > 0 && (
          <button className="btn ghost danger" onClick={onClear}>
            Xoá hết
          </button>
        )}
      </div>
      <p className="sub">Bấm vào một mục để chép lại. Chỉ lưu trên máy này.</p>

      {items.length === 0 ? (
        <div className="empty">
          Chưa có gì.
          <br />
          Mỗi lần bạn chép một mật khẩu hoặc tạo địa chỉ email, nó sẽ xuất hiện ở đây.
        </div>
      ) : (
        items.map((it) => (
          <div key={it.id} className="hist-item" onClick={() => onCopy(it.value)}>
            <div className="txt">
              <div className="val">{it.value}</div>
              <div className="meta">
                {clock(it.at)} · {it.note}
              </div>
            </div>
            <span className="pill">{KIND_LABEL[it.kind]}</span>
          </div>
        ))
      )}
    </div>
  );
}
