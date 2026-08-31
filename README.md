# PassMail

Ứng dụng Windows gồm hai tính năng: **tạo mật khẩu mạnh** và **email dùng một lần**.
Viết bằng Tauri 2 + React + TypeScript, backend Rust.

---

## Chạy thử

```powershell
npm install
npm run tauri:dev
```

## Đóng gói file cài đặt

```powershell
npm run tauri:build
```

Kết quả nằm ở:

- `src-tauri/target/release/bundle/nsis/PassMail_1.0.0_x64-setup.exe`
- `src-tauri/target/release/bundle/msi/PassMail_1.0.0_x64_en-US.msi`

## Cần cài sẵn

| Thứ cần | Ghi chú |
|---|---|
| Node.js 18+ | https://nodejs.org |
| Rust (stable) | https://rustup.rs → `rustup default stable` |
| Visual Studio Build Tools 2022 | Chọn workload **Desktop development with C++** |
| WebView2 Runtime | Windows 11 có sẵn; Windows 10 tải từ Microsoft nếu thiếu |

---

## Cấu trúc

```
src/                     giao diện React
  App.tsx                khung, tab, lịch sử, toast, dark/light
  components/
    PasswordTab.tsx      tab mật khẩu (ngẫu nhiên + cụm từ)
    MailTab.tsx          tab email tạm + hộp thư
    HistoryPanel.tsx     lịch sử, bấm để chép lại
    TitleBar.tsx         thanh tiêu đề tự vẽ (cửa sổ không viền)
  lib/
    api.ts               bọc các lệnh invoke sang Rust
    clipboard.ts         chép + tự xoá clipboard sau 30s
    types.ts             kiểu dùng chung

src-tauri/src/
  password.rs            sinh mật khẩu, entropy, ước lượng thời gian bẻ khoá
  wordlist.rs            410 âm tiết tiếng Việt không dấu cho passphrase
  mailtm.rs              client HTTP cho api.mail.tm
  lib.rs                 các lệnh Tauri + vòng lặp theo dõi hộp thư
```

---

## Tính năng

**Mật khẩu**

- Sinh bằng `OsRng` — bộ ngẫu nhiên mật mã của hệ điều hành, không phải `Math.random()`.
- **Lấy mẫu từ chối**: sinh chuỗi đồng đều từ bộ ký tự rồi loại kết quả thiếu nhóm, cho phân bố
  đồng đều tuyệt đối trên tập mật khẩu hợp lệ. Không dùng cách "cài sẵn mỗi nhóm một ký tự rồi
  xáo trộn" — cách đó ưu ái các mật khẩu có số ký tự giữa các nhóm cân bằng nhau.
- Độ dài 4–128, bật/tắt từng nhóm ký tự, bỏ ký tự dễ nhầm (`0 O 1 l I`), không lặp ký tự.
- Bảo đảm mỗi nhóm đã chọn đều xuất hiện ít nhất một lần.
- Chế độ cụm từ dễ nhớ: 4–15 từ tiếng Việt không dấu (mặc định 7 từ ≈ 70 bit).
- Thanh đo entropy thật, tính bằng bao hàm–loại trừ trên không gian khoá đã bị ràng buộc thu hẹp
  (thấp hơn công thức log₂(bộ ký tự) × độ dài mà đa số công cụ khác dùng), kèm thời gian dò
  online và offline GPU.
- Nút tạo 20 mật khẩu một lượt và chép hàng loạt.

**Email tạm**

- Dựa trên [mail.tm](https://mail.tm) — hộp thư thật, nhận được thư thật.
- Tự chọn tên ngẫu nhiên hoặc bạn tự đặt; chọn tên miền trong danh sách.
- Vòng lặp nền trong Rust kiểm tra hộp thư mỗi 6 giây, bắn thông báo Windows khi có thư.
  Nếu phiên hết hạn thì tự đăng nhập lại bằng mật khẩu đã lưu, không im lặng ngừng cập nhật.
- Tự dò mã OTP trong tiêu đề và nội dung, hiện thành chip bấm-là-chép.
- Địa chỉ được nhớ lại sau khi tắt app (đăng nhập lại bằng mật khẩu đã lưu cục bộ).

**Chung**

- Lịch sử giữ lại cả sau khi tắt app (60 mục gần nhất), bấm một mục là chép lại.
- Clipboard tự xoá sau 30 giây — và chỉ xoá nếu nội dung vẫn đúng là thứ vừa chép.
- Giao diện kính mờ, có nền sáng và nền tối.

---

## Lưu ý bảo mật

- Mật khẩu **không bao giờ rời khỏi máy** — sinh hoàn toàn trong tiến trình Rust.
- Hộp thư mail.tm là **công khai**: ai biết địa chỉ đều đọc được thư. Đừng dùng cho
  ngân hàng, công việc, hay bất cứ tài khoản nào bạn quan tâm.
- Lịch sử và thông tin hộp thư lưu trong `localStorage` của WebView, dạng thô. Nếu máy
  dùng chung, nhớ bấm "Xoá hết" trước khi rời máy.
- Toàn bộ lời gọi HTTP đi qua Rust, không qua WebView — CSP chỉ cho phép `'self'`.

---

## Kiểm thử

```powershell
cd src-tauri
cargo test
```

Có sẵn test cho: độ dài mật khẩu, đủ mọi nhóm ký tự, loại ký tự dễ nhầm, không lặp ký tự,
từ chối tuỳ chọn rỗng, số từ passphrase, dấu nối rỗng, danh sách từ không trùng lặp,
chống tràn số khi tính thời gian bẻ khoá, lọc thẻ HTML, và nhận diện mã OTP.

---

## CI / CD

Repo có sẵn hai workflow GitHub Actions:

| Workflow | Khi nào chạy | Việc làm |
|---|---|---|
| `.github/workflows/ci.yml` | mỗi push vào `main`, mỗi pull request | `tsc --noEmit` + `vite build`; `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` |
| `.github/workflows/release.yml` | đẩy tag `v*`, hoặc chạy tay | Build trên `windows-latest` bằng `npm run tauri:build`, xuất installer NSIS + MSI |

Muốn phát hành một bản mới:

```powershell
git tag v1.0.0
git push origin v1.0.0
```

Workflow sẽ build và đính kèm `PassMail_1.0.0_x64-setup.exe` cùng `PassMail_1.0.0_x64_en-US.msi`
vào GitHub Release của tag đó. Nếu chỉ muốn lấy file mà chưa phát hành, chạy tay workflow
**Release (Windows installer)** rồi tải ở mục artifact.

Job Rust chạy trên Ubuntu nên cần cài sẵn thư viện GUI của hệ thống
(`libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`,
`libsoup-3.0-dev`) — workflow đã làm sẵn bước này.
