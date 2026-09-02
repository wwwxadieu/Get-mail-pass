# PassMail 1.1.0

**VI —** Giao diện được làm lại theo Apple Human Interface Guidelines, nền dịu hẳn, và app chạy nhẹ hơn rõ rệt. Email tạm nay có nhà cung cấp dự phòng.

**EN —** The interface was reworked against Apple's Human Interface Guidelines, the background is much calmer, and the app runs noticeably lighter. Disposable email now has a fallback provider.

---

## Tải về · Downloads

| File | Dùng khi · Use it when |
|---|---|
| `PassMail_1.1.0_x64-setup.exe` | **Khuyên dùng.** Bộ cài NSIS, cài cho người dùng hiện tại nên không cần quyền quản trị. · **Recommended.** NSIS installer, installs for the current user, so no administrator rights are needed. |
| `PassMail_1.1.0_x64_en-US.msi` | Gói MSI, hợp khi triển khai bằng công cụ quản lý máy hoặc chính sách nhóm. · MSI package, for deployment via management tooling or group policy. |
| `SHA256SUMS.txt` | Mã băm để đối chiếu file tải về. · Checksums to verify your download. |

**Yêu cầu · Requirements:** Windows 10 hoặc 11, 64-bit. Windows 11 đã có sẵn WebView2 Runtime; Windows 10 nếu thiếu thì tải từ Microsoft.
Windows 10 or 11, 64-bit. Windows 11 ships WebView2 Runtime; on Windows 10, install it from Microsoft if missing.

> **VI —** Installer chưa được ký số, nên lần chạy đầu Windows SmartScreen sẽ hiện cảnh báo. Bấm **More info → Run anyway** nếu bạn tin nguồn tải này.
> **EN —** The installers are unsigned, so Windows SmartScreen will warn on first run. Choose **More info → Run anyway** if you trust this download.

---

## Có gì mới · What changed

### Giao diện theo chuẩn Apple · Apple-standard interface

**VI**

- Dùng bảng màu hệ thống của Apple (systemBlue, systemGray, systemGreen…), thang chữ 13px của macOS, nhịp giãn cách 4pt, bo góc 12px cho thẻ và 8px cho điều khiển.
- Nền dịu hẳn: ba khối gradient động đã đổi thành một lớp wash tĩnh rất nhạt — đủ có chiều sâu mà không tranh chỗ với nội dung.
- Nút bấm đổi nền khi trỏ chuột thay vì nhấc lên và đổ bóng. Công tắc chuyển sang màu xanh lá kiểu Apple. Có vòng focus rõ ràng cho người dùng bàn phím.
- Nút phụ cạnh nút chính (tạo lại, làm mới) nay có viền — trước đó trong suốt hoàn toàn nên nhìn không ra là bấm được.

*Ghi chú:* SF Pro không có sẵn trên Windows và không được phép đóng gói kèm, nên bản này dùng đúng thang chữ và độ dày của Apple nhưng để font hệ thống của Windows (Segoe UI Variable) đảm nhận — đó mới là lựa chọn đúng cho máy đích.

**EN**

- Apple's system palette (systemBlue, systemGray, systemGreen…), the 13px macOS type scale, a 4pt spacing rhythm, and 12px card / 8px control radii.
- A much calmer background: three animated gradient blobs became a single, very faint static wash — enough depth without competing with the content.
- Buttons change fill on hover instead of lifting with a shadow. Switches use Apple's green. Keyboard users get a clear focus ring.
- Secondary buttons beside a primary action (regenerate, refresh) now have a border — previously fully transparent, so they did not read as clickable.

*Note:* SF Pro is not available on Windows and cannot be bundled, so this release follows Apple's type scale and weights while letting Windows' own system font (Segoe UI Variable) do the rendering — the correct choice for the target platform.

### Nhẹ hơn và ổn định hơn · Lighter and steadier

**VI**

- Bỏ `backdrop-filter: blur(28px)` trên các panel. Panel có thanh cuộn, nên mỗi khung hình khi cuộn đều phải vẽ lại vùng mờ — đây là nguồn giật chính.
- Debounce việc sinh lại mật khẩu 140 ms. Kéo thanh độ dài 51 bước trước đây bắn 51 lời gọi sang Rust, mỗi lời gọi còn lấy mẫu từ chối; nay chỉ còn **1** — đo bằng bộ đếm trên bản chạy thật, không phải ước lượng.
- Mỗi yêu cầu sinh mật khẩu được đánh số: kết quả cũ không còn ghi đè kết quả mới, nên mật khẩu hiện ra luôn khớp với tuỳ chọn đang bật.

**EN**

- Dropped `backdrop-filter: blur(28px)` from the panels. They scroll, so every frame during a scroll had to re-render the blurred region — the main source of jank.
- Password regeneration is debounced by 140 ms. Dragging the length slider 51 steps previously fired 51 calls into Rust, each running rejection sampling; it now fires **1** — measured with a counter on a live build, not estimated.
- Each generation request is sequenced, so an older result can no longer overwrite a newer one; the password shown always matches the active options.

### Dự phòng nhà cung cấp email tạm · Disposable-email provider fallback

**VI**

- Trước đây chỉ có mail.tm. Nó chết, quá tải, hoặc bị chặn trên mạng của bạn là tính năng email tạm hết đường dùng.
- Nay app thử lần lượt **mail.tm → mail.gw**. Hai bên dùng chung codebase nên API giống hệt; đã kiểm chứng cả bốn bước `/domains`, `/accounts`, `/token`, `/messages` trên mail.gw.
- Tài khoản tạo ở nhà cung cấp nào thì mọi thao tác sau đó đều đi đúng nhà cung cấp đó — vì tài khoản mail.tm không đăng nhập được ở mail.gw và ngược lại.
- Địa chỉ đã lưu từ bản trước vẫn dùng tiếp được: app thử lần lượt từng nhà cung cấp để đăng nhập lại thay vì bỏ luôn.

**EN**

- Previously mail.tm was the only provider. If it went down, got overloaded, or was blocked on your network, disposable email simply stopped working.
- The app now tries **mail.tm → mail.gw** in order. Both run the same codebase, so the API is identical; all four steps (`/domains`, `/accounts`, `/token`, `/messages`) were verified against mail.gw.
- Whichever provider created an account, every later call for it goes to that same provider — a mail.tm account cannot log in to mail.gw, and vice versa.
- Addresses saved by earlier versions keep working: the app tries each provider in turn to re-authenticate rather than dropping them.

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
