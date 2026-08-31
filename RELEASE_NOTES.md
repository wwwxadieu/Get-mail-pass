# PassMail 1.0.0

**VI —** Bản phát hành đầu tiên. Một ứng dụng Windows gọn nhẹ gồm hai việc: sinh mật khẩu mạnh và tạo email dùng một lần. Viết bằng Tauri 2 + React + Rust, không dùng Electron.

**EN —** First public release. A small Windows app that does two things: generate strong passwords and create disposable email addresses. Built with Tauri 2 + React + Rust — no Electron.

---

## Tải về · Downloads

| File | Dùng khi · Use it when |
|---|---|
| `PassMail_1.0.0_x64-setup.exe` | **Khuyên dùng.** Bộ cài NSIS, cài cho người dùng hiện tại nên không cần quyền quản trị. · **Recommended.** NSIS installer, installs for the current user, so no administrator rights are needed. |
| `PassMail_1.0.0_x64_en-US.msi` | Gói MSI, hợp khi triển khai bằng công cụ quản lý máy hoặc chính sách nhóm. · MSI package, for deployment via management tooling or group policy. |

**Yêu cầu · Requirements:** Windows 10 hoặc 11, 64-bit. Windows 11 đã có sẵn WebView2 Runtime; Windows 10 nếu thiếu thì tải từ Microsoft.
Windows 10 or 11, 64-bit. Windows 11 ships WebView2 Runtime; on Windows 10, install it from Microsoft if missing.

> **VI —** Installer chưa được ký số, nên lần chạy đầu Windows SmartScreen sẽ hiện cảnh báo. Bấm **More info → Run anyway** nếu bạn tin nguồn tải này.
> **EN —** The installers are unsigned, so Windows SmartScreen will warn on first run. Choose **More info → Run anyway** if you trust this download.

---

## Có gì trong bản này · What's inside

### Mật khẩu · Passwords

**VI**

- Sinh bằng `OsRng` — bộ ngẫu nhiên mật mã của hệ điều hành, chạy hoàn toàn trong tiến trình Rust. Mật khẩu không bao giờ rời khỏi máy.
- Lấy mẫu từ chối cho phân bố đồng đều tuyệt đối trên tập mật khẩu hợp lệ, thay vì cách "cài sẵn mỗi nhóm một ký tự rồi xáo trộn" vốn thiên vị.
- Độ dài 4–128, bật/tắt từng nhóm ký tự, bỏ ký tự dễ nhầm (`0 O 1 l I`), tuỳ chọn không lặp ký tự.
- Cụm từ dễ nhớ: 4–15 từ từ danh sách 410 âm tiết tiếng Việt không dấu (mặc định 7 từ ≈ 70 bit).
- Thanh đo entropy thật, tính trên không gian khoá đã bị ràng buộc thu hẹp — thấp hơn và trung thực hơn công thức `log₂(bộ ký tự) × độ dài` mà đa số công cụ khác dùng.
- Sinh 20 mật khẩu một lượt và chép hàng loạt.

**EN**

- Generated with `OsRng`, the OS cryptographic RNG, entirely inside the Rust process. Passwords never leave the machine.
- Rejection sampling gives an exactly uniform distribution over valid passwords, instead of the biased "seed one char per class, then shuffle" trick.
- Length 4–128, per-class toggles, ambiguous-character exclusion (`0 O 1 l I`), optional no-repeat mode.
- Memorable passphrases: 4–15 words from a 410-syllable unaccented Vietnamese wordlist (default 7 words ≈ 70 bits).
- A real entropy meter computed over the constrained key space — lower and more honest than the `log₂(charset) × length` figure most tools show.
- Batch of 20 passwords with bulk copy.

### Email dùng một lần · Disposable email

**VI**

- Hộp thư thật dựa trên mail.tm — nhận được thư thật. Tên ngẫu nhiên hoặc tự đặt, chọn tên miền trong danh sách.
- Vòng lặp nền trong Rust kiểm tra hộp thư mỗi 6 giây, bắn thông báo Windows khi có thư mới.
- Phiên hết hạn thì tự đăng nhập lại, không im lặng ngừng cập nhật.
- Tự dò mã OTP trong tiêu đề và nội dung, hiện thành chip bấm-là-chép.
- Địa chỉ được nhớ lại sau khi tắt app.

**EN**

- Real inboxes backed by mail.tm — they receive real mail. Random or custom name, domain picked from the live list.
- A Rust background loop polls every 6 seconds and raises a Windows notification on new mail.
- Expired sessions re-authenticate automatically instead of silently going stale.
- OTP codes are detected in subject and body and shown as click-to-copy chips.
- The address is remembered across restarts.

### Chung · General

**VI**

- Lịch sử 60 mục gần nhất, còn nguyên sau khi tắt app; bấm một mục là chép lại.
- Clipboard tự xoá sau 30 giây — và chỉ xoá nếu nội dung vẫn đúng là thứ vừa chép.
- Giao diện kính mờ, thanh tiêu đề tự vẽ, có nền sáng và nền tối.

**EN**

- History of the 60 most recent items, kept across restarts; click to copy again.
- Clipboard self-clears after 30 seconds — and only if its content is still what was copied.
- Frosted-glass UI with a custom title bar, in light and dark themes.

---

## Lưu ý bảo mật · Security notes

**VI**

- Mật khẩu sinh hoàn toàn cục bộ, không có lời gọi mạng nào liên quan đến chúng.
- **Hộp thư mail.tm là công khai:** ai biết địa chỉ đều đọc được thư. Đừng dùng cho ngân hàng, công việc, hay bất cứ tài khoản nào bạn quan tâm.
- Lịch sử và thông tin hộp thư lưu dạng thô trong localStorage của WebView. Máy dùng chung thì nhớ bấm **Xoá hết** trước khi rời máy.
- Toàn bộ lời gọi HTTP đi qua Rust, không qua WebView; CSP chỉ cho phép `'self'`.

**EN**

- Passwords are generated entirely locally; no network call is involved in producing them.
- **mail.tm inboxes are public:** anyone who knows the address can read the mail. Do not use it for banking, work, or any account you care about.
- History and mailbox credentials are stored unencrypted in the WebView's localStorage. On a shared machine, hit **Xoá hết** (Clear all) before you walk away.
- All HTTP goes through Rust, never the WebView; the CSP allows `'self'` only.

---

## Tự build · Build from source

```bash
npm install
npm run tauri:build
```

**VI —** Cần Node.js 18+, Rust stable, Visual Studio Build Tools 2022 (workload *Desktop development with C++*). Chi tiết trong [README](https://github.com/wwwxadieu/Get-mail-pass/blob/main/README.md).
**EN —** Requires Node.js 18+, Rust stable, and Visual Studio Build Tools 2022 (*Desktop development with C++* workload). See the [README](https://github.com/wwwxadieu/Get-mail-pass/blob/main/README.md).

Toàn bộ thay đổi: [CHANGELOG.md](https://github.com/wwwxadieu/Get-mail-pass/blob/main/CHANGELOG.md) · Full history: [CHANGELOG.md](https://github.com/wwwxadieu/Get-mail-pass/blob/main/CHANGELOG.md)
