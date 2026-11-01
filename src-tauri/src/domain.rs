use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const GOAL_MIN_ML: i32 = 1200;
pub const GOAL_MAX_ML: i32 = 4500;
pub const GOAL_STEP_ML: i32 = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortionSpec {
    pub id: &'static str,
    pub category: &'static str,
    pub icon: &'static str,
    pub label_short: &'static str,
    pub gross_ml: i32,
    pub effective_ml: i32,
}

pub fn lookup_portion(portion_id: &str) -> Option<PortionSpec> {
    match portion_id {
        "water_glass" => Some(PortionSpec {
            id: "water_glass",
            category: "water",
            icon: "🥛",
            label_short: "Su Bardağı",
            gross_ml: 200,
            effective_ml: 200,
        }),
        "water_sip" => Some(PortionSpec {
            id: "water_sip",
            category: "water",
            icon: "💧",
            label_short: "Yudum Su",
            gross_ml: 50,
            effective_ml: 50,
        }),
        "water_mug" => Some(PortionSpec {
            id: "water_mug",
            category: "water",
            icon: "🫗",
            label_short: "Kupa Su",
            gross_ml: 300,
            effective_ml: 300,
        }),
        "water_bottle" => Some(PortionSpec {
            id: "water_bottle",
            category: "water",
            icon: "🍼",
            label_short: "Şişe / Matara Su",
            gross_ml: 500,
            effective_ml: 500,
        }),
        "tea_ince" => Some(PortionSpec {
            id: "tea_ince",
            category: "tea",
            icon: "🫖",
            label_short: "İnce Belli Çay",
            gross_ml: 100,
            effective_ml: 90,
        }),
        "tea_ajda" => Some(PortionSpec {
            id: "tea_ajda",
            category: "tea",
            icon: "🍵",
            label_short: "Ajda Çay",
            gross_ml: 150,
            effective_ml: 135,
        }),
        "tea_mug" => Some(PortionSpec {
            id: "tea_mug",
            category: "tea",
            icon: "☕",
            label_short: "Kupa Çay",
            gross_ml: 200,
            effective_ml: 180,
        }),
        "coffee_turk" => Some(PortionSpec {
            id: "coffee_turk",
            category: "coffee",
            icon: "☕",
            label_short: "Fincan Türk Kahvesi",
            gross_ml: 70,
            effective_ml: 56,
        }),
        "coffee_mug" => Some(PortionSpec {
            id: "coffee_mug",
            category: "coffee",
            icon: "🥤",
            label_short: "Kupa Filtre Kahve",
            gross_ml: 200,
            effective_ml: 160,
        }),
        "ayran_small" => Some(PortionSpec {
            id: "ayran_small",
            category: "ayran",
            icon: "🥛",
            label_short: "Küçük Bardak Ayran",
            gross_ml: 200,
            effective_ml: 200,
        }),
        "ayran_large" => Some(PortionSpec {
            id: "ayran_large",
            category: "ayran",
            icon: "🫙",
            label_short: "Büyük Bardak Ayran",
            gross_ml: 300,
            effective_ml: 300,
        }),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettings {
    pub onboarding_completed: bool,
    pub weight_kg: i32,
    pub age_bracket: String,
    pub activity_level: String,
    pub daily_goal_ml: i32,
    pub active_start_min: i32,
    pub active_end_min: i32,
    pub interval_min: i32,
    pub next_alarm_epoch_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntakeLog {
    pub id: String,
    pub tracking_date: String,
    pub logged_at_ms: i64,
    pub category: String,
    pub portion_id: String,
    pub gross_ml: i32,
    pub effective_ml: i32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayProgress {
    pub tracking_date: String,
    pub goal_ml: i32,
    pub pure_water_ml: i32,
    pub raw_other_effective_ml: i32,
    pub counted_other_ml: i32,
    pub other_cap_ml: i32,
    pub min_pure_water_ml: i32,
    pub total_effective_ml: i32,
    pub pct: i32,
    pub remaining_ml: i32,
    pub remaining_glasses: i32,
    pub is_other_cap_reached: bool,
    pub cup_summary: String,
    pub logs: Vec<IntakeLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardState {
    pub settings: UserSettings,
    pub today_date: String,
    pub yesterday_date: String,
    pub active_day: DayProgress,
    pub streak_days: i32,
    pub weekly_freeze_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDayEntry {
    pub tracking_date: String,
    pub pct: i32,
    pub goal_ml: i32,
    pub total_effective_ml: i32,
    pub status: String,
    pub cup_summary: String,
}

pub fn clamp_goal_ml(ml: i32) -> i32 {
    let rounded = ((ml as f64 / GOAL_STEP_ML as f64).round() as i32) * GOAL_STEP_ML;
    rounded.clamp(GOAL_MIN_ML, GOAL_MAX_ML)
}

/// 04:00 uyku odaklı gün dönümü:
/// Yerel saat 00:00 - 03:59 arasındayken yapılan kayıtlar bir önceki takvim gününe yazılır.
pub fn current_tracking_date_from_epoch(epoch_ms: i64) -> String {
    let shifted_ms = epoch_ms - 4 * 3600 * 1000;
    let dt = Local
        .timestamp_millis_opt(shifted_ms)
        .single()
        .unwrap_or_else(Local::now);
    dt.format("%Y-%m-%d").to_string()
}

pub fn current_tracking_date() -> String {
    current_tracking_date_from_epoch(Local::now().timestamp_millis())
}

pub fn shift_tracking_date(tracking_date: &str, delta_days: i64) -> String {
    if let Ok(nd) = NaiveDate::parse_from_str(tracking_date, "%Y-%m-%d") {
        let next = nd + Duration::days(delta_days);
        return next.format("%Y-%m-%d").to_string();
    }
    tracking_date.to_string()
}

pub fn summarize_in_cup_language(logs: &[IntakeLog]) -> String {
    if logs.is_empty() {
        return "Henüz içecek kaydedilmedi".to_string();
    }
    let mut order: Vec<String> = Vec::new();
    let mut counts: BTreeMap<String, i32> = BTreeMap::new();
    for log in logs {
        if !counts.contains_key(&log.portion_id) {
            order.push(log.portion_id.clone());
        }
        *counts.entry(log.portion_id.clone()).or_insert(0) += 1;
    }

    order
        .into_iter()
        .map(|pid| {
            let c = counts.get(&pid).copied().unwrap_or(1);
            if let Some(p) = lookup_portion(&pid) {
                format!("{} {}× {}", p.icon, c, p.label_short)
            } else {
                format!("{}× İçecek", c)
            }
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

pub fn compute_day_progress(
    tracking_date: &str,
    logs: Vec<IntakeLog>,
    goal_ml: i32,
) -> DayProgress {
    let mut pure_water_ml = 0;
    let mut raw_other_effective_ml = 0;

    for log in &logs {
        if log.category == "water" {
            pure_water_ml += log.effective_ml;
        } else {
            raw_other_effective_ml += log.effective_ml;
        }
    }

    let other_cap_ml = ((goal_ml as f64) * 0.30).round() as i32;
    let min_pure_water_ml = goal_ml - other_cap_ml;
    let counted_other_ml = raw_other_effective_ml.min(other_cap_ml);
    let total_effective_ml = pure_water_ml + counted_other_ml;
    let pct = if goal_ml > 0 {
        (((total_effective_ml as f64 / goal_ml as f64) * 100.0).round() as i32).min(100)
    } else {
        0
    };
    let remaining_ml = (goal_ml - total_effective_ml).max(0);
    let remaining_glasses = ((remaining_ml as f64) / 200.0).ceil() as i32;
    let is_other_cap_reached = raw_other_effective_ml >= other_cap_ml;
    let cup_summary = summarize_in_cup_language(&logs);

    DayProgress {
        tracking_date: tracking_date.to_string(),
        goal_ml,
        pure_water_ml,
        raw_other_effective_ml,
        counted_other_ml,
        other_cap_ml,
        min_pure_water_ml,
        total_effective_ml,
        pct,
        remaining_ml,
        remaining_glasses,
        is_other_cap_reached,
        cup_summary,
        logs,
    }
}

fn iso_week_key(tracking_date: &str) -> String {
    if let Ok(nd) = NaiveDate::parse_from_str(tracking_date, "%Y-%m-%d") {
        let iw = nd.iso_week();
        return format!("{}-W{:02}", iw.year(), iw.week());
    }
    "0000-W00".to_string()
}

/// İki Katmanlı Esnek Günlük Seri (Streak) Hesaplayıcı:
/// - %100: Seri +1 artar
/// - %80-99 (Koruma Eşiği): Seri korunur, mola hakkı harcanmaz
/// - <%80 (Haftalık Mola Hakkı): Haftada 1 kez seri korunur; aynı hafta 2. kez <%80 olursa seri 0'a sıfırlanır.
pub fn evaluate_streak(
    today_date: &str,
    day_pct_by_date: &BTreeMap<String, i32>,
) -> (i32, bool) {
    let current_week = iso_week_key(today_date);
    let Some((earliest_date, _)) = day_pct_by_date.iter().next() else {
        return (0, true);
    };

    let mut streak = 0;
    let mut used_freeze_in_week: BTreeMap<String, bool> = BTreeMap::new();
    let mut cursor = earliest_date.clone();

    while cursor.as_str() < today_date {
        let pct = day_pct_by_date.get(&cursor).copied().unwrap_or(0);
        let wk = iso_week_key(&cursor);
        if pct >= 100 {
            streak += 1;
        } else if pct >= 80 {
            // Koruma Eşiği: seri sabit kalır, haftalık mola hakkı harcanmaz
        } else if !used_freeze_in_week.get(&wk).copied().unwrap_or(false) && streak > 0 {
            used_freeze_in_week.insert(wk, true);
        } else {
            streak = 0;
        }
        cursor = shift_tracking_date(&cursor, 1);
    }

    if day_pct_by_date.get(today_date).copied().unwrap_or(0) >= 100 {
        streak += 1;
    }

    let weekly_freeze_available = !used_freeze_in_week
        .get(&current_week)
        .copied()
        .unwrap_or(false);
    (streak, weekly_freeze_available)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other_beverage_30_percent_hard_cap() {
        let p = lookup_portion("tea_ajda").unwrap();
        let logs: Vec<IntakeLog> = (0..5)
            .map(|i| IntakeLog {
                id: format!("id-{i}"),
                tracking_date: "2026-11-01".to_string(),
                logged_at_ms: 1000,
                category: p.category.to_string(),
                portion_id: p.id.to_string(),
                gross_ml: p.gross_ml,
                effective_ml: p.effective_ml,
                source: "app".to_string(),
            })
            .collect();

        let prog = compute_day_progress("2026-11-01", logs, 2000);
        assert_eq!(prog.raw_other_effective_ml, 675);
        assert_eq!(prog.counted_other_ml, 600);
        assert_eq!(prog.total_effective_ml, 600);
        assert_eq!(prog.pct, 30);
        assert!(prog.is_other_cap_reached);
        assert_eq!(prog.remaining_glasses, 7);
    }

    #[test]
    fn test_two_tier_streak_protection_and_weekly_freeze() {
        let mut map = BTreeMap::new();
        map.insert("2026-10-26".to_string(), 100);
        map.insert("2026-10-27".to_string(), 100);
        map.insert("2026-10-28".to_string(), 85); // %85 koruma eşiği
        map.insert("2026-10-29".to_string(), 40); // <%80 haftalık mola hakkı
        map.insert("2026-10-30".to_string(), 100);

        let (streak, freeze_avail) = evaluate_streak("2026-10-30", &map);
        assert_eq!(streak, 3);
        assert!(!freeze_avail);
    }
}
