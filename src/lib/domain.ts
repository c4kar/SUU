export type BeverageCategory = 'water' | 'tea' | 'coffee' | 'ayran';
export type AgeBracket = 'under_30' | '30_55' | 'over_55';
export type ActivityLevel = 'low' | 'medium' | 'high';

export interface PortionSpec {
  id: string;
  category: BeverageCategory;
  icon: string;
  title: string;
  grossMl: number;
  factor: number;
  effectiveMl: number;
  isDefault?: boolean;
}

export const PORTIONS: Record<string, PortionSpec> = {
  water_glass: {
    id: 'water_glass',
    category: 'water',
    icon: '🥛',
    title: '1 Su Bardağı',
    grossMl: 200,
    factor: 1.0,
    effectiveMl: 200,
    isDefault: true,
  },
  water_sip: {
    id: 'water_sip',
    category: 'water',
    icon: '💧',
    title: '1 Yudum Su',
    grossMl: 50,
    factor: 1.0,
    effectiveMl: 50,
  },
  water_mug: {
    id: 'water_mug',
    category: 'water',
    icon: '🫗',
    title: '1 Kupa Su',
    grossMl: 300,
    factor: 1.0,
    effectiveMl: 300,
  },
  water_bottle: {
    id: 'water_bottle',
    category: 'water',
    icon: '🍼',
    title: '1 Şişe / Matara Su',
    grossMl: 500,
    factor: 1.0,
    effectiveMl: 500,
  },
  tea_ince: {
    id: 'tea_ince',
    category: 'tea',
    icon: '🫖',
    title: '1 İnce Belli Çay',
    grossMl: 100,
    factor: 0.9,
    effectiveMl: 90,
    isDefault: true,
  },
  tea_ajda: {
    id: 'tea_ajda',
    category: 'tea',
    icon: '🍵',
    title: '1 Ajda Çay',
    grossMl: 150,
    factor: 0.9,
    effectiveMl: 135,
  },
  tea_mug: {
    id: 'tea_mug',
    category: 'tea',
    icon: '☕',
    title: '1 Kupa Çay',
    grossMl: 200,
    factor: 0.9,
    effectiveMl: 180,
  },
  coffee_turk: {
    id: 'coffee_turk',
    category: 'coffee',
    icon: '☕',
    title: '1 Fincan Türk Kahvesi',
    grossMl: 70,
    factor: 0.8,
    effectiveMl: 56,
    isDefault: true,
  },
  coffee_mug: {
    id: 'coffee_mug',
    category: 'coffee',
    icon: '🥤',
    title: '1 Kupa Filtre Kahve',
    grossMl: 200,
    factor: 0.8,
    effectiveMl: 160,
  },
  ayran_small: {
    id: 'ayran_small',
    category: 'ayran',
    icon: '🥛',
    title: '1 Küçük Bardak Ayran',
    grossMl: 200,
    factor: 1.0,
    effectiveMl: 200,
    isDefault: true,
  },
  ayran_large: {
    id: 'ayran_large',
    category: 'ayran',
    icon: '🫙',
    title: '1 Büyük Bardak Ayran',
    grossMl: 300,
    factor: 1.0,
    effectiveMl: 300,
  },
};

export interface UserSettings {
  onboardingCompleted: boolean;
  weightKg: number;
  ageBracket: AgeBracket;
  activityLevel: ActivityLevel;
  dailyGoalMl: number;
  activeStartMin: number; // 540 = 09:00
  activeEndMin: number;   // 1320 = 22:00
  intervalMin: number;    // 60 | 90 | 120 | 180
  nextAlarmEpochMs: number | null;
}

export interface IntakeLog {
  id: string;
  trackingDate: string; // YYYY-MM-DD (04:00 boundary)
  loggedAtMs: number;
  category: BeverageCategory;
  portionId: string;
  grossMl: number;
  effectiveMl: number;
  source: 'app' | 'notification';
}

export interface DayProgress {
  trackingDate: string;
  goalMl: number;
  pureWaterMl: number;
  rawOtherEffectiveMl: number;
  countedOtherMl: number;
  otherCapMl: number;
  minPureWaterMl: number;
  totalEffectiveMl: number;
  pct: number;
  remainingMl: number;
  remainingGlasses: number;
  isOtherCapReached: boolean;
  cupSummary: string;
  logs: IntakeLog[];
}

export interface DashboardState {
  settings: UserSettings;
  todayDate: string;
  yesterdayDate: string;
  activeDay: DayProgress;
  streakDays: number;
  weeklyFreezeAvailable: boolean;
}

export interface CalendarDayEntry {
  trackingDate: string;
  pct: number;
  goalMl: number;
  totalEffectiveMl: number;
  status: 'completed' | 'protected' | 'freeze_used' | 'missed' | 'empty';
  cupSummary: string;
}

export const GOAL_MIN_ML = 1200;
export const GOAL_MAX_ML = 4500;
export const GOAL_STEP_ML = 50;

export function clampGoalMl(ml: number): number {
  const rounded = Math.round(ml / GOAL_STEP_ML) * GOAL_STEP_ML;
  return Math.max(GOAL_MIN_ML, Math.min(GOAL_MAX_ML, rounded));
}

export function calculateScientificGoalMl(
  weightKg: number,
  ageBracket: AgeBracket,
  activityLevel: ActivityLevel,
): number {
  const ageFactor =
    ageBracket === 'under_30' ? 35 : ageBracket === '30_55' ? 32 : 28;
  const activityDelta =
    activityLevel === 'low' ? 0 : activityLevel === 'medium' ? 300 : 500;
  return clampGoalMl(weightKg * ageFactor + activityDelta);
}

/**
 * 04:00 uyku odaklı gün dönümü:
 * Yerel saat 00:00 - 03:59 arasındayken yapılan kayıtlar bir önceki takvim gününe aittir.
 */
export function getTrackingDate(now: Date = new Date()): string {
  const shifted = new Date(now.getTime() - 4 * 60 * 60 * 1000);
  const y = shifted.getFullYear();
  const m = String(shifted.getMonth() + 1).padStart(2, '0');
  const d = String(shifted.getDate()).padStart(2, '0');
  return `${y}-${m}-${d}`;
}

export function shiftTrackingDate(trackingDate: string, deltaDays: number): string {
  const [y, m, d] = trackingDate.split('-').map(Number);
  const dt = new Date(Date.UTC(y, m - 1, d + deltaDays, 12, 0, 0));
  const ry = dt.getUTCFullYear();
  const rm = String(dt.getUTCMonth() + 1).padStart(2, '0');
  const rd = String(dt.getUTCDate()).padStart(2, '0');
  return `${ry}-${rm}-${rd}`;
}

export function summarizeInCupLanguage(logs: IntakeLog[]): string {
  if (!logs.length) return 'Henüz içecek kaydedilmedi';
  const counts: Record<string, number> = {};
  for (const log of logs) {
    counts[log.portionId] = (counts[log.portionId] || 0) + 1;
  }
  return Object.entries(counts)
    .map(([portionId, count]) => {
      const p = PORTIONS[portionId];
      if (!p) return `${count}× İçecek`;
      const cleanName = p.title.replace(/^1\s+/, '');
      return `${p.icon} ${count}× ${cleanName}`;
    })
    .join(' · ');
}

export function computeDayProgress(
  trackingDate: string,
  logs: IntakeLog[],
  goalMl: number,
): DayProgress {
  let pureWaterMl = 0;
  let rawOtherEffectiveMl = 0;

  for (const log of logs) {
    if (log.category === 'water') {
      pureWaterMl += log.effectiveMl;
    } else {
      rawOtherEffectiveMl += log.effectiveMl;
    }
  }

  const otherCapMl = Math.round(goalMl * 0.3);
  const minPureWaterMl = goalMl - otherCapMl;
  const countedOtherMl = Math.min(rawOtherEffectiveMl, otherCapMl);
  const totalEffectiveMl = pureWaterMl + countedOtherMl;
  const pct = goalMl > 0 ? Math.min(100, Math.round((totalEffectiveMl / goalMl) * 100)) : 0;
  const remainingMl = Math.max(0, goalMl - totalEffectiveMl);
  const remainingGlasses = Math.ceil(remainingMl / 200);
  const isOtherCapReached = rawOtherEffectiveMl >= otherCapMl;

  return {
    trackingDate,
    goalMl,
    pureWaterMl,
    rawOtherEffectiveMl,
    countedOtherMl,
    otherCapMl,
    minPureWaterMl,
    totalEffectiveMl,
    pct,
    remainingMl,
    remainingGlasses,
    isOtherCapReached,
    cupSummary: summarizeInCupLanguage(logs),
    logs,
  };
}

function getIsoWeekKey(trackingDate: string): string {
  const [y, m, d] = trackingDate.split('-').map(Number);
  const dt = new Date(Date.UTC(y, m - 1, d));
  const dayNum = dt.getUTCDay() || 7;
  dt.setUTCDate(dt.getUTCDate() + 4 - dayNum);
  const yearStart = new Date(Date.UTC(dt.getUTCFullYear(), 0, 1));
  const weekNo = Math.ceil((((dt.getTime() - yearStart.getTime()) / 86400000) + 1) / 7);
  return `${dt.getUTCFullYear()}-W${String(weekNo).padStart(2, '0')}`;
}

/**
 * İki Katmanlı Esnek Günlük Seri (Streak) Hesaplayıcı:
 * - %100: Seri +1 artar
 * - %80-99 (Koruma Eşiği): Seri sabit kalır (mola hakkı harcanmaz)
 * - <%80 (Haftalık Mola Hakkı): Haftada 1 gün seri korunur; aynı hafta 2. kez <%80 olursa seri 0'a sıfırlanır.
 */
export function evaluateStreak(
  todayDate: string,
  dayPercentByDate: Record<string, number>,
  earliestDate?: string,
): { streakDays: number; weeklyFreezeAvailable: boolean } {
  const currentWeek = getIsoWeekKey(todayDate);
  if (!earliestDate) {
    const todayPct = dayPercentByDate[todayDate] ?? 0;
    return {
      streakDays: todayPct >= 100 ? 1 : 0,
      weeklyFreezeAvailable: true,
    };
  }

  let streak = 0;
  const usedFreezeInWeek: Record<string, boolean> = {};
  let cursor = earliestDate;

  while (cursor < todayDate) {
    const pct = dayPercentByDate[cursor] ?? 0;
    const wk = getIsoWeekKey(cursor);
    if (pct >= 100) {
      streak += 1;
    } else if (pct >= 80) {
      // Koruma Eşiği: seri sıfırlanmaz, mola hakkı harcanmaz
    } else {
      if (!usedFreezeInWeek[wk] && streak > 0) {
        usedFreezeInWeek[wk] = true;
      } else {
        streak = 0;
      }
    }
    cursor = shiftTrackingDate(cursor, 1);
  }

  // Bugün %100 olduysa seriye +1 ekle (bugün henüz bitmediği için <%100 ise seriyi kırmaz)
  if ((dayPercentByDate[todayDate] ?? 0) >= 100) {
    streak += 1;
  }

  return {
    streakDays: streak,
    weeklyFreezeAvailable: !usedFreezeInWeek[currentWeek],
  };
}
