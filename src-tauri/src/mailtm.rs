use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use rand::Rng;
use regex::Regex;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const BASE: &str = "https://api.mail.tm";

static HTTP: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent("PassMail/1.0")
        .build()
        .expect("không tạo được HTTP client")
});

static OTP_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{4,8})\b").expect("biểu thức chính quy OTP_RE không hợp lệ"));
// crate `regex` khong ho tro backreference nen phai viet tuong minh tung the mot
static TAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>|<!--.*?-->|<[^>]+>")
        .expect("biểu thức chính quy TAG_RE không hợp lệ")
});

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MailAccount {
    pub id: String,
    pub address: String,
    pub password: String,
    pub token: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MailSummary {
    pub id: String,
    pub from_name: String,
    pub from_address: String,
    pub subject: String,
    pub intro: String,
    pub seen: bool,
    pub created_at: String,
    pub has_attachments: bool,
    /// Ma OTP doan duoc tu tieu de hoac doan mo dau, neu co
    pub otp: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MailDetail {
    pub id: String,
    pub subject: String,
    pub from_name: String,
    pub from_address: String,
    pub created_at: String,
    pub text: String,
    pub otp: Option<String>,
}

// ---------- kieu du lieu tra ve tu API ----------

#[derive(Deserialize)]
struct HydraList<T> {
    #[serde(rename = "hydra:member", default = "Vec::new")]
    member: Vec<T>,
}

#[derive(Deserialize)]
struct ApiDomain {
    domain: String,
    #[serde(rename = "isActive", default)]
    is_active: bool,
    #[serde(rename = "isPrivate", default)]
    is_private: bool,
}

#[derive(Deserialize)]
struct ApiAccount {
    id: String,
    address: String,
}

#[derive(Deserialize)]
struct ApiToken {
    token: String,
    id: String,
}

#[derive(Deserialize)]
struct ApiAddress {
    #[serde(default)]
    address: String,
    #[serde(default)]
    name: String,
}

#[derive(Deserialize)]
struct ApiMessage {
    id: String,
    #[serde(default)]
    from: Option<ApiAddress>,
    #[serde(default)]
    subject: String,
    #[serde(default)]
    intro: String,
    #[serde(default)]
    seen: bool,
    #[serde(rename = "createdAt", default)]
    created_at: String,
    #[serde(rename = "hasAttachments", default)]
    has_attachments: bool,
}

#[derive(Deserialize)]
struct ApiMessageDetail {
    id: String,
    #[serde(default)]
    from: Option<ApiAddress>,
    #[serde(default)]
    subject: String,
    #[serde(rename = "createdAt", default)]
    created_at: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    html: Vec<String>,
}

// ---------- ham tien ich ----------

fn err(msg: impl Into<String>) -> String {
    msg.into()
}

fn map_status(code: StatusCode) -> String {
    match code {
        StatusCode::TOO_MANY_REQUESTS => err("mail.tm đang giới hạn tốc độ, thử lại sau vài giây"),
        StatusCode::UNAUTHORIZED => err("Phiên đăng nhập hết hạn, hãy tạo địa chỉ mới"),
        StatusCode::NOT_FOUND => err("Không tìm thấy dữ liệu trên máy chủ"),
        StatusCode::UNPROCESSABLE_ENTITY => {
            err("Máy chủ từ chối dữ liệu (địa chỉ đã tồn tại hoặc domain không hợp lệ)")
        }
        c => format!("Máy chủ trả về lỗi {}", c.as_u16()),
    }
}

fn random_local_part() -> String {
    const ALPHABET: &[u8] = b"abcdefghijkmnopqrstuvwxyz23456789";
    let mut rng = OsRng;
    let len = rng.gen_range(10..=14);
    (0..len)
        .map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char)
        .collect()
}

fn random_password() -> String {
    const ALPHABET: &[u8] = b"abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = OsRng;
    (0..20)
        .map(|_| ALPHABET[rng.gen_range(0..ALPHABET.len())] as char)
        .collect()
}

fn html_to_text(html: &str) -> String {
    let stripped = TAG_RE.replace_all(html, " ");
    let decoded = stripped
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Chuoi 4 so giong nam (2026) hoac 8 so giong ngay YYYYMMDD (20260101)
fn looks_like_date(s: &str) -> bool {
    (s.len() == 4 || s.len() == 8) && (s.starts_with("19") || s.starts_with("20"))
}

/// Tim ma xac thuc: uu tien doan van co tu khoa code/otp/ma
fn extract_otp(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    // "pin" phai khop nguyen tu — neu khong thi "shipping" cung bi tinh la tu khoa
    let has_keyword = [
        "code",
        "otp",
        "verification",
        "verify",
        "ma xac",
        "mã xác",
        "confirm",
    ]
    .iter()
    .any(|k| lower.contains(k))
        || lower
            .split(|c: char| !c.is_alphanumeric())
            .any(|w| w == "pin");

    let mut candidates: Vec<String> = OTP_RE
        .captures_iter(text)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        // Ngay dang YYYYMMDD gan nhu chac chan khong phai ma xac thuc.
        // Rieng chuoi 4 so giong nam thi van giu lai neu email co tu khoa:
        // mot ma PIN hop le hoan toan co the la 2048.
        .filter(|s| !(s.len() == 8 && looks_like_date(s)) && (has_keyword || !looks_like_date(s)))
        .collect();
    if candidates.is_empty() {
        return None;
    }
    if !has_keyword {
        // khong co tu khoa: chi nhan ma 6 chu so
        candidates.retain(|s| s.len() == 6);
        if candidates.is_empty() {
            return None;
        }
    }
    // Do dai pho bien nhat cua ma xac thuc: 6 -> 4 -> 8 -> con lai.
    // Chuoi giong nam luon bi xep sau cung: chi duoc chon khi khong con gi khac.
    candidates.sort_by_key(|s| {
        let by_len = match s.len() {
            6 => 0,
            4 => 1,
            8 => 2,
            _ => 3,
        };
        (looks_like_date(s) as u8, by_len)
    });
    Some(candidates.remove(0))
}

// ---------- cac loi goi API ----------

pub async fn list_domains() -> Result<Vec<String>, String> {
    let resp = HTTP
        .get(format!("{BASE}/domains?page=1"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if !resp.status().is_success() {
        return Err(map_status(resp.status()));
    }
    let list: HydraList<ApiDomain> = resp
        .json()
        .await
        .map_err(|e| format!("Dữ liệu domain không hợp lệ: {e}"))?;
    let domains: Vec<String> = list
        .member
        .into_iter()
        .filter(|d| d.is_active && !d.is_private)
        .map(|d| d.domain)
        .collect();
    if domains.is_empty() {
        return Err(err("mail.tm hiện không có domain khả dụng"));
    }
    Ok(domains)
}

/// Tao dia chi moi. `desired_local` de trong thi sinh ngau nhien.
pub async fn create_account(
    desired_local: Option<String>,
    desired_domain: Option<String>,
) -> Result<MailAccount, String> {
    let domains = list_domains().await?;
    let domain = match desired_domain {
        Some(d) if domains.iter().any(|x| x == &d) => d,
        _ => domains[0].clone(),
    };

    let password = random_password();
    let mut last_err = String::new();

    for attempt in 0..4 {
        let local = match (&desired_local, attempt) {
            (Some(l), 0) if !l.trim().is_empty() => l.trim().to_lowercase(),
            _ => random_local_part(),
        };
        let address = format!("{local}@{domain}");

        let resp = HTTP
            .post(format!("{BASE}/accounts"))
            .json(&serde_json::json!({ "address": address, "password": password }))
            .send()
            .await
            .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;

        if resp.status().is_success() {
            let acc: ApiAccount = resp
                .json()
                .await
                .map_err(|e| format!("Dữ liệu tài khoản không hợp lệ: {e}"))?;
            let token = login(&acc.address, &password).await?;
            return Ok(MailAccount {
                id: acc.id,
                address: acc.address,
                password,
                token,
            });
        }
        last_err = map_status(resp.status());
        if resp_status_is_conflict(&last_err) {
            continue; // dia chi trung, thu ten khac
        }
        return Err(last_err);
    }
    Err(last_err)
}

fn resp_status_is_conflict(msg: &str) -> bool {
    msg.contains("đã tồn tại")
}

pub async fn login(address: &str, password: &str) -> Result<String, String> {
    let resp = HTTP
        .post(format!("{BASE}/token"))
        .json(&serde_json::json!({ "address": address, "password": password }))
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if !resp.status().is_success() {
        return Err(map_status(resp.status()));
    }
    let t: ApiToken = resp
        .json()
        .await
        .map_err(|e| format!("Dữ liệu token không hợp lệ: {e}"))?;
    let _ = t.id;
    Ok(t.token)
}

pub async fn list_messages(token: &str) -> Result<Vec<MailSummary>, String> {
    let resp = HTTP
        .get(format!("{BASE}/messages?page=1"))
        .bearer_auth(token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if !resp.status().is_success() {
        return Err(map_status(resp.status()));
    }
    let list: HydraList<ApiMessage> = resp
        .json()
        .await
        .map_err(|e| format!("Dữ liệu hộp thư không hợp lệ: {e}"))?;
    Ok(list
        .member
        .into_iter()
        .map(|m| {
            let (from_name, from_address) = m.from.map(|f| (f.name, f.address)).unwrap_or_default();
            let otp = extract_otp(&format!("{} {}", m.subject, m.intro));
            MailSummary {
                id: m.id,
                from_name,
                from_address,
                subject: m.subject,
                intro: m.intro,
                seen: m.seen,
                created_at: m.created_at,
                has_attachments: m.has_attachments,
                otp,
            }
        })
        .collect())
}

pub async fn read_message(token: &str, id: &str) -> Result<MailDetail, String> {
    let resp = HTTP
        .get(format!("{BASE}/messages/{id}"))
        .bearer_auth(token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if !resp.status().is_success() {
        return Err(map_status(resp.status()));
    }
    let m: ApiMessageDetail = resp
        .json()
        .await
        .map_err(|e| format!("Dữ liệu email không hợp lệ: {e}"))?;
    let mut text = m.text.trim().to_string();
    if text.is_empty() && !m.html.is_empty() {
        text = html_to_text(&m.html.join("\n"));
    }
    let (from_name, from_address) = m.from.map(|f| (f.name, f.address)).unwrap_or_default();
    let otp = extract_otp(&format!("{} {}", m.subject, text));
    Ok(MailDetail {
        id: m.id,
        subject: m.subject,
        from_name,
        from_address,
        created_at: m.created_at,
        text,
        otp,
    })
}

pub async fn delete_message(token: &str, id: &str) -> Result<(), String> {
    let resp = HTTP
        .delete(format!("{BASE}/messages/{id}"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if resp.status().is_success() || resp.status() == StatusCode::NO_CONTENT {
        Ok(())
    } else {
        Err(map_status(resp.status()))
    }
}

pub async fn delete_account(token: &str, id: &str) -> Result<(), String> {
    let resp = HTTP
        .delete(format!("{BASE}/accounts/{id}"))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Không kết nối được mail.tm: {e}"))?;
    if resp.status().is_success() || resp.status() == StatusCode::NO_CONTENT {
        Ok(())
    } else {
        Err(map_status(resp.status()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_part_hop_le() {
        for _ in 0..100 {
            let l = random_local_part();
            assert!(l.len() >= 10 && l.len() <= 14);
            assert!(l
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
        }
    }

    #[test]
    fn bo_the_html() {
        let h = "<p>Xin <b>chao</b></p><script>alert(1)</script><!-- ghichu --><style>p{color:red}</style>&nbsp;ban";
        let t = html_to_text(h);
        assert!(!t.contains('<'));
        assert!(!t.contains("alert"));
        assert!(!t.contains("ghichu"), "comment HTML chua bi loai bo");
        assert!(!t.contains("color"), "the style chua bi loai bo");
        assert!(t.contains("chao"));
    }

    #[test]
    fn tim_otp() {
        assert_eq!(
            extract_otp("Your verification code is 481920").as_deref(),
            Some("481920")
        );
        assert_eq!(extract_otp("Ma xac thuc: 4829").as_deref(), Some("4829"));
        // khong co tu khoa, khong phai 6 chu so -> bo qua
        assert_eq!(extract_otp("Don hang 12345678 da giao"), None);
        // khong co tu khoa nhung dung 6 chu so -> van nhan
        assert_eq!(extract_otp("So don 481920").as_deref(), Some("481920"));
        assert_eq!(extract_otp("Hoa don thang 9 nam 2026"), None);
        assert_eq!(extract_otp("Khong co so nao ca"), None);
        // so dang ngay thang khong duoc coi la ma
        assert_eq!(
            extract_otp("PIN 1234 va don 20260101").as_deref(),
            Some("1234")
        );
        assert_eq!(extract_otp("Hoa don 20260101"), None);
        // ma 4 so bat dau bang 19/20 van la ma hop le khi co tu khoa
        assert_eq!(extract_otp("Your PIN is 2048").as_deref(), Some("2048"));
        assert_eq!(extract_otp("Ma xac thuc: 1999").as_deref(), Some("1999"));
        // nhung neu co ung vien khac thi chuoi giong nam bi xep sau
        assert_eq!(
            extract_otp("Your code is 481920, sent in 2026").as_deref(),
            Some("481920")
        );
        // "shipping" khong duoc coi la tu khoa "pin"
        assert_eq!(extract_otp("Your shipping order 12345678 is out"), None);
    }
}
