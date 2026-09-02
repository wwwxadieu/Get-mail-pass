import { useCallback, useEffect, useRef, useState } from "react";
import { api, errText } from "../lib/api";
import type { GeneratedPassword, PassphraseOptions, PasswordOptions } from "../lib/types";
import Switch from "./Switch";

type Mode = "password" | "passphrase";

const DEFAULT_PW: PasswordOptions = {
  length: 20,
  lowercase: true,
  uppercase: true,
  digits: true,
  symbols: true,
  avoidAmbiguous: false,
  noRepeat: false,
};

const DEFAULT_PP: PassphraseOptions = {
  words: 7,
  separator: "-",
  capitalize: true,
  addNumber: true,
};

type Props = {
  onGenerated: (kind: "password" | "passphrase", value: string, note: string) => void;
  onCopy: (value: string) => void;
  notify: (text: string, kind?: "ok" | "err") => void;
};

/** Tô màu chữ số và ký hiệu để dễ đọc mật khẩu */
function Colored({ value }: { value: string }) {
  return (
    <>
      {[...value].map((ch, i) => {
        const cls = /[0-9]/.test(ch) ? "d" : /[a-zA-Z]/.test(ch) ? "" : "s";
        return (
          <span key={i} className={cls}>
            {ch}
          </span>
        );
      })}
    </>
  );
}

export default function PasswordTab({ onGenerated, onCopy, notify }: Props) {
  const [mode, setMode] = useState<Mode>("password");
  const [pw, setPw] = useState<PasswordOptions>(DEFAULT_PW);
  const [pp, setPp] = useState<PassphraseOptions>(DEFAULT_PP);
  const [result, setResult] = useState<GeneratedPassword | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const classCount = [pw.lowercase, pw.uppercase, pw.digits, pw.symbols].filter(Boolean).length;

  // Đánh số từng yêu cầu: nếu hai lần sinh chồng nhau, kết quả về sau mới được
  // hiển thị. Không có nó thì một lời gọi chậm có thể ghi đè kết quả mới hơn,
  // làm mật khẩu hiện ra không khớp với tuỳ chọn đang bật.
  const reqId = useRef(0);

  const generate = useCallback(
    async (record: boolean) => {
      const id = ++reqId.current;
      setBusy(true);
      try {
        const r =
          mode === "password"
            ? await api.generatePassword(pw)
            : await api.generatePassphrase(pp);
        if (id !== reqId.current) return; // đã có yêu cầu mới hơn
        setResult(r);
        setError(null);
        if (record) {
          onGenerated(mode, r.value, `${r.label} · ${r.entropyBits} bit`);
        }
      } catch (e) {
        if (id !== reqId.current) return;
        // Hiện lỗi ngay trong ô kết quả, không để mật khẩu cũ nằm lại gây hiểu nhầm
        setResult(null);
        setError(errText(e));
      } finally {
        if (id === reqId.current) setBusy(false);
      }
    },
    [mode, pw, pp, onGenerated],
  );

  // Sinh lại mỗi khi đổi tuỳ chọn (không ghi vào lịch sử để tránh spam).
  // Kéo thanh độ dài từ 20 lên 128 làm state đổi hơn trăm lần; sinh ngay lập tức
  // nghĩa là hơn trăm lời gọi IPC sang Rust, mỗi lời gọi còn lấy mẫu từ chối.
  // Chờ lắng 140ms rồi mới sinh — mắt không thấy trễ, máy đỡ hẳn.
  // Lần đầu thì sinh ngay, để mở app lên là đã có sẵn mật khẩu.
  const firstRun = useRef(true);
  useEffect(() => {
    if (classCount === 0) return;
    if (firstRun.current) {
      firstRun.current = false;
      void generate(false);
      return;
    }
    const t = window.setTimeout(() => void generate(false), 140);
    return () => window.clearTimeout(t);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mode, pw, pp]);

  const exportBatch = async () => {
    try {
      const list = await api.generateBatch(pw, 20);
      onCopy(list.join("\n"));
      notify("Đã chép 20 mật khẩu vào clipboard");
    } catch (e) {
      notify(errText(e), "err");
    }
  };

  return (
    <div className="panel">
      <h2>Tạo mật khẩu mạnh</h2>
      <p className="sub">
        Sinh bằng bộ ngẫu nhiên của hệ điều hành (CSPRNG), phân bố đồng đều tuyệt đối — không có
        quy tắc hay khuôn mẫu nào. Không lưu lên mạng, không rời khỏi máy bạn.
      </p>

      <div className="seg" style={{ marginBottom: 16 }}>
        <button className={mode === "password" ? "on" : ""} onClick={() => setMode("password")}>
          Ngẫu nhiên
        </button>
        <button className={mode === "passphrase" ? "on" : ""} onClick={() => setMode("passphrase")}>
          Cụm từ dễ nhớ
        </button>
      </div>

      <div className="result">
        <div className="value" data-copyable>
          {error ? (
            <span style={{ color: "var(--danger)", fontSize: 14, fontFamily: "inherit" }}>
              {error}
            </span>
          ) : result ? (
            <Colored value={result.value} />
          ) : (
            <span style={{ opacity: 0.4 }}>—</span>
          )}
        </div>
        <div className="actions">
          <button
            className="btn ghost"
            onClick={() => void generate(false)}
            disabled={busy || classCount === 0}
            title="Tạo lại"
          >
            {busy ? <span className="spin" /> : "↻"}
          </button>
          <button
            className="btn primary"
            disabled={!result}
            onClick={() => {
              if (!result) return;
              onCopy(result.value);
              onGenerated(mode, result.value, `${result.label} · ${result.entropyBits} bit`);
            }}
          >
            Chép
          </button>
        </div>
      </div>

      {result && (
        <div className="meter">
          <div className="meter-bar">
            {[0, 1, 2, 3, 4].map((i) => (
              <span
                key={i}
                className={i <= result.score ? `meter-seg on-${result.score}` : "meter-seg"}
              />
            ))}
          </div>
          <div className="meter-info">
            <strong>{result.label}</strong>
            <span>
              {result.entropyBits} bit entropy · bộ ký tự {result.poolSize}
            </span>
          </div>
          <div className="crack">
            <div>
              <span>Dò qua mạng (100 lần/giây)</span>
              <b>{result.crackTimeOnline}</b>
            </div>
            <div>
              <span>Dò offline bằng GPU (10¹¹/giây)</span>
              <b>{result.crackTimeOffline}</b>
            </div>
          </div>
        </div>
      )}

      {mode === "password" ? (
        <>
          <div className="section-title">Độ dài</div>
          <div className="len-head">
            <b>{pw.length}</b>
            <span style={{ color: "var(--text-3)", fontSize: 12 }}>4 – 128 ký tự</span>
          </div>
          <input
            type="range"
            min={4}
            max={128}
            value={pw.length}
            onChange={(e) => setPw({ ...pw, length: Number(e.target.value) })}
          />

          <div className="section-title">Nhóm ký tự</div>
          <Switch
            label="Chữ thường (a–z)"
            checked={pw.lowercase}
            disabled={pw.lowercase && classCount === 1}
            onChange={(v) => setPw({ ...pw, lowercase: v })}
          />
          <Switch
            label="Chữ hoa (A–Z)"
            checked={pw.uppercase}
            disabled={pw.uppercase && classCount === 1}
            onChange={(v) => setPw({ ...pw, uppercase: v })}
          />
          <Switch
            label="Chữ số (0–9)"
            checked={pw.digits}
            disabled={pw.digits && classCount === 1}
            onChange={(v) => setPw({ ...pw, digits: v })}
          />
          <Switch
            label="Ký hiệu (!@#$…)"
            checked={pw.symbols}
            disabled={pw.symbols && classCount === 1}
            onChange={(v) => setPw({ ...pw, symbols: v })}
          />

          <div className="section-title">Tuỳ chọn</div>
          <Switch
            label="Bỏ ký tự dễ nhầm (0 O 1 l I)"
            checked={pw.avoidAmbiguous}
            onChange={(v) => setPw({ ...pw, avoidAmbiguous: v })}
          />
          <Switch
            label="Không lặp lại ký tự"
            checked={pw.noRepeat}
            onChange={(v) => setPw({ ...pw, noRepeat: v })}
          />

          <button className="btn wide" style={{ marginTop: 18 }} onClick={() => void exportBatch()}>
            Tạo 20 mật khẩu và chép hàng loạt
          </button>
          <p className="sub" style={{ marginTop: 18 }}>
            Entropy hiển thị là số bit thật của không gian khoá, đã trừ phần bị thu hẹp do ràng
            buộc &quot;mỗi nhóm xuất hiện ít nhất một lần&quot; — nên nó thấp hơn một chút so với
            công thức log₂(bộ ký tự) × độ dài mà đa số công cụ khác dùng.
          </p>
        </>
      ) : (
        <>
          <div className="section-title">Số từ</div>
          <div className="len-head">
            <b>{pp.words}</b>
            <span style={{ color: "var(--text-3)", fontSize: 12 }}>4 – 15 từ</span>
          </div>
          <input
            type="range"
            min={4}
            max={15}
            value={pp.words}
            onChange={(e) => setPp({ ...pp, words: Number(e.target.value) })}
          />

          <div className="section-title">Dấu nối</div>
          <div className="seg">
            {["-", ".", "_", " ", ""].map((s) => (
              <button
                key={s || "none"}
                className={pp.separator === s ? "on" : ""}
                onClick={() => setPp({ ...pp, separator: s })}
              >
                {s === "" ? "liền" : s === " " ? "cách" : s}
              </button>
            ))}
          </div>

          <div className="section-title">Tuỳ chọn</div>
          <Switch
            label="Viết hoa chữ cái đầu"
            checked={pp.capitalize}
            onChange={(v) => setPp({ ...pp, capitalize: v })}
          />
          <Switch
            label="Chèn thêm một con số"
            checked={pp.addNumber}
            onChange={(v) => setPp({ ...pp, addNumber: v })}
          />
          <p className="sub" style={{ marginTop: 18 }}>
            Cụm từ ghép từ 410 âm tiết tiếng Việt không dấu — dễ đọc, dễ gõ, và vẫn khó đoán vì
            các từ được chọn hoàn toàn ngẫu nhiên.
          </p>
        </>
      )}
    </div>
  );
}
