# PassMail 1.1.1

**VI —** Bản vá nhỏ: bỏ menu chuột phải mặc định của WebView, chỉ giữ lại ở đúng những chỗ cần chép.

**EN —** A small patch: the WebView's default context menu is gone, kept only where copying is the point.

---

## Tải về · Downloads

| File | Dùng khi · Use it when |
|---|---|
| `PassMail_1.1.1_x64-setup.exe` | **Khuyên dùng.** Bộ cài NSIS, cài cho người dùng hiện tại nên không cần quyền quản trị. · **Recommended.** NSIS installer, installs for the current user, so no administrator rights are needed. |
| `PassMail_1.1.1_x64_en-US.msi` | Gói MSI, hợp khi triển khai bằng công cụ quản lý máy hoặc chính sách nhóm. · MSI package, for deployment via management tooling or group policy. |
| `SHA256SUMS.txt` | Mã băm để đối chiếu file tải về. · Checksums to verify your download. |

**Yêu cầu · Requirements:** Windows 10 hoặc 11, 64-bit. Windows 11 đã có sẵn WebView2 Runtime; Windows 10 nếu thiếu thì tải từ Microsoft.
Windows 10 or 11, 64-bit. Windows 11 ships WebView2 Runtime; on Windows 10, install it from Microsoft if missing.

> **VI —** Installer chưa được ký số, nên lần chạy đầu Windows SmartScreen sẽ hiện cảnh báo. Bấm **More info → Run anyway** nếu bạn tin nguồn tải này.
> **EN —** The installers are unsigned, so Windows SmartScreen will warn on first run. Choose **More info → Run anyway** if you trust this download.

---

## Có gì mới · What changed

### Menu chuột phải · Context menu

**VI**

App chạy trên WebView nên thừa hưởng luôn menu chuột phải của trình duyệt — Reload, Back, Inspect. Nó lạc lõng trong một app desktop, và còn mở được DevTools.

Nay menu bị chặn trên toàn app, chỉ chừa lại đúng những chỗ mà chép chính là mục đích:

| Vùng | Menu chuột phải |
|---|---|
| Ô mật khẩu đã sinh | còn |
| Nội dung thư đang mở | còn |
| Ô nhập chữ | còn — chặn ở đây sẽ mất luôn lệnh Dán |
| Nút, nhãn, thư trong danh sách, vùng trống | bỏ |

**EN**

The app runs on a WebView, so it inherited the browser's context menu — Reload, Back, Inspect. That looks out of place in a desktop app and exposes DevTools.

The menu is now suppressed app-wide, kept only where copying is the point:

| Area | Context menu |
|---|---|
| The generated password | kept |
| The open message body | kept |
| Text inputs | kept — removing it would also remove Paste |
| Buttons, labels, message list, empty space | removed |

---

## Nâng cấp từ bản cũ · Upgrading

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
