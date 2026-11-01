import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import {
  PORTIONS,
  type ActivityLevel,
  type AgeBracket,
  type CalendarDayEntry,
  type DashboardState,
  type IntakeLog,
  type UserSettings,
  clampGoalMl,
  computeDayProgress,
  evaluateStreak,
  getTrackingDate,
  shiftTrackingDate,
} from './domain';

function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

// ============================================================================
// Browser Preview Fallback Store (when running `vite` outside Tauri Android/Desktop)
// ============================================================================
const STORAGE_KEY = 'suu_local_fallback_v1';

interface FallbackStore {
  settings: UserSettings;
  logs: IntakeLog[];
  dailyGoals: Record<string, number>;
}

function loadFallbackStore(): FallbackStore {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw);
  } catch {
    // ignore
  }
  return {
    settings: {
      onboardingCompleted: false,
      weightKg: 65,
      ageBracket: '30_55',
      activityLevel: 'low',
      dailyGoalMl: 2000,
      activeStartMin: 540, // 09:00
      activeEndMin: 1320,  // 22:00
      intervalMin: 120,    // 2 saat
      nextAlarmEpochMs: null,
    },
    logs: [],
    dailyGoals: {},
  };
}

function saveFallbackStore(store: FallbackStore): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(store));
  } catch {
    // ignore
  }
}

function buildFallbackDashboard(selectedDate?: string): DashboardState {
  const store = loadFallbackStore();
  const todayDate = getTrackingDate();
  const yesterdayDate = shiftTrackingDate(todayDate, -1);
  const targetDate = selectedDate || todayDate;

  const targetGoal = store.dailyGoals[targetDate] ?? store.settings.dailyGoalMl;
  const dayLogs = store.logs
    .filter((l) => l.trackingDate === targetDate)
    .sort((a, b) => b.loggedAtMs - a.loggedAtMs);

  const activeDay = computeDayProgress(targetDate, dayLogs, targetGoal);

  // Build streak across all recorded dates
  const allDates = Array.from(
    new Set([...Object.keys(store.dailyGoals), ...store.logs.map((l) => l.trackingDate)]),
  ).sort();
  const pctMap: Record<string, number> = {};
  for (const dt of allDates) {
    const g = store.dailyGoals[dt] ?? store.settings.dailyGoalMl;
    const dl = store.logs.filter((l) => l.trackingDate === dt);
    pctMap[dt] = computeDayProgress(dt, dl, g).pct;
  }
  const streakInfo = evaluateStreak(todayDate, pctMap, allDates[0]);

  return {
    settings: store.settings,
    todayDate,
    yesterdayDate,
    activeDay,
    streakDays: streakInfo.streakDays,
    weeklyFreezeAvailable: streakInfo.weeklyFreezeAvailable,
  };
}

// ============================================================================
// Unified API (Tauri v2 Rust + Kotlin Plugin Bridge)
// ============================================================================

export async function getDashboardState(trackingDate?: string): Promise<DashboardState> {
  if (isTauriRuntime()) {
    return await invoke<DashboardState>('get_dashboard_state', {
      trackingDate: trackingDate ?? null,
    });
  }
  return buildFallbackDashboard(trackingDate);
}

export async function logIntake(
  portionId: string,
  trackingDate?: string,
): Promise<DashboardState> {
  if (isTauriRuntime()) {
    const state = await invoke<DashboardState>('log_intake', {
      portionId,
      trackingDate: trackingDate ?? null,
    });
    await rescheduleAndroidNotifications().catch(() => {});
    return state;
  }

  const p = PORTIONS[portionId];
  if (!p) throw new Error(`Bilinmeyen porsiyon: ${portionId}`);
  const store = loadFallbackStore();
  const todayDate = getTrackingDate();
  const targetDate = trackingDate || todayDate;
  store.dailyGoals[targetDate] = store.dailyGoals[targetDate] ?? store.settings.dailyGoalMl;
  store.logs.unshift({
    id: crypto.randomUUID(),
    trackingDate: targetDate,
    loggedAtMs: Date.now(),
    category: p.category,
    portionId: p.id,
    grossMl: p.grossMl,
    effectiveMl: p.effectiveMl,
    source: 'app',
  });
  store.settings.nextAlarmEpochMs = Date.now() + store.settings.intervalMin * 60_000;
  saveFallbackStore(store);
  return buildFallbackDashboard(targetDate);
}

export async function deleteIntake(
  logId: string,
  trackingDate?: string,
): Promise<DashboardState> {
  if (isTauriRuntime()) {
    const state = await invoke<DashboardState>('delete_intake', {
      logId,
      trackingDate: trackingDate ?? null,
    });
    await rescheduleAndroidNotifications().catch(() => {});
    return state;
  }

  const store = loadFallbackStore();
  store.logs = store.logs.filter((l) => l.id !== logId);
  saveFallbackStore(store);
  return buildFallbackDashboard(trackingDate);
}

export interface UpdateSettingsPayload {
  onboardingCompleted: boolean;
  weightKg: number;
  ageBracket: AgeBracket;
  activityLevel: ActivityLevel;
  dailyGoalMl: number;
  activeStartMin: number;
  activeEndMin: number;
  intervalMin: number;
}

export async function updateSettings(
  payload: UpdateSettingsPayload,
  requestPermissions = false,
): Promise<DashboardState> {
  if (isTauriRuntime()) {
    const state = await invoke<DashboardState>('update_settings', { payload });
    if (requestPermissions) {
      await invoke('plugin:suu-notifications|request_one_ui_permissions').catch(() => {});
    }
    await rescheduleAndroidNotifications().catch(() => {});
    return state;
  }

  const store = loadFallbackStore();
  const todayDate = getTrackingDate();
  store.settings = {
    ...store.settings,
    ...payload,
    dailyGoalMl: clampGoalMl(payload.dailyGoalMl),
  };
  store.dailyGoals[todayDate] = store.settings.dailyGoalMl;
  saveFallbackStore(store);
  return buildFallbackDashboard(todayDate);
}

export async function getCalendarHistory(yearMonth: string): Promise<CalendarDayEntry[]> {
  if (isTauriRuntime()) {
    return await invoke<CalendarDayEntry[]>('get_calendar_history', { yearMonth });
  }

  const store = loadFallbackStore();
  const [y, m] = yearMonth.split('-').map(Number);
  const daysInMonth = new Date(y, m, 0).getDate();
  const entries: CalendarDayEntry[] = [];

  for (let d = 1; d <= daysInMonth; d++) {
    const dt = `${y}-${String(m).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
    const dayLogs = store.logs.filter((l) => l.trackingDate === dt);
    const goal = store.dailyGoals[dt] ?? store.settings.dailyGoalMl;
    const prog = computeDayProgress(dt, dayLogs, goal);
    let status: CalendarDayEntry['status'] = 'empty';
    if (dayLogs.length > 0) {
      if (prog.pct >= 100) status = 'completed';
      else if (prog.pct >= 80) status = 'protected';
      else status = 'freeze_used';
    }
    entries.push({
      trackingDate: dt,
      pct: prog.pct,
      goalMl: goal,
      totalEffectiveMl: prog.totalEffectiveMl,
      status,
      cupSummary: prog.cupSummary,
    });
  }
  return entries;
}

export async function rescheduleAndroidNotifications(): Promise<void> {
  if (!isTauriRuntime()) return;
  await invoke('plugin:suu-notifications|reschedule_next_reminder');
}

export async function consumeQuickDrawerLaunchIntent(): Promise<boolean> {
  if (!isTauriRuntime()) return false;
  try {
    const res = await invoke<{ openQuickDrawer?: boolean }>(
      'plugin:suu-notifications|consume_launch_intent',
    );
    return Boolean(res?.openQuickDrawer);
  } catch {
    return false;
  }
}

export async function onBackgroundIntakeUpdated(
  callback: () => void,
): Promise<UnlistenFn> {
  if (!isTauriRuntime()) return () => {};
  return await listen('suu://intake-updated', () => {
    callback();
  });
}
