import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { api, errText } from "../lib/api";
import type { InboxEvent, MailAccount, MailDetail, MailSummary } from "../lib/types";

const STORE_KEY = "passmail.account";

type Props = {
  onCopy: (value: string) => void;
  onGenerated: (kind: "email", value: string, note: string) => void;
  notify: (text: string, kind?: "ok" | "err") => void;
};

function relTime(iso: string): string {
  const t = new Date(iso).getTime();
  if (Number.isNaN(t)) return "";
  const diff = Math.max(0, Date.now() - t);
  const m = Math.floor(diff / 60000);
  if (m < 1) return "vừa xong";
  if (m < 60) return `${m} phút trước`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h} giờ trước`;
  return new Date(t).toLocaleDateString("vi-VN");
}

export default function MailTab({ onCopy, onGenerated, notify }: Props) {
  const [account, setAccount] = useState<MailAccount | null>(null);
  const [domains, setDomains] = useState<string[]>([]);
  const [domain, setDomain] = useState("");
  const [localPart, setLocalPart] = useState("");
  const [messages, setMessages] = useState<MailSummary[]>([]);
  const [open, setOpen] = useState<MailDetail | null>(null);
  const [busy, setBusy] = useState(false);
  const [loadingInbox, setLoadingInbox] = useState(false);
  const [domainsError, setDomainsError] = useState<string | null>(null);
  const [loadingDomains, setLoadingDomains] = useState(true);
  const restored = useRef(false);

  // Khôi phục địa chỉ của phiên trước
  useEffect(() => {
    if (restored.current) return;
    restored.current = true;
    const raw = localStorage.getItem(STORE_KEY);
    if (!raw) return;
    try {
      const saved = JSON.parse(raw) as MailAccount;
      void (async () => {
        try {
          const acc = await api.mailRestore(saved.id, saved.address, saved.password);
          setAccount(acc);
        } catch {
          localStorage.removeItem(STORE_KEY);
        }
      })();
    } catch {
      localStorage.removeItem(STORE_KEY);
    }
  }, []);

  // Nuot loi o day tung khien danh sach ten mien rong ma khong ai biet tai sao:
  // giao dien cu bao "Dang tai danh sach..." mai mai. Gio hien han loi va cho thu lai.
  const loadDomains = useCallback(async () => {
    setLoadingDomains(true);
    try {
      const d = await api.mailDomains();
      setDomains(d);
      setDomain((cur) => cur || d[0] || "");
      setDomainsError(null);
    } catch (e) {
      setDomains([]);
      setDomainsError(errText(e));
    } finally {
      setLoadingDomains(false);
    }
  }, []);

  useEffect(() => {
    void loadDomains();
  }, [loadDomains]);

  // Nhận hộp thư mới từ vòng lặp nền trong Rust
  useEffect(() => {
    const un = listen<InboxEvent>("inbox-updated", (e) => {
      setMessages(e.payload.messages);
      if (e.payload.newCount > 0) {
        notify(`Có ${e.payload.newCount} thư mới`);
      }
    });
    return () => {
      void un.then((f) => f());
    };
  }, [notify]);

  const refresh = useCallback(async () => {
    if (!account) return;
    setLoadingInbox(true);
    try {
      setMessages(await api.mailInbox());
    } catch (e) {
      notify(errText(e), "err");
    } finally {
      setLoadingInbox(false);
    }
  }, [account, notify]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const create = async () => {
    setBusy(true);
    try {
      const acc = await api.mailCreate(localPart.trim() || undefined, domain || undefined);
      setAccount(acc);
      setMessages([]);
      setOpen(null);
      setLocalPart("");
      localStorage.setItem(STORE_KEY, JSON.stringify(acc));
      onGenerated("email", acc.address, "mail.tm");
      onCopy(acc.address);
      notify("Đã tạo địa chỉ và chép vào clipboard");
    } catch (e) {
      notify(errText(e), "err");
    } finally {
      setBusy(false);
    }
  };

  const destroy = async () => {
    setBusy(true);
    try {
      await api.mailDestroy();
    } catch (e) {
      notify(errText(e), "err");
    } finally {
      localStorage.removeItem(STORE_KEY);
      setAccount(null);
      setMessages([]);
      setOpen(null);
      setBusy(false);
    }
  };

  const openMail = async (id: string) => {
    try {
      const d = await api.mailRead(id);
      setOpen(d);
      setMessages((prev) => prev.map((m) => (m.id === id ? { ...m, seen: true } : m)));
    } catch (e) {
      notify(errText(e), "err");
    }
  };

  const removeMail = async (id: string) => {
    try {
      await api.mailDelete(id);
      setMessages((prev) => prev.filter((m) => m.id !== id));
      if (open?.id === id) setOpen(null);
    } catch (e) {
      notify(errText(e), "err");
    }
  };

  const checkConn = async () => {
    try {
      notify(await api.mailCheckConnection());
    } catch (e) {
      notify(errText(e), "err");
    }
  };

  if (open) {
    return (
      <div className="panel">
        <button className="btn ghost" onClick={() => setOpen(null)}>
          ← Về hộp thư
        </button>
        <h2 style={{ marginTop: 16 }}>{open.subject || "(không có tiêu đề)"}</h2>
        <p className="sub">
          {open.fromName || open.fromAddress} · {relTime(open.createdAt)}
        </p>
        {open.otp && (
          <div>
            <span
              className="otp-chip"
              style={{ cursor: "pointer" }}
              onClick={() => {
                onCopy(open.otp!);
                notify("Đã chép mã xác thực");
              }}
              title="Bấm để chép"
            >
              {open.otp} ⧉
            </span>
          </div>
        )}
        <div className="mail-body">{open.text || "(thư trống)"}</div>
        <button
          className="btn danger"
          style={{ marginTop: 14 }}
          onClick={() => void removeMail(open.id)}
        >
          Xoá thư này
        </button>
      </div>
    );
  }

  return (
    <div className="panel">
      <h2>Email dùng một lần</h2>
      <p className="sub">
        Địa chỉ thật, nhận được thư thật qua mail.tm. Dùng để đăng ký thử dịch vụ mà không lộ hộp
        thư chính của bạn.
      </p>

      {account ? (
        <>
          <div className="addr-box">
            <div className="addr">{account.address}</div>
            <div className="actions">
              <button
                className="btn primary"
                onClick={() => {
                  onCopy(account.address);
                  notify("Đã chép địa chỉ");
                }}
              >
                Chép
              </button>
              <button className="btn ghost" onClick={() => void refresh()} title="Làm mới">
                {loadingInbox ? <span className="spin" /> : "↻"}
              </button>
            </div>
          </div>

          <div className="row">
            <span className="live">
              <span className="pulse" />
              Đang theo dõi hộp thư · tự kiểm tra mỗi 6 giây
            </span>
            <button className="btn ghost danger" disabled={busy} onClick={() => void destroy()}>
              Xoá địa chỉ
            </button>
          </div>

          <div className="section-title">Hộp thư đến ({messages.length})</div>
          {messages.length === 0 ? (
            <div className="empty">
              Chưa có thư nào.
              <br />
              Thư mới sẽ tự hiện ở đây kèm thông báo Windows.
            </div>
          ) : (
            <div className="mail-list">
              {messages.map((m) => (
                <div
                  key={m.id}
                  className={m.seen ? "mail" : "mail unseen"}
                  onClick={() => void openMail(m.id)}
                >
                  <div className="top">
                    <span className="from">{m.fromName || m.fromAddress}</span>
                    <span className="time">{relTime(m.createdAt)}</span>
                  </div>
                  <div className="subj">{m.subject || "(không có tiêu đề)"}</div>
                  {m.intro && <div className="intro">{m.intro}</div>}
                  {m.otp && (
                    <span
                      className="otp-chip"
                      onClick={(ev) => {
                        ev.stopPropagation();
                        onCopy(m.otp!);
                        notify("Đã chép mã xác thực");
                      }}
                      title="Bấm để chép mã"
                    >
                      {m.otp} ⧉
                    </span>
                  )}
                </div>
              ))}
            </div>
          )}
        </>
      ) : (
        <>
          <div className="section-title">Tên hộp thư (để trống sẽ tự sinh ngẫu nhiên)</div>
          <input
            className="field"
            placeholder="vidu-ten-cua-ban"
            value={localPart}
            spellCheck={false}
            onChange={(e) => setLocalPart(e.target.value.replace(/[^a-zA-Z0-9._-]/g, ""))}
          />

          <div className="section-title">Tên miền</div>
          <select
            className="field"
            value={domain}
            onChange={(e) => setDomain(e.target.value)}
            disabled={domains.length === 0}
          >
            {domains.length === 0 ? (
              <option>
                {loadingDomains ? "Đang tải danh sách…" : "Không tải được danh sách"}
              </option>
            ) : (
              domains.map((d) => (
                <option key={d} value={d}>
                  @{d}
                </option>
              ))
            )}
          </select>
          {domainsError && (
            <div className="field-error">
              <span>Không lấy được danh sách tên miền: {domainsError}</span>
              <button className="btn ghost" onClick={() => void loadDomains()} disabled={loadingDomains}>
                {loadingDomains ? <span className="spin" /> : "Thử lại"}
              </button>
            </div>
          )}

          <button
            className="btn primary wide"
            style={{ marginTop: 20 }}
            disabled={busy || domains.length === 0}
            onClick={() => void create()}
          >
            {busy ? <span className="spin" /> : null}
            Tạo địa chỉ mới
          </button>
          <button className="btn wide ghost" style={{ marginTop: 8 }} onClick={() => void checkConn()}>
            Kiểm tra kết nối tới mail.tm
          </button>
          <p className="sub" style={{ marginTop: 18 }}>
            Lưu ý: đây là hộp thư công khai, ai biết địa chỉ đều đọc được. Đừng dùng cho tài khoản
            ngân hàng, công việc hay bất cứ thứ gì quan trọng.
          </p>
        </>
      )}
    </div>
  );
}
