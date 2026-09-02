# PassMail 1.0.1

**VI —** Bản vá. Email tạm nay có nhà cung cấp dự phòng: nếu mail.tm không phản hồi, app tự chuyển sang mail.gw thay vì chịu thua.

**EN —** A patch release. Disposable email now has a fallback provider: if mail.tm does not respond, the app switches to mail.gw instead of giving up.

---

## Tải về · Downloads

| File | Dùng khi · Use it when |
|---|---|
| `PassMail_1.0.1_x64-setup.exe` | **Khuyên dùng.** Bộ cài NSIS, cài cho người dùng hiện tại nên không cần quyền quản trị. · **Recommended.** NSIS installer, installs for the current user, so no administrator rights are needed. |
| `PassMail_1.0.1_x64_en-US.msi` | Gói MSI, hợp khi triển khai bằng công cụ quản lý máy hoặc chính sách nhóm. · MSI package, for deployment via management tooling or group policy. |
| `SHA256SUMS.txt` | Mã băm để đối chiếu file tải về. · Checksums to verify your download. |

**Yêu cầu · Requirements:** Windows 10 hoặc 11, 64-bit. Windows 11 đã có sẵn WebView2 Runtime; Windows 10 nếu thiếu thì tải từ Microsoft.
Windows 10 or 11, 64-bit. Windows 11 ships WebView2 Runtime; on Windows 10, install it from Microsoft if missing.

> **VI —** Installer chưa được ký số, nên lần chạy đầu Windows SmartScreen sẽ hiện cảnh báo. Bấm **More info → Run anyway** nếu bạn tin nguồn tải này.
> **EN —** The installers are unsigned, so Windows SmartScreen will warn on first run. Choose **More info → Run anyway** if you trust this download.

---

## Có gì mới · What changed

### Dự phòng nhà cung cấp email tạm · Disposable-email provider fallback

**VI**

- Trước đây chỉ có mail.tm. Nó chết, quá tải, hoặc bị chặn trên mạng của bạn là tính năng email tạm hết đường dùng.
- Nay app thử lần lượt **mail.tm → mail.gw**. Hai bên dùng chung codebase nên API giống hệt; đã kiểm chứng cả bốn bước `/domains`, `/accounts`, `/token`, `/messages` trên mail.gw.
- Tài khoản tạo ở nhà cung cấp nào thì mọi thao tác sau đó (đăng nhập lại, đọc thư, xoá) đều đi đúng nhà cung cấp đó — vì tài khoản mail.tm không đăng nhập được ở mail.gw và ngược lại.
- Địa chỉ đã lưu từ bản 1.0.0 vẫn dùng tiếp được: app sẽ thử lần lượt từng nhà cung cấp để đăng nhập lại thay vì bỏ luôn.
- Thông báo lỗi nay nêu đúng tên nhà cung cấp đang gặp sự cố; khi cả hai cùng hỏng thì gộp lý do của từng bên để dễ lần ra nguyên nhân.

**EN**

- Previously mail.tm was the only provider. If it went down, got overloaded, or was blocked on your network, disposable email simply stopped working.
- The app now tries **mail.tm → mail.gw** in order. Both run the same codebase, so the API is identical; all four steps (`/domains`, `/accounts`, `/token`, `/messages`) were verified against mail.gw.
- Whichever provider created an account, every later call for it (re-login, reading mail, deletion) goes to that same provider — a mail.tm account cannot log in to mail.gw, and vice versa.
- Addresses saved by 1.0.0 keep working: the app tries each provider in turn to re-authenticate rather than dropping them.
- Error messages now name the provider that actually failed, and when both fail, the reasons from each are combined so the cause is traceable.

---

## Nâng cấp từ 1.0.0 · Upgrading from 1.0.0

**VI —** Cài đè lên bản cũ, không cần gỡ trước. Địa chỉ email và lịch sử đã lưu vẫn giữ nguyên.
**EN —** Install over the existing version; no need to uninstall first. Saved addresses and history are preserved.

---

## Lưu ý bảo mật · Security notes

**VI**

- Mật khẩu sinh hoàn toàn cục bộ, không có lời gọi mạng nào liên quan đến chúng.
- **Hộp thư mail.tm và mail.gw đều công khai:** ai biết địa chỉ đều đọc được thư. Đừng dùng cho ngân hàng, công việc, hay bất cứ tài khoản nào bạn quan tâm.
- Lịch sử và thông tin hộp thư lưu dạng thô trong localStorage của WebView. Máy dùng chung thì nhớ bấm **Xoá hết** trước khi rời máy.

**EN**

- Passwords are generated entirely locally; no network call is involved in producing them.
- **Both mail.tm and mail.gw inboxes are public:** anyone who knows the address can read the mail. Do not use them for banking, work, or any account you care about.
- History and mailbox credentials are stored unencrypted in the WebView's localStorage. On a shared machine, hit **Xoá hết** (Clear all) before you walk away.

---

## Tự build · Build from source

```bash
npm install
npm run tauri:build
```

**VI —** Cần Node.js 18+, Rust stable, Visual Studio Build Tools 2022 (workload *Desktop development with C++*). Chi tiết trong [README](https://github.com/wwwxadieu/Get-mail-pass/blob/main/README.md).
**EN —** Requires Node.js 18+, Rust stable, and Visual Studio Build Tools 2022 (*Desktop development with C++* workload). See the [README](https://github.com/wwwxadieu/Get-mail-pass/blob/main/README.md).

Toàn bộ thay đổi: [CHANGELOG.md](https://github.com/wwwxadieu/Get-mail-pass/blob/main/CHANGELOG.md) · Full history: [CHANGELOG.md](https://github.com/wwwxadieu/Get-mail-pass/blob/main/CHANGELOG.md)
