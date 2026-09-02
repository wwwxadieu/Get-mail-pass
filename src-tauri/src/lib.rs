mod mailtm;
mod password;
mod wordlist;

use std::sync::Mutex;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_notification::NotificationExt;

use mailtm::{MailAccount, MailDetail, MailSummary};
use password::{GeneratedPassword, PassphraseOptions, PasswordOptions};

#[derive(Default)]
struct AppState {
    /// Tai khoan mail dang hoat dong
    account: Mutex<Option<MailAccount>>,
    /// Id cac email da bao cho nguoi dung, tranh bao trung
    notified: Mutex<Vec<String>>,
    /// Nha cung cap da tra ve danh sach domain gan nhat — dia chi moi phai duoc
    /// tao dung tren nha cung cap da cung cap ten mien do.
    domain_base: Mutex<Option<String>>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct InboxEvent {
    messages: Vec<MailSummary>,
    new_count: usize,
}

/// Tra ve (base, token) cua tai khoan dang hoat dong. Phai lay ca base chu khong
/// chi token: tai khoan tao o mail.tm khong dung duoc voi mail.gw va nguoc lai.
fn session_of(state: &State<'_, AppState>) -> Result<(String, String), String> {
    state
        .account
        .lock()
        .map_err(|_| "Lỗi trạng thái nội bộ".to_string())?
        .as_ref()
        .map(|a| (a.base.clone(), a.token.clone()))
        .ok_or_else(|| "Chưa có địa chỉ email nào, hãy tạo một địa chỉ trước".to_string())
}

// ---------------- lenh: mat khau ----------------

#[tauri::command]
fn generate_password(options: PasswordOptions) -> Result<GeneratedPassword, String> {
    password::generate_password(&options)
}

#[tauri::command]
fn generate_passphrase(options: PassphraseOptions) -> Result<GeneratedPassword, String> {
    password::generate_passphrase(&options)
}

#[tauri::command]
fn generate_batch(options: PasswordOptions, count: usize) -> Result<Vec<String>, String> {
    let count = count.clamp(1, 100);
    (0..count)
        .map(|_| password::generate_password(&options).map(|p| p.value))
        .collect()
}

// ---------------- lenh: email ----------------

#[tauri::command]
async fn mail_domains(app: AppHandle) -> Result<Vec<String>, String> {
    let (base, domains) = mailtm::list_domains().await?;
    let state = app.state::<AppState>();
    *state
        .domain_base
        .lock()
        .map_err(|_| "Lỗi trạng thái nội bộ")? = Some(base);
    Ok(domains)
}

#[tauri::command]
async fn mail_check_connection() -> Result<String, String> {
    let (base, domains) = mailtm::list_domains().await?;
    Ok(format!(
        "Kết nối tốt qua {} — {} domain khả dụng (ví dụ: {})",
        base.trim_start_matches("https://api."),
        domains.len(),
        domains[0]
    ))
}

#[tauri::command]
async fn mail_create(
    app: AppHandle,
    local_part: Option<String>,
    domain: Option<String>,
) -> Result<MailAccount, String> {
    // Dung lai nha cung cap da tra ve danh sach ten mien; neu chua co (nguoi dung
    // bam tao truoc khi danh sach kip tai) thi tu do lai tu dau.
    let known = {
        let state = app.state::<AppState>();
        let guard = state
            .domain_base
            .lock()
            .map_err(|_| "Lỗi trạng thái nội bộ".to_string())?;
        guard.clone()
    };
    let base = match known {
        Some(b) => b,
        None => mailtm::list_domains().await?.0,
    };
    let acc = mailtm::create_account(&base, local_part, domain).await?;
    let state = app.state::<AppState>();
    *state.account.lock().map_err(|_| "Lỗi trạng thái nội bộ")? = Some(acc.clone());
    state
        .notified
        .lock()
        .map_err(|_| "Lỗi trạng thái nội bộ")?
        .clear();
    Ok(acc)
}

/// Dang nhap lai vao mot dia chi da tao truoc do (khoi phuc sau khi tat app)
#[tauri::command]
async fn mail_restore(
    app: AppHandle,
    id: String,
    address: String,
    password: String,
    base: Option<String>,
) -> Result<MailAccount, String> {
    // Dia chi luu tu ban cu khong co `base`. Thay vi bo luon, thu tung nha cung
    // cap cho den khi dang nhap duoc — dia chi cu van dung tiep duoc.
    let (base, token) = match base {
        Some(b) => {
            let t = mailtm::login(&b, &address, &password).await?;
            (b, t)
        }
        None => {
            let mut last = String::new();
            let mut found = None;
            for b in mailtm::PROVIDERS {
                match mailtm::login(b, &address, &password).await {
                    Ok(t) => {
                        found = Some(((*b).to_string(), t));
                        break;
                    }
                    Err(e) => last = e,
                }
            }
            found.ok_or(last)?
        }
    };
    let acc = MailAccount {
        id,
        address,
        password,
        token,
        base,
    };
    let state = app.state::<AppState>();
    *state.account.lock().map_err(|_| "Lỗi trạng thái nội bộ")? = Some(acc.clone());
    state
        .notified
        .lock()
        .map_err(|_| "Lỗi trạng thái nội bộ")?
        .clear();
    Ok(acc)
}

#[tauri::command]
async fn mail_inbox(app: AppHandle) -> Result<Vec<MailSummary>, String> {
    let (base, token) = session_of(&app.state::<AppState>())?;
    mailtm::list_messages(&base, &token).await
}

#[tauri::command]
async fn mail_read(app: AppHandle, id: String) -> Result<MailDetail, String> {
    let (base, token) = session_of(&app.state::<AppState>())?;
    mailtm::read_message(&base, &token, &id).await
}

#[tauri::command]
async fn mail_delete(app: AppHandle, id: String) -> Result<(), String> {
    let (base, token) = session_of(&app.state::<AppState>())?;
    mailtm::delete_message(&base, &token, &id).await
}

#[tauri::command]
async fn mail_destroy(app: AppHandle) -> Result<(), String> {
    // Lay ban sao roi tha khoa ngay, khong giu MutexGuard qua diem await
    let acc: Option<MailAccount> = {
        let state = app.state::<AppState>();
        let guard = state
            .account
            .lock()
            .map_err(|_| "Lỗi trạng thái nội bộ".to_string())?;
        guard.clone()
    };
    let result = match &acc {
        Some(a) => mailtm::delete_account(&a.base, &a.token, &a.id).await,
        None => Ok(()),
    };
    {
        let state = app.state::<AppState>();
        let mut guard = state
            .account
            .lock()
            .map_err(|_| "Lỗi trạng thái nội bộ".to_string())?;
        *guard = None;
    }
    result
}

// ---------------- vong lap kiem tra hop thu ----------------

fn spawn_poller(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(6));
        ticker.tick().await; // bo qua nhip dau tien (chay ngay lap tuc)
        loop {
            ticker.tick().await;

            // Lay ban sao roi tha khoa ngay, khong giu MutexGuard qua diem await
            let acc = {
                let state = app.state::<AppState>();
                let guard = match state.account.lock() {
                    Ok(g) => g,
                    Err(_) => continue,
                };
                match guard.as_ref() {
                    Some(a) => a.clone(),
                    None => continue,
                }
            };

            let messages = match mailtm::list_messages(&acc.base, &acc.token).await {
                Ok(m) => m,
                Err(e) if e.contains("hết hạn") => {
                    // Token het han. Neu chi bo qua thi hop thu se im lang ngung cap nhat
                    // ma nguoi dung khong biet — nen dang nhap lai bang mat khau da luu.
                    if let Ok(new_token) =
                        mailtm::login(&acc.base, &acc.address, &acc.password).await
                    {
                        let state = app.state::<AppState>();
                        let mut guard = match state.account.lock() {
                            Ok(g) => g,
                            Err(_) => continue,
                        };
                        if let Some(cur) = guard.as_mut() {
                            // chi cap nhat neu nguoi dung chua doi sang dia chi khac
                            if cur.address == acc.address {
                                cur.token = new_token;
                            }
                        }
                    }
                    continue; // thu lai o nhip sau
                }
                Err(_) => continue, // loi mang tam thoi: bo qua nhip nay
            };

            let mut fresh: Vec<MailSummary> = Vec::new();
            {
                let state = app.state::<AppState>();
                let mut seen = match state.notified.lock() {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                for m in &messages {
                    if !seen.contains(&m.id) {
                        seen.push(m.id.clone());
                        fresh.push(m.clone());
                    }
                }
                if seen.len() > 200 {
                    let cut = seen.len() - 200;
                    seen.drain(0..cut);
                }
            }

            if !fresh.is_empty() {
                for m in &fresh {
                    let from = if m.from_name.is_empty() {
                        m.from_address.clone()
                    } else {
                        m.from_name.clone()
                    };
                    let body = match &m.otp {
                        Some(code) => format!("Mã: {} — {}", code, m.subject),
                        None => m.subject.clone(),
                    };
                    let _ = app
                        .notification()
                        .builder()
                        .title(format!("Thư mới từ {}", from))
                        .body(body)
                        .show();
                }
            }

            let _ = app.emit(
                "inbox-updated",
                InboxEvent {
                    new_count: fresh.len(),
                    messages,
                },
            );
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState::default())
        .setup(|app| {
            spawn_poller(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_password,
            generate_passphrase,
            generate_batch,
            mail_domains,
            mail_check_connection,
            mail_create,
            mail_restore,
            mail_inbox,
            mail_read,
            mail_delete,
            mail_destroy,
        ])
        .run(tauri::generate_context!())
        .expect("không khởi động được ứng dụng");
}
