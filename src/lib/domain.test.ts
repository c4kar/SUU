import { describe, expect, test } from 'bun:test';
import {
  PORTIONS,
  calculateScientificGoalMl,
  computeDayProgress,
  evaluateStreak,
  getTrackingDate,
  type IntakeLog,
} from './domain';

function makeLog(portionId: string, trackingDate = '2026-11-01'): IntakeLog {
  const p = PORTIONS[portionId];
  return {
    id: `${portionId}-${Math.random()}`,
    trackingDate,
    loggedAtMs: Date.now(),
    category: p.category,
    portionId: p.id,
    grossMl: p.grossMl,
    effectiveMl: p.effectiveMl,
    source: 'app',
  };
}

describe('SUU Domain Model', () => {
  test('Bilimsel günlük hedef hesabı yaş ve hareketliliğe göre 50 ml adımla yuvarlanır', () => {
    // 65 kg, 30-55 yaş (32 ml/kg) = 2080 -> 2100 ml
    expect(calculateScientificGoalMl(65, '30_55', 'low')).toBe(2100);
    // 60 kg, >55 yaş (28 ml/kg) + orta (300) = 1980 -> 2000 ml
    expect(calculateScientificGoalMl(60, 'over_55', 'medium')).toBe(2000);
  });

  test('%30 Diğer İçecek Kotası sert tavan olarak uygulanır ve kalan %70 saf su ister', () => {
    // Hedef 2000 ml -> Diğer içecek tavanı 600 ml
    // 5 Ajda çay = 5 * 135 = 675 ml (600 ml tavanı aşar!)
    const logs = [
      makeLog('tea_ajda'),
      makeLog('tea_ajda'),
      makeLog('tea_ajda'),
      makeLog('tea_ajda'),
      makeLog('tea_ajda'),
    ];
    const prog = computeDayProgress('2026-11-01', logs, 2000);
    expect(prog.rawOtherEffectiveMl).toBe(675);
    expect(prog.countedOtherMl).toBe(600);
    expect(prog.totalEffectiveMl).toBe(600);
    expect(prog.pct).toBe(30);
    expect(prog.isOtherCapReached).toBe(true);
    expect(prog.remainingGlasses).toBe(7); // Kalan 1400 ml = 7 Su Bardağı
  });

  test('04:00 uyku odaklı gün dönümü gece 01:30 kaydını önceki güne yazar', () => {
    const night = new Date(2026, 10, 2, 1, 30, 0); // 2 Kasım 01:30
    expect(getTrackingDate(night)).toBe('2026-11-01');

    const morning = new Date(2026, 10, 2, 4, 5, 0); // 2 Kasım 04:05
    expect(getTrackingDate(morning)).toBe('2026-11-02');
  });

  test('İki katmanlı esnek Günlük Seri (%80 koruma eşiği + haftalık 1 mola hakkı)', () => {
    const map = {
      '2026-10-26': 100, // Pzt: +1 (seri 1)
      '2026-10-27': 100, // Sal: +1 (seri 2)
      '2026-10-28': 85,  // Çar: %85 Koruma Eşiği -> seri 2 kalır, mola harcanmaz!
      '2026-10-29': 50,  // Per: <%80 -> Haftalık mola hakkı kullanılır, seri 2 kalır!
      '2026-10-30': 100, // Cum: +1 (seri 3)
    };
    const res = evaluateStreak('2026-10-30', map, '2026-10-26');
    expect(res.streakDays).toBe(3);
    expect(res.weeklyFreezeAvailable).toBe(false);
  });
});
