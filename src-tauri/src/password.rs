use rand::rngs::OsRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::wordlist::WORDS;

const LOWER: &str = "abcdefghijkmnopqrstuvwxyz";
const LOWER_AMB: &str = "l";
const UPPER: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const UPPER_AMB: &str = "IO";
const DIGITS: &str = "23456789";
const DIGITS_AMB: &str = "01";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.?/";
const SYMBOLS_AMB: &str = "|`'\"~<>\\";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOptions {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    /// Bo cac ky tu de nham lan: 0 O o, 1 l I, | ` ' "
    pub avoid_ambiguous: bool,
    /// Khong cho ky tu lap lai (chi ap dung khi do dai <= so ky tu kha dung)
    pub no_repeat: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassphraseOptions {
    pub words: usize,
    pub separator: String,
    pub capitalize: bool,
    /// Chen mot so ngau nhien vao cuoi mot tu bat ky
    pub add_number: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedPassword {
    pub value: String,
    pub entropy_bits: f64,
    pub pool_size: usize,
    pub score: u8,
    pub label: String,
    pub crack_time_online: String,
    pub crack_time_offline: String,
}

fn class_chars(base: &str, ambiguous: &str, avoid: bool) -> Vec<char> {
    let mut v: Vec<char> = base.chars().collect();
    if !avoid {
        v.extend(ambiguous.chars());
    }
    v
}

/// Sinh so ngau nhien khong lech (rejection sampling) trong khoang [0, n)
fn uniform_index(rng: &mut OsRng, n: usize) -> usize {
    rng.gen_range(0..n)
}

/// Lay mau dong deu tu pool, co lap lai.
fn sample_with_repeat(rng: &mut OsRng, pool: &[char], len: usize) -> Vec<char> {
    (0..len)
        .map(|_| pool[uniform_index(rng, pool.len())])
        .collect()
}

/// Lay mau dong deu tu pool, khong lap lai (Fisher-Yates mot phan).
fn sample_distinct(rng: &mut OsRng, pool: &[char], len: usize) -> Vec<char> {
    let mut buf = pool.to_vec();
    for i in 0..len {
        let j = i + uniform_index(rng, buf.len() - i);
        buf.swap(i, j);
    }
    buf.truncate(len);
    buf
}

fn covers_all(chars: &[char], classes: &[Vec<char>]) -> bool {
    classes
        .iter()
        .all(|class| chars.iter().any(|c| class.contains(c)))
}

/// So luong mat khau hop le, tinh bang log2.
///
/// Rang buoc "moi nhom xuat hien it nhat mot lan" loai bot mot phan khong gian,
/// nen entropy thuc te nho hon log2(pool^len). Dung bao ham - loai tru, chuan hoa
/// theo tong so day khong rang buoc de moi so hang deu nam trong [0, 1] va khong tran so.
fn log2_valid_count(sizes: &[usize], len: usize, distinct: bool) -> f64 {
    let pool: usize = sizes.iter().sum();
    let k = sizes.len();
    let mut fraction = 0f64;
    for mask in 0u32..(1u32 << k) {
        let excluded: usize = (0..k)
            .filter(|i| (mask >> i) & 1 == 1)
            .map(|i| sizes[i])
            .sum();
        let remaining = pool - excluded;
        let sign = if mask.count_ones() % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        let ratio = if distinct {
            if remaining < len {
                0.0
            } else {
                (0..len)
                    .map(|i| (remaining - i) as f64 / (pool - i) as f64)
                    .product::<f64>()
            }
        } else {
            (remaining as f64 / pool as f64).powi(len as i32)
        };
        fraction += sign * ratio;
    }
    let base: f64 = if distinct {
        (0..len).map(|i| ((pool - i) as f64).log2()).sum()
    } else {
        (pool as f64).log2() * len as f64
    };
    base + fraction.max(f64::MIN_POSITIVE).log2()
}

/// So lan thu toi da truoc khi bo cuoc. Ty le chap nhan thap nhat (do dai 4, du 4 nhom)
/// van khoang 6%, nen xac suat that bai sau ngan nay lan la khong dang ke.
const MAX_TRIES: usize = 10_000;

pub fn generate_password(opts: &PasswordOptions) -> Result<GeneratedPassword, String> {
    if opts.length < 4 || opts.length > 128 {
        return Err("Độ dài mật khẩu phải từ 4 đến 128 ký tự".into());
    }
    let mut classes: Vec<Vec<char>> = Vec::new();
    if opts.lowercase {
        classes.push(class_chars(LOWER, LOWER_AMB, opts.avoid_ambiguous));
    }
    if opts.uppercase {
        classes.push(class_chars(UPPER, UPPER_AMB, opts.avoid_ambiguous));
    }
    if opts.digits {
        classes.push(class_chars(DIGITS, DIGITS_AMB, opts.avoid_ambiguous));
    }
    if opts.symbols {
        classes.push(class_chars(SYMBOLS, SYMBOLS_AMB, opts.avoid_ambiguous));
    }
    if classes.is_empty() {
        return Err("Cần chọn ít nhất một nhóm ký tự".into());
    }
    if classes.len() > opts.length {
        return Err("Độ dài quá ngắn so với số nhóm ký tự đã chọn".into());
    }

    let pool: Vec<char> = classes.iter().flatten().copied().collect();
    let sizes: Vec<usize> = classes.iter().map(|c| c.len()).collect();

    if opts.no_repeat && opts.length > pool.len() {
        return Err(format!(
            "Không lặp ký tự thì độ dài tối đa là {} với các nhóm đang chọn",
            pool.len()
        ));
    }

    // Lay mau tu choi: sinh dong deu roi loai bo ket qua thieu nhom.
    // Cach nay cho phan bo dong deu TUYET DOI tren tap mat khau hop le —
    // khac voi cach "cai san moi nhom mot ky tu roi xao tron", vi cach do
    // uu ai nhung mat khau co so ky tu cac nhom can bang nhau.
    let mut rng = OsRng;
    let mut chars: Option<Vec<char>> = None;
    for _ in 0..MAX_TRIES {
        let candidate = if opts.no_repeat {
            sample_distinct(&mut rng, &pool, opts.length)
        } else {
            sample_with_repeat(&mut rng, &pool, opts.length)
        };
        if covers_all(&candidate, &classes) {
            chars = Some(candidate);
            break;
        }
    }
    let chars = chars.ok_or_else(|| {
        "Không sinh được mật khẩu thoả mãn điều kiện — hãy tăng độ dài hoặc bớt ràng buộc"
            .to_string()
    })?;

    let value: String = chars.into_iter().collect();
    let entropy = log2_valid_count(&sizes, opts.length, opts.no_repeat);
    Ok(build_result(value, entropy, pool.len()))
}

pub fn generate_passphrase(opts: &PassphraseOptions) -> Result<GeneratedPassword, String> {
    if opts.words < 4 || opts.words > 15 {
        return Err("Số từ phải từ 4 đến 15".into());
    }
    let mut rng = OsRng;
    let mut parts: Vec<String> = Vec::with_capacity(opts.words);
    for _ in 0..opts.words {
        let w = WORDS[uniform_index(&mut rng, WORDS.len())];
        if opts.capitalize {
            let mut c = w.chars();
            let first = c.next().unwrap().to_ascii_uppercase();
            parts.push(format!("{}{}", first, c.as_str()));
        } else {
            parts.push(w.to_string());
        }
    }
    let mut entropy = (WORDS.len() as f64).log2() * opts.words as f64;
    if opts.add_number {
        let idx = uniform_index(&mut rng, parts.len());
        let n: u32 = rng.gen_range(10..100);
        parts[idx] = format!("{}{}", parts[idx], n);
        entropy += (parts.len() as f64).log2() + (90f64).log2();
    }
    // Chuoi rong la mot lua chon hop le ("lien") tren giao dien, khong phai gia tri thieu.
    let value = parts.join(opts.separator.as_str());
    Ok(build_result(value, entropy, WORDS.len()))
}

fn build_result(value: String, entropy_bits: f64, pool_size: usize) -> GeneratedPassword {
    let (score, label) = classify(entropy_bits);
    GeneratedPassword {
        value,
        entropy_bits: (entropy_bits * 10.0).round() / 10.0,
        pool_size,
        score,
        label: label.to_string(),
        // Online co gioi han toc do: 100 lan doan/giay
        crack_time_online: crack_time(entropy_bits, 100.0),
        // Offline GPU nhanh: 100 ti lan doan/giay
        crack_time_offline: crack_time(entropy_bits, 1e11),
    }
}

fn classify(bits: f64) -> (u8, &'static str) {
    match bits {
        b if b < 40.0 => (0, "Rất yếu"),
        b if b < 60.0 => (1, "Yếu"),
        b if b < 80.0 => (2, "Khá"),
        b if b < 110.0 => (3, "Mạnh"),
        _ => (4, "Rất mạnh"),
    }
}

/// Thoi gian trung binh de do het (mot nua khong gian khoa)
fn crack_time(bits: f64, guesses_per_sec: f64) -> String {
    // seconds = 2^(bits-1) / rate; tinh trong khong gian log de tranh tran so
    let log10_seconds =
        (bits - 1.0) * std::f64::consts::LN_2 / std::f64::consts::LN_10 - guesses_per_sec.log10();
    if log10_seconds < 0.0 {
        return "tức thì".into();
    }
    let seconds = if log10_seconds > 300.0 {
        f64::INFINITY
    } else {
        10f64.powf(log10_seconds)
    };
    const MINUTE: f64 = 60.0;
    const HOUR: f64 = 3600.0;
    const DAY: f64 = 86_400.0;
    const MONTH: f64 = 2_629_800.0;
    const YEAR: f64 = 31_557_600.0;

    if seconds < MINUTE {
        format!("{:.0} giây", seconds.max(1.0))
    } else if seconds < HOUR {
        format!("{:.0} phút", seconds / MINUTE)
    } else if seconds < DAY {
        format!("{:.0} giờ", seconds / HOUR)
    } else if seconds < MONTH {
        format!("{:.0} ngày", seconds / DAY)
    } else if seconds < YEAR {
        format!("{:.0} tháng", seconds / MONTH)
    } else {
        let years = seconds / YEAR;
        if years < 1000.0 {
            format!("{:.0} năm", years)
        } else if years < 1e6 {
            format!("{:.0} nghìn năm", years / 1e3)
        } else if years < 1e9 {
            format!("{:.0} triệu năm", years / 1e6)
        } else if years < 1e12 {
            format!("{:.0} tỉ năm", years / 1e9)
        } else if years.is_finite() {
            format!("10^{:.0} năm", years.log10())
        } else {
            "vô hạn".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(len: usize) -> PasswordOptions {
        PasswordOptions {
            length: len,
            lowercase: true,
            uppercase: true,
            digits: true,
            symbols: true,
            avoid_ambiguous: false,
            no_repeat: false,
        }
    }

    #[test]
    fn do_dai_dung() {
        for len in [4usize, 12, 32, 128] {
            let p = generate_password(&opts(len)).unwrap();
            assert_eq!(p.value.chars().count(), len);
        }
    }

    #[test]
    fn du_moi_nhom_ky_tu() {
        for _ in 0..200 {
            let p = generate_password(&opts(8)).unwrap();
            assert!(p.value.chars().any(|c| c.is_ascii_lowercase()));
            assert!(p.value.chars().any(|c| c.is_ascii_uppercase()));
            assert!(p.value.chars().any(|c| c.is_ascii_digit()));
            assert!(p
                .value
                .chars()
                .any(|c| SYMBOLS.contains(c) || SYMBOLS_AMB.contains(c)));
        }
    }

    #[test]
    fn tranh_ky_tu_nham_lan() {
        let mut o = opts(60);
        o.avoid_ambiguous = true;
        let p = generate_password(&o).unwrap();
        for c in "0O1lI|`'\"~<>\\".chars() {
            assert!(!p.value.contains(c), "van con ky tu {}", c);
        }
    }

    #[test]
    fn khong_lap_ky_tu() {
        let mut o = opts(20);
        o.no_repeat = true;
        let p = generate_password(&o).unwrap();
        let mut seen: Vec<char> = p.value.chars().collect();
        seen.sort_unstable();
        let before = seen.len();
        seen.dedup();
        assert_eq!(before, seen.len());
    }

    #[test]
    fn entropy_dung_khi_chi_mot_nhom() {
        // Mot nhom thi rang buoc "du moi nhom" la hien nhien:
        // entropy phai bang dung log2(pool) * do dai
        let mut o = opts(16);
        o.uppercase = false;
        o.digits = false;
        o.symbols = false;
        let p = generate_password(&o).unwrap();
        let expected = (26f64).log2() * 16.0;
        assert!(
            (p.entropy_bits - expected).abs() < 0.05,
            "{} != {}",
            p.entropy_bits,
            expected
        );
    }

    #[test]
    fn entropy_thap_hon_khi_bi_rang_buoc() {
        // Voi du 4 nhom, khong gian bi thu hep nen entropy phai NHO HON
        // log2(pool^len), va khoang lech giam dan khi mat khau dai ra.
        let naive = |len: usize| (94f64).log2() * len as f64;
        let gap8 = naive(8) - generate_password(&opts(8)).unwrap().entropy_bits;
        let gap32 = naive(32) - generate_password(&opts(32)).unwrap().entropy_bits;
        assert!(gap8 > 0.5 && gap8 < 3.0, "lech o do dai 8: {}", gap8);
        assert!(gap32 > 0.0 && gap32 < gap8, "lech o do dai 32: {}", gap32);
    }

    #[test]
    fn bao_loi_khi_khong_lap_ma_qua_dai() {
        let mut o = opts(128);
        o.no_repeat = true;
        let e = generate_password(&o).unwrap_err();
        assert!(e.contains("94"), "thong bao loi khong neu gioi han: {}", e);
    }

    #[test]
    fn moi_ky_tu_trong_pool_deu_co_the_xuat_hien() {
        // Kiem tra khong ky tu nao bi thuat toan bo quen
        let mut thay = std::collections::HashSet::new();
        for _ in 0..400 {
            for c in generate_password(&opts(40)).unwrap().value.chars() {
                thay.insert(c);
            }
        }
        assert_eq!(thay.len(), 94, "chi thay {} / 94 ky tu", thay.len());
    }

    #[test]
    fn tu_choi_khi_khong_chon_nhom_nao() {
        let mut o = opts(12);
        o.lowercase = false;
        o.uppercase = false;
        o.digits = false;
        o.symbols = false;
        assert!(generate_password(&o).is_err());
    }

    #[test]
    fn passphrase_dung_so_tu() {
        let p = generate_passphrase(&PassphraseOptions {
            words: 7,
            separator: "-".into(),
            capitalize: true,
            add_number: false,
        })
        .unwrap();
        assert_eq!(p.value.split('-').count(), 7);
        // 7 tu tu danh sach 410 am tiet phai vuot 60 bit
        assert!(
            p.entropy_bits > 60.0,
            "entropy qua thap: {}",
            p.entropy_bits
        );
        assert!(generate_passphrase(&PassphraseOptions {
            words: 3,
            separator: "-".into(),
            capitalize: false,
            add_number: false,
        })
        .is_err());
    }

    #[test]
    fn passphrase_dau_noi_rong_thi_viet_lien() {
        // Giao dien co lua chon "lien": chuoi rong phai duoc ton trong,
        // khong duoc am tham thay bang dau gach ngang.
        let p = generate_passphrase(&PassphraseOptions {
            words: 5,
            separator: String::new(),
            capitalize: false,
            add_number: false,
        })
        .unwrap();
        assert!(
            !p.value.contains('-'),
            "van chen dau gach ngang: {}",
            p.value
        );
        assert!(p.value.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn wordlist_khong_trung_lap() {
        let mut v: Vec<&str> = WORDS.to_vec();
        let n = v.len();
        v.sort_unstable();
        v.dedup();
        assert_eq!(n, v.len());
        assert!(n >= 256);
    }

    #[test]
    fn crack_time_khong_tran_so() {
        let s = crack_time(1024.0, 1e11);
        assert!(!s.is_empty());
        assert_eq!(crack_time(1.0, 1e11), "tức thì");
    }
}
