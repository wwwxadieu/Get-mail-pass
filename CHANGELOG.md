# Changelog

Mọi thay đổi đáng chú ý của PassMail được ghi lại ở đây.
All notable changes to PassMail are documented in this file.

Định dạng theo [Keep a Changelog](https://keepachangelog.com/vi/1.1.0/),
phiên bản theo [Semantic Versioning](https://semver.org/lang/vi/).
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
versioning follows [Semantic Versioning](https://semver.org/).

---

## [Chưa phát hành] · [Unreleased]

Chưa có thay đổi nào. · Nothing yet.

---

## [1.0.1] — 2026-09-02

### Đã thêm · Added

- **VI —** Nhà cung cấp dự phòng cho email tạm: thử lần lượt mail.tm rồi [mail.gw](https://mail.gw). Hai bên dùng chung codebase nên API giống hệt, đã kiểm chứng cả `/domains`, `/accounts`, `/token` và `/messages` trên mail.gw (#4).
  **EN —** Fallback provider for disposable email: tries mail.tm, then [mail.gw](https://mail.gw). Both run the same codebase so the API is identical; `/domains`, `/accounts`, `/token` and `/messages` were all verified against mail.gw (#4).

### Đã đổi · Changed

- **VI —** Tài khoản nhớ luôn nhà cung cấp đã tạo ra nó (`MailAccount.base`), vì tài khoản mail.tm không đăng nhập được ở mail.gw và ngược lại. Mọi lời gọi sau đó — đăng nhập lại, đọc thư, xoá thư, xoá địa chỉ, và vòng lặp theo dõi hộp thư — đều đi đúng nhà cung cấp đó.
  **EN —** An account now records the provider that created it (`MailAccount.base`), because a mail.tm account cannot authenticate against mail.gw or vice versa. Every subsequent call — re-login, reading, deleting mail, deleting the address, and the inbox polling loop — is routed to that same provider.
- **VI —** Thông báo lỗi nêu đúng tên nhà cung cấp đang gặp sự cố thay vì luôn ghi mail.tm; khi cả hai cùng hỏng thì gộp lý do của từng bên.
  **EN —** Error messages name the provider that actually failed instead of always saying mail.tm; when both fail, the reasons from each are combined.
- **VI —** Địa chỉ lưu từ 1.0.0 chưa có thông tin nhà cung cấp sẽ được thử lần lượt từng nhà cung cấp để đăng nhập lại, thay vì bị bỏ.
  **EN —** Addresses saved by 1.0.0, which carry no provider information, are re-authenticated by trying each provider in turn rather than being discarded.

---

## [1.0.0] — 2026-08-31

Bản phát hành đầu tiên. · First public release.

### Đã thêm · Added

**Mật khẩu · Passwords**

- **VI —** Sinh mật khẩu bằng `OsRng`, bộ ngẫu nhiên mật mã của hệ điều hành, chạy hoàn toàn trong tiến trình Rust.
  **EN —** Password generation via `OsRng`, the operating system's cryptographic RNG, entirely inside the Rust process.
- **VI —** Lấy mẫu từ chối (rejection sampling): sinh chuỗi đồng đều rồi loại kết quả thiếu nhóm ký tự, cho phân bố đồng đều tuyệt đối trên tập mật khẩu hợp lệ — thay vì cách "cài sẵn mỗi nhóm một ký tự rồi xáo trộn", vốn thiên vị các mật khẩu cân bằng số ký tự giữa các nhóm.
  **EN —** Rejection sampling: draw uniformly, then discard candidates missing a required class, giving an exactly uniform distribution over valid passwords — rather than the "seed one char per class, then shuffle" approach, which skews toward class-balanced passwords.
- **VI —** Độ dài 4–128 ký tự, bật/tắt từng nhóm (thường, HOA, số, ký hiệu), tuỳ chọn bỏ ký tự dễ nhầm (`0 O 1 l I`) và không lặp ký tự.
  **EN —** Length 4–128, per-class toggles (lower, upper, digits, symbols), optional exclusion of ambiguous characters (`0 O 1 l I`) and no-repeat mode.
- **VI —** Chế độ cụm từ dễ nhớ: 4–15 từ lấy từ danh sách 410 âm tiết tiếng Việt không dấu (mặc định 7 từ ≈ 70 bit).
  **EN —** Memorable passphrase mode: 4–15 words from a 410-syllable unaccented Vietnamese wordlist (default 7 words ≈ 70 bits).
- **VI —** Thanh đo entropy tính bằng bao hàm–loại trừ trên không gian khoá đã bị các ràng buộc thu hẹp, nên thấp hơn — và trung thực hơn — công thức `log₂(bộ ký tự) × độ dài` mà đa số công cụ khác dùng; kèm ước lượng thời gian dò online và offline GPU.
  **EN —** Entropy meter computed by inclusion–exclusion over the constrained key space, so it reads lower — and truer — than the `log₂(charset) × length` formula most tools use; shown with online and offline-GPU cracking-time estimates.
- **VI —** Nút sinh 20 mật khẩu một lượt và chép hàng loạt.
  **EN —** One-click batch of 20 passwords with bulk copy.

**Email dùng một lần · Disposable email**

- **VI —** Hộp thư thật dựa trên [mail.tm](https://mail.tm) — nhận được thư thật, chọn tên ngẫu nhiên hoặc tự đặt, chọn tên miền trong danh sách.
  **EN —** Real inboxes backed by [mail.tm](https://mail.tm) — receives real mail, random or custom local part, domain picked from the live list.
- **VI —** Vòng lặp nền trong Rust kiểm tra hộp thư mỗi 6 giây và bắn thông báo Windows khi có thư mới.
  **EN —** Rust background loop polls the inbox every 6 seconds and raises a Windows notification on new mail.
- **VI —** Phiên hết hạn thì tự đăng nhập lại bằng mật khẩu đã lưu, không im lặng ngừng cập nhật.
  **EN —** On session expiry the app re-authenticates with the stored password instead of silently going stale.
- **VI —** Tự dò mã OTP trong tiêu đề và nội dung thư, hiện thành chip bấm-là-chép.
  **EN —** OTP codes are detected in subject and body and surfaced as click-to-copy chips.
- **VI —** Địa chỉ được nhớ lại sau khi tắt app và khôi phục ở lần mở sau.
  **EN —** The address is remembered across restarts and restored on next launch.

**Chung · General**

- **VI —** Lịch sử giữ 60 mục gần nhất, còn nguyên sau khi tắt app; bấm một mục là chép lại.
  **EN —** History keeps the 60 most recent items across restarts; click an item to copy it again.
- **VI —** Clipboard tự xoá sau 30 giây, và chỉ xoá nếu nội dung vẫn đúng là thứ vừa chép — không xoá nhầm thứ bạn copy sau đó.
  **EN —** Clipboard self-clears after 30 seconds, and only if its content is still what was copied — it will not wipe something you copied afterwards.
- **VI —** Giao diện kính mờ, thanh tiêu đề tự vẽ (cửa sổ không viền), có nền sáng và nền tối.
  **EN —** Frosted-glass UI with a custom-drawn title bar (undecorated window), in light and dark themes.
- **VI —** Toàn bộ lời gọi HTTP đi qua Rust, không qua WebView; CSP chỉ cho phép `'self'`.
  **EN —** All HTTP goes through Rust, never the WebView; the CSP allows `'self'` only.

**Hạ tầng · Infrastructure**

- **VI —** Bộ test `cargo test` phủ độ dài mật khẩu, đủ mọi nhóm ký tự, loại ký tự dễ nhầm, không lặp ký tự, từ chối tuỳ chọn rỗng, số từ passphrase, dấu nối rỗng, danh sách từ không trùng lặp, chống tràn số khi tính thời gian bẻ khoá, lọc thẻ HTML, và nhận diện OTP.
  **EN —** `cargo test` suite covering password length, class coverage, ambiguous-character exclusion, no-repeat mode, empty-option rejection, passphrase word count, empty separator, wordlist uniqueness, overflow safety in crack-time math, HTML tag stripping, and OTP detection.
- **VI —** Workflow CI (`tsc --noEmit`, `vite build`, `cargo fmt`, `cargo clippy -D warnings`, `cargo test`) và workflow Release đóng gói installer NSIS + MSI trên `windows-latest`.
  **EN —** CI workflow (`tsc --noEmit`, `vite build`, `cargo fmt`, `cargo clippy -D warnings`, `cargo test`) and a Release workflow that builds NSIS + MSI installers on `windows-latest`.

### Đã sửa · Fixed

- **VI —** Đọc đúng dữ liệu trả về từ mail.tm (#3).
  **EN —** Correctly parse mail.tm API responses (#3).
- **VI —** Thêm dấu tiếng Việt cho các chuỗi hiển thị trả về từ Rust (#2).
  **EN —** Restore Vietnamese diacritics in user-facing strings returned from Rust (#2).

### Đã biết · Known limitations

- **VI —** Installer chưa được ký số, nên Windows SmartScreen sẽ cảnh báo ở lần chạy đầu.
  **EN —** The installers are unsigned, so Windows SmartScreen warns on first run.
- **VI —** Hộp thư mail.tm là công khai: ai biết địa chỉ đều đọc được thư.
  **EN —** mail.tm inboxes are public: anyone who knows the address can read the mail.
- **VI —** Lịch sử và thông tin hộp thư lưu thô trong localStorage của WebView.
  **EN —** History and mailbox credentials are stored unencrypted in the WebView's localStorage.
- **VI —** Chỉ hỗ trợ Windows x64. Chưa có bản macOS hay Linux.
  **EN —** Windows x64 only. No macOS or Linux build yet.

[Chưa phát hành]: https://github.com/wwwxadieu/Get-mail-pass/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/wwwxadieu/Get-mail-pass/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/wwwxadieu/Get-mail-pass/releases/tag/v1.0.0
