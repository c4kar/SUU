<script lang="ts">
  import { onMount } from 'svelte';
  import IosEmoji from './lib/IosEmoji.svelte';
  import {
    PORTIONS,
    GOAL_MIN_ML,
    GOAL_MAX_ML,
    GOAL_STEP_ML,
    calculateScientificGoalMl,
    clampGoalMl,
    type ActivityLevel,
    type AgeBracket,
    type CalendarDayEntry,
    type DashboardState,
  } from './lib/domain';
  import {
    consumeQuickDrawerLaunchIntent,
    deleteIntake,
    getCalendarHistory,
    getDashboardState,
    logIntake,
    onBackgroundIntakeUpdated,
    updateSettings,
  } from './lib/backend';

  let dashboard = $state<DashboardState | null>(null);
  let activeTab = $state<'today' | 'yesterday'>('today');
  let showQuickDrawer = $state(false);
  let showCalendarModal = $state(false);
  let showSettingsModal = $state(false);
  let toastMessage = $state('');
  let toastIcon = $state('water_drop');
  let toastVisible = $state(false);
  let calmPulse = $state(false);
  let nightMode = $state(false);
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  // Onboarding / Settings form state
  let formWeightKg = $state(65);
  let formAgeBracket = $state<AgeBracket>('30_55');
  let formActivityLevel = $state<ActivityLevel>('low');
  let formGoalMl = $state(2000);
  let formGoalManuallyEdited = $state(false);
  let formActiveStartMin = $state(540); // 09:00
  let formActiveEndMin = $state(1320);  // 22:00
  let formIntervalMin = $state(120);    // 2 saat

  // Calendar state
  let calendarYearMonth = $state('');
  let calendarEntries = $state<CalendarDayEntry[]>([]);
  let selectedCalendarDay = $state<CalendarDayEntry | null>(null);

  const recommendedGoalMl = $derived(
    calculateScientificGoalMl(formWeightKg, formAgeBracket, formActivityLevel),
  );

  const drawerGroups = [
    {
      title: 'Çay Seçenekleri',
      emoji: 'tea_ince',
      ids: ['tea_ince', 'tea_ajda', 'tea_mug'],
      cls: 'tea-card',
    },
    {
      title: 'Kahve Seçenekleri',
      emoji: 'coffee_turk',
      ids: ['coffee_turk', 'coffee_mug'],
      cls: 'coffee-card',
    },
    {
      title: 'Ayran Seçenekleri',
      emoji: 'ayran_small',
      ids: ['ayran_small', 'ayran_large'],
      cls: 'ayran-card',
    },
    {
      title: 'Su Seçenekleri',
      emoji: 'water_drop',
      ids: ['water_glass', 'water_sip', 'water_mug', 'water_bottle'],
      cls: 'water-card',
    },
  ];

  function detectInitialNightMode(): boolean {
    try {
      const saved = localStorage.getItem('suu_theme_pref');
      if (saved === 'night') return true;
      if (saved === 'day') return false;
    } catch {
      // ignore
    }
    const hour = new Date().getHours();
    return hour >= 22 || hour < 6;
  }

  function toggleNightMode() {
    nightMode = !nightMode;
    try {
      localStorage.setItem('suu_theme_pref', nightMode ? 'night' : 'day');
    } catch {
      // ignore
    }
  }

  function triggerToast(msg: string, iconName = 'water_drop') {
    toastMessage = msg;
    toastIcon = iconName;
    toastVisible = true;
    calmPulse = true;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      toastVisible = false;
      calmPulse = false;
    }, 2400);
  }

  function syncFormFromSettings(data: DashboardState) {
    formWeightKg = data.settings.weightKg;
    formAgeBracket = data.settings.ageBracket;
    formActivityLevel = data.settings.activityLevel;
    formGoalMl = data.settings.dailyGoalMl;
    formActiveStartMin = data.settings.activeStartMin;
    formActiveEndMin = data.settings.activeEndMin;
    formIntervalMin = data.settings.intervalMin;
  }

  async function refreshState() {
    const targetDate =
      dashboard && activeTab === 'yesterday' ? dashboard.yesterdayDate : undefined;
    const data = await getDashboardState(targetDate);
    dashboard = data;
    if (!data.settings.onboardingCompleted) {
      syncFormFromSettings(data);
      formGoalMl = calculateScientificGoalMl(
        formWeightKg,
        formAgeBracket,
        formActivityLevel,
      );
    }
  }

  async function handleSelectTab(tab: 'today' | 'yesterday') {
    activeTab = tab;
    if (!dashboard) return;
    const targetDate = tab === 'today' ? dashboard.todayDate : dashboard.yesterdayDate;
    dashboard = await getDashboardState(targetDate);
  }

  async function handleAddPortion(portionId: string) {
    if (!dashboard) return;
    const p = PORTIONS[portionId];
    const targetDate =
      activeTab === 'today' ? dashboard.todayDate : dashboard.yesterdayDate;
    dashboard = await logIntake(portionId, targetDate);
    showQuickDrawer = false;
    if (p) {
      triggerToast(`+${p.title} eklendi`, p.id);
    }
  }

  async function handleRemoveLog(logId: string) {
    if (!dashboard) return;
    const targetDate =
      activeTab === 'today' ? dashboard.todayDate : dashboard.yesterdayDate;
    dashboard = await deleteIntake(logId, targetDate);
    triggerToast('Kayıt geri alındı', 'water_drop');
  }

  function adjustGoalStep(delta: number) {
    formGoalManuallyEdited = true;
    formGoalMl = clampGoalMl(formGoalMl + delta);
  }

  function handleProfileParamChange() {
    if (!formGoalManuallyEdited) {
      formGoalMl = calculateScientificGoalMl(
        formWeightKg,
        formAgeBracket,
        formActivityLevel,
      );
    }
  }

  async function handleFinishOnboarding(skipWithDefault = false) {
    const goal = skipWithDefault ? 2000 : clampGoalMl(formGoalMl);
    dashboard = await updateSettings(
      {
        onboardingCompleted: true,
        weightKg: formWeightKg,
        ageBracket: formAgeBracket,
        activityLevel: formActivityLevel,
        dailyGoalMl: goal,
        activeStartMin: formActiveStartMin,
        activeEndMin: formActiveEndMin,
        intervalMin: formIntervalMin,
      },
      true,
    );
    triggerToast('Hazırsın! İlk suyunla başlayalım', 'water_glass');
  }

  async function handleSaveSettings() {
    dashboard = await updateSettings(
      {
        onboardingCompleted: true,
        weightKg: formWeightKg,
        ageBracket: formAgeBracket,
        activityLevel: formActivityLevel,
        dailyGoalMl: clampGoalMl(formGoalMl),
        activeStartMin: formActiveStartMin,
        activeEndMin: formActiveEndMin,
        intervalMin: formIntervalMin,
      },
      false,
    );
    showSettingsModal = false;
    triggerToast('Ayarlar kaydedildi', 'water_drop');
  }

  async function openCalendar() {
    if (!dashboard) return;
    const ym = dashboard.todayDate.slice(0, 7);
    calendarYearMonth = ym;
    calendarEntries = await getCalendarHistory(ym);
    selectedCalendarDay =
      calendarEntries.find((e) => e.trackingDate === dashboard?.todayDate) ??
      calendarEntries[calendarEntries.length - 1] ??
      null;
    showCalendarModal = true;
  }

  function openSettings() {
    if (!dashboard) return;
    syncFormFromSettings(dashboard);
    formGoalManuallyEdited = true;
    showSettingsModal = true;
  }

  function formatTimeMs(ms: number): string {
    const d = new Date(ms);
    return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`;
  }

  function formatMinToClock(totalMin: number): string {
    const h = Math.floor(totalMin / 60);
    const m = totalMin % 60;
    return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}`;
  }

  onMount(() => {
    nightMode = detectInitialNightMode();
    refreshState();

    consumeQuickDrawerLaunchIntent().then((shouldOpen) => {
      if (shouldOpen) {
        showQuickDrawer = true;
      }
    });

    let unlisten: (() => void) | null = null;
    onBackgroundIntakeUpdated(() => {
      refreshState();
      consumeQuickDrawerLaunchIntent().then((shouldOpen) => {
        if (shouldOpen) showQuickDrawer = true;
      });
    }).then((fn) => {
      unlisten = fn;
    });

    const onVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        refreshState();
        consumeQuickDrawerLaunchIntent().then((shouldOpen) => {
          if (shouldOpen) showQuickDrawer = true;
        });
      }
    };
    document.addEventListener('visibilitychange', onVisibilityChange);

    return () => {
      if (unlisten) unlisten();
      document.removeEventListener('visibilitychange', onVisibilityChange);
    };
  });
</script>

<main class="app-shell" class:theme-night={nightMode}>
  {#if toastVisible}
    <div class="toast-pill" role="status">
      <IosEmoji name={toastIcon} size={20} />
      <span>{toastMessage}</span>
    </div>
  {/if}

  {#if !dashboard}
    <div class="loading-screen">
      <IosEmoji name="water_drop" size={32} />
      <span>SUU yükleniyor...</span>
    </div>
  {:else if !dashboard.settings.onboardingCompleted}
    <!-- TEK EKRANLI DOĞAL & YUMUŞAK HIZLI KURULUM (ONBOARDING) -->
    <section class="onboarding-card">
      <div class="onboard-top">
        <div>
          <div class="brand-badge">
            <IosEmoji name="water_drop" size={18} />
            <span>SUU</span>
          </div>
          <h1>Hoş Geldin</h1>
          <p class="sub">Sana en uygun günlük su hedefini birlikte belirleyelim.</p>
        </div>
        <button class="skip-btn" onclick={() => handleFinishOnboarding(true)}>
          Şimdilik Atla (10 Bardak)
        </button>
      </div>

      <div class="field-group">
        <label for="weightRange">Kilon: <b>{formWeightKg} kg</b></label>
        <input
          id="weightRange"
          type="range"
          min="40"
          max="130"
          step="1"
          bind:value={formWeightKg}
          oninput={handleProfileParamChange}
        />
      </div>

      <div class="field-group">
        <span class="field-label">Yaş Grubun</span>
        <div class="seg-row">
          <button
            type="button"
            class:active={formAgeBracket === 'under_30'}
            onclick={() => {
              formAgeBracket = 'under_30';
              handleProfileParamChange();
            }}>30 altı</button
          >
          <button
            type="button"
            class:active={formAgeBracket === '30_55'}
            onclick={() => {
              formAgeBracket = '30_55';
              handleProfileParamChange();
            }}>30 – 55</button
          >
          <button
            type="button"
            class:active={formAgeBracket === 'over_55'}
            onclick={() => {
              formAgeBracket = 'over_55';
              handleProfileParamChange();
            }}>55 üstü</button
          >
        </div>
      </div>

      <div class="field-group">
        <span class="field-label">Günlük Hareketlilik</span>
        <div class="seg-row">
          <button
            type="button"
            class:active={formActivityLevel === 'low'}
            onclick={() => {
              formActivityLevel = 'low';
              handleProfileParamChange();
            }}>Düşük</button
          >
          <button
            type="button"
            class:active={formActivityLevel === 'medium'}
            onclick={() => {
              formActivityLevel = 'medium';
              handleProfileParamChange();
            }}>Orta</button
          >
          <button
            type="button"
            class:active={formActivityLevel === 'high'}
            onclick={() => {
              formActivityLevel = 'high';
              handleProfileParamChange();
            }}>Yüksek</button
          >
        </div>
      </div>

      <div class="goal-preview-box">
        <div class="gp-label">
          Önerilen Hedef: {Math.round(recommendedGoalMl / 200)} Su Bardağı ({recommendedGoalMl} ml)
        </div>
        <div class="gp-stepper">
          <button type="button" class="step-btn" onclick={() => adjustGoalStep(-50)}>−</button>
          <div class="gp-main">
            <div class="gp-cups">
              <IosEmoji name="water_glass" size={26} />
              <span>{Math.round((formGoalMl / 200) * 10) / 10} Su Bardağı</span>
            </div>
            <div class="gp-ml">{formGoalMl} ml / gün</div>
          </div>
          <button type="button" class="step-btn" onclick={() => adjustGoalStep(50)}>+</button>
        </div>
      </div>

      <div class="reminder-summary-box">
        <div class="rs-title">🔔 Nazik Hatırlatmalar</div>
        <div class="rs-desc">
          Her gün <b>{formatMinToClock(formActiveStartMin)} – {formatMinToClock(formActiveEndMin)}</b> arası,
          <b>{formIntervalMin / 60} saatte bir</b> hatırlatılır.
        </div>
      </div>

      <button type="button" class="start-cta-btn" onclick={() => handleFinishOnboarding(false)}>
        <span>Başla</span>
        <IosEmoji name="water_drop" size={20} />
      </button>
    </section>
  {:else}
    <!-- ANA EKRAN: VARYANT A (YUMUŞAK DOĞAL TONLAR + SAKİN DALGA ANİMASYONU) -->
    {@const prog = dashboard.activeDay}
    {@const waterHeightPct = Math.min(100, (prog.pureWaterMl / prog.goalMl) * 100)}
    {@const otherHeightPct = Math.min(100 - waterHeightPct, (prog.countedOtherMl / prog.goalMl) * 100)}

    <header class="topbar">
      <div class="day-switcher">
        <button
          type="button"
          class="day-btn"
          class:active={activeTab === 'today'}
          onclick={() => handleSelectTab('today')}
        >
          Bugün
        </button>
        <button
          type="button"
          class="day-btn"
          class:active={activeTab === 'yesterday'}
          onclick={() => handleSelectTab('yesterday')}
        >
          Dün
        </button>
      </div>

      <div class="topbar-right">
        <button type="button" class="streak-chip" onclick={openCalendar} title="Takvim ve Seri Durumu">
          <IosEmoji name="🔥" size={18} />
          <span>{dashboard.streakDays} Gün</span>
          {#if dashboard.weeklyFreezeAvailable}
            <IosEmoji name="🛡️" size={16} />
          {/if}
        </button>
        <button type="button" class="icon-btn" onclick={openCalendar} aria-label="Takvim">
          <IosEmoji name="📅" size={20} />
        </button>
        <button type="button" class="icon-btn" onclick={toggleNightMode} aria-label="Gece / Gündüz Modu">
          <IosEmoji name={nightMode ? '☀️' : '🌙'} size={20} />
        </button>
        <button type="button" class="icon-btn" onclick={openSettings} aria-label="Ayarlar">
          <IosEmoji name="⚙️" size={20} />
        </button>
      </div>
    </header>

    <section class="main-scroll">
      <!-- Merkezi Canlı Su Haznesi (Sakin Nefes Alan Dalga) -->
      <div class="water-tank" class:calm-pulse={calmPulse}>
        <!-- Saf Su Katmanı -->
        <div class="tank-layer-water" style="height: {waterHeightPct}%">
          <svg class="calm-wave" viewBox="0 0 1200 60" preserveAspectRatio="none">
            <path
              d="M0,30 C300,58 600,2 900,30 C1050,44 1150,20 1200,30 L1200,60 L0,60 Z"
              fill="currentColor"
            />
          </svg>
          {#if calmPulse}
            <span class="soft-bubble b1"></span>
            <span class="soft-bubble b2"></span>
            <span class="soft-bubble b3"></span>
          {/if}
        </div>

        <!-- Çay / Kahve / Ayran Katmanı -->
        <div
          class="tank-layer-other"
          style="bottom: {waterHeightPct}%; height: {otherHeightPct}%"
        ></div>

        <div class="tank-center-card">
          <div class="tank-pct">%{prog.pct}</div>
          <div class="tank-headline">
            {#if prog.remainingMl > 0}
              Hedefe <b>{prog.remainingGlasses} Su Bardağı</b> kaldı
            {:else}
              Günlük Hedef Tamamlandı
            {/if}
          </div>
          <div class="tank-summary">{prog.cupSummary}</div>
        </div>
      </div>

      <!-- Sade ve Doğal Kota Kutuları -->
      <div class="quota-grid">
        <div class="quota-card">
          <div class="q-head">
            <span class="q-title-inline">
              <IosEmoji name="water_drop" size={15} />
              <span>İçilen Saf Su</span>
            </span>
          </div>
          <div class="q-val water-col">
            {Math.round((prog.pureWaterMl / 200) * 10) / 10} Su Bardağı
          </div>
          <div class="q-bar">
            <div
              class="q-fill water-bg"
              style="width: {Math.min(100, (prog.pureWaterMl / prog.minPureWaterMl) * 100)}%"
            ></div>
          </div>
        </div>

        <div class="quota-card">
          <div class="q-head">
            <span class="q-title-inline">
              <IosEmoji name="tea_ince" size={16} />
              <span>Çay / Kahve Hakkı</span>
            </span>
            {#if prog.isOtherCapReached}
              <span class="cap-tag">DOLDU</span>
            {/if}
          </div>
          <div class="q-val other-col">
            %{Math.min(100, Math.round((prog.countedOtherMl / prog.otherCapMl) * 100))} kullanıldı
          </div>
          <div class="q-bar">
            <div
              class="q-fill other-bg"
              style="width: {Math.min(100, (prog.countedOtherMl / prog.otherCapMl) * 100)}%"
            ></div>
          </div>
        </div>
      </div>

      {#if prog.isOtherCapReached}
        <div class="cap-banner">
          <IosEmoji name="tea_ince" size={20} />
          <span>
            Bugünkü çay/kahve hakkın doldu. Kalan hedefini tamamlamak için <b>Saf Su</b> içmelisin.
          </span>
        </div>
      {/if}

      <!-- Bugün / Dün İçtiklerim Listesi -->
      <div class="logs-card">
        <div class="logs-head">
          <span>{activeTab === 'today' ? 'Bugün İçtiklerim' : 'Dün İçtiklerim (Telafi Açık)'}</span>
          <span class="logs-count">{prog.logs.length} kayıt</span>
        </div>

        {#if prog.logs.length === 0}
          <div class="empty-logs">Henüz içecek eklenmedi. Aşağıdaki butonlara dokunarak başlayabilirsin.</div>
        {:else}
          <div class="logs-list">
            {#each prog.logs as item (item.id)}
              {@const p = PORTIONS[item.portionId]}
              <div class="log-row">
                <div class="log-info">
                  <IosEmoji name={item.portionId} size={28} />
                  <div>
                    <div class="log-title">{p?.title ?? 'İçecek'}</div>
                    <div class="log-time">{formatTimeMs(item.loggedAtMs)}</div>
                  </div>
                </div>
                <button
                  type="button"
                  class="delete-btn"
                  onclick={() => handleRemoveLog(item.id)}
                >
                  Sil
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </section>

    <!-- Alt Sabit 3'lü Hızlı Bar (Bildirim Butonlarıyla Birebir Aynı) -->
    <footer class="bottom-dock">
      <button
        type="button"
        class="dock-btn btn-glass"
        onclick={() => handleAddPortion('water_glass')}
      >
        <IosEmoji name="water_glass" size={28} />
        <span class="d-title">+1 Su Bardağı</span>
      </button>

      <button
        type="button"
        class="dock-btn btn-sip"
        onclick={() => handleAddPortion('water_sip')}
      >
        <IosEmoji name="water_drop" size={26} />
        <span class="d-title">+1 Yudum</span>
      </button>

      <button
        type="button"
        class="dock-btn btn-other"
        onclick={() => (showQuickDrawer = true)}
      >
        <IosEmoji name="tea_ince" size={28} />
        <span class="d-title">Diğer</span>
      </button>
    </footer>
  {/if}

  <!-- HIZLI SEÇİM ÇEKMECESİ ("Diğer" Butonu veya Bildirimden Açılır) -->
  {#if showQuickDrawer}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="sheet-backdrop" onclick={(e) => e.target === e.currentTarget && (showQuickDrawer = false)}>
      <div class="sheet-panel">
        <div class="sheet-handle"></div>
        <div class="sheet-header">
          <h2>Ne İçtin?</h2>
          <button type="button" class="close-pill" onclick={() => (showQuickDrawer = false)}>
            Kapat ✕
          </button>
        </div>

        {#each drawerGroups as group}
          <div class="group-title">
            <IosEmoji name={group.emoji} size={18} />
            <span>{group.title}</span>
          </div>
          <div class="portion-grid">
            {#each group.ids as pid}
              {@const p = PORTIONS[pid]}
              <button
                type="button"
                class="portion-card {group.cls}"
                onclick={() => handleAddPortion(pid)}
              >
                <IosEmoji name={pid} size={32} />
                <span class="p-name">{p.title}</span>
              </button>
            {/each}
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- TAKVİM & SERİ GEÇMİŞİ ÇEKMECESİ -->
  {#if showCalendarModal && dashboard}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="sheet-backdrop" onclick={(e) => e.target === e.currentTarget && (showCalendarModal = false)}>
      <div class="sheet-panel">
        <div class="sheet-handle"></div>
        <div class="sheet-header">
          <h2 class="inline-h2">
            <IosEmoji name="🔥" size={22} />
            <span>{dashboard.streakDays} Günlük Seri ({calendarYearMonth})</span>
          </h2>
          <button type="button" class="close-pill" onclick={() => (showCalendarModal = false)}>
            Kapat ✕
          </button>
        </div>

        <p class="cal-sub">
          {dashboard.weeklyFreezeAvailable
            ? 'Bu haftaki 1 günlük Seri Koruma (Mola) hakkın hazır.'
            : 'Bu haftaki Seri Koruma hakkın kullanıldı, serin güvende.'}
        </p>

        <div class="calendar-grid">
          {#each calendarEntries as day}
            <button
              type="button"
              class="cal-day status-{day.status}"
              class:selected={selectedCalendarDay?.trackingDate === day.trackingDate}
              onclick={() => (selectedCalendarDay = day)}
            >
              <span class="c-num">{Number(day.trackingDate.slice(-2))}</span>
              <span class="c-pct">{day.pct > 0 ? `%${day.pct}` : '·'}</span>
            </button>
          {/each}
        </div>

        {#if selectedCalendarDay}
          <div class="cal-detail-box">
            <div class="cd-date">
              {selectedCalendarDay.trackingDate} — <b>%{selectedCalendarDay.pct}</b>
            </div>
            <div class="cd-summary">{selectedCalendarDay.cupSummary}</div>
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <!-- ULTRA SADE AYARLAR ÇEKMECESİ -->
  {#if showSettingsModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="sheet-backdrop" onclick={(e) => e.target === e.currentTarget && (showSettingsModal = false)}>
      <div class="sheet-panel">
        <div class="sheet-handle"></div>
        <div class="sheet-header">
          <h2 class="inline-h2">
            <IosEmoji name="⚙️" size={20} />
            <span>Hedef ve Hatırlatma Ayarları</span>
          </h2>
          <button type="button" class="close-pill" onclick={() => (showSettingsModal = false)}>
            Kapat ✕
          </button>
        </div>

        <div class="goal-preview-box">
          <div class="gp-label">Günlük Su Hedefin ({GOAL_MIN_ML} – {GOAL_MAX_ML} ml)</div>
          <div class="gp-stepper">
            <button type="button" class="step-btn" onclick={() => adjustGoalStep(-GOAL_STEP_ML)}>−</button>
            <div class="gp-main">
              <div class="gp-cups">
                <IosEmoji name="water_glass" size={24} />
                <span>{Math.round((formGoalMl / 200) * 10) / 10} Su Bardağı</span>
              </div>
              <div class="gp-ml">{formGoalMl} ml / gün</div>
            </div>
            <button type="button" class="step-btn" onclick={() => adjustGoalStep(GOAL_STEP_ML)}>+</button>
          </div>
        </div>

        <div class="field-group">
          <span class="field-label">Hatırlatma Sıklığı (09:00 – 22:00 arası)</span>
          <div class="seg-row">
            {#each [60, 90, 120, 180] as mins}
              <button
                type="button"
                class:active={formIntervalMin === mins}
                onclick={() => (formIntervalMin = mins)}
              >
                {mins === 60
                  ? '1 Saat'
                  : mins === 90
                    ? '1.5 Saat'
                    : mins === 120
                      ? '2 Saat'
                      : '3 Saat'}
              </button>
            {/each}
          </div>
        </div>

        <button type="button" class="start-cta-btn" onclick={handleSaveSettings}>
          Ayarları Kaydet ✓
        </button>
      </div>
    </div>
  {/if}
</main>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    -webkit-tap-highlight-color: transparent;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Rounded', 'Segoe UI', Roboto, sans-serif;
  }

  /* AÇIK TEMA — YUMUŞAK TONLAR VE DOĞAL RENKLER (VARSAYILAN) */
  .app-shell {
    --bg-main: #fbf9f5;
    --bg-card: #ffffff;
    --bg-subtle: #f3efe8;
    --border-soft: #eae4da;
    --text-primary: #2c3539;
    --text-secondary: #6c787e;
    --water-light: #e8f4fa;
    --water-border: #c5e2f2;
    --water-fill-top: rgba(91, 168, 214, 0.78);
    --water-fill-bot: rgba(54, 132, 181, 0.9);
    --water-accent: #2f7bb0;
    --amber-fill: rgba(219, 146, 78, 0.72);
    --amber-accent: #b86b28;
    --center-glass: rgba(255, 255, 255, 0.86);

    max-width: 480px;
    margin: 0 auto;
    min-height: 100dvh;
    display: flex;
    flex-direction: column;
    background: var(--bg-main);
    color: var(--text-primary);
    position: relative;
    transition:
      background-color 0.45s ease,
      color 0.45s ease;
  }

  /* YUMUŞAK GECE MODU (GÖZ YORMAYAN DOĞAL LACİVERT/TAŞ TONLARI) */
  .app-shell.theme-night {
    --bg-main: #171f26;
    --bg-card: #212b34;
    --bg-subtle: #293540;
    --border-soft: #32404d;
    --text-primary: #edf2f6;
    --text-secondary: #9fb0bc;
    --water-light: #1d2d3a;
    --water-border: #2e485c;
    --water-fill-top: rgba(68, 142, 189, 0.74);
    --water-fill-bot: rgba(41, 105, 148, 0.88);
    --water-accent: #73bce8;
    --amber-fill: rgba(201, 130, 65, 0.68);
    --amber-accent: #efb071;
    --center-glass: rgba(33, 43, 52, 0.86);
  }

  :global(body) {
    background: #fbf9f5;
    overscroll-behavior: none;
    user-select: none;
  }

  /* SAKİN BİLDİRİM TOAST */
  .toast-pill {
    position: fixed;
    top: 18px;
    left: 50%;
    transform: translateX(-50%);
    background: var(--bg-card);
    color: var(--text-primary);
    border: 1px solid var(--border-soft);
    font-size: 14px;
    font-weight: 700;
    padding: 9px 16px;
    border-radius: 999px;
    box-shadow: 0 8px 24px rgba(44, 53, 57, 0.12);
    z-index: 100;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    white-space: nowrap;
    animation: calmFadeIn 0.35s ease;
  }
  @keyframes calmFadeIn {
    from {
      opacity: 0;
      transform: translate(-50%, -8px);
    }
    to {
      opacity: 1;
      transform: translate(-50%, 0);
    }
  }

  .loading-screen {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    font-size: 17px;
    font-weight: 700;
    color: var(--water-accent);
  }

  /* ONBOARDING */
  .onboarding-card {
    flex: 1;
    padding: 24px 20px;
    display: flex;
    flex-direction: column;
    gap: 15px;
  }
  .onboard-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 10px;
  }
  .brand-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 800;
    color: var(--water-accent);
    letter-spacing: 0.04em;
  }
  .onboard-top h1 {
    font-size: 24px;
    font-weight: 800;
    margin-top: 4px;
  }
  .sub {
    font-size: 14px;
    color: var(--text-secondary);
    margin-top: 3px;
  }
  .skip-btn {
    border: 1px solid var(--border-soft);
    background: var(--bg-card);
    color: var(--text-secondary);
    padding: 8px 13px;
    border-radius: 999px;
    font-size: 12px;
    font-weight: 700;
    cursor: pointer;
    white-space: nowrap;
  }

  .field-group {
    background: var(--bg-card);
    border: 1px solid var(--border-soft);
    border-radius: 18px;
    padding: 13px 15px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .field-label,
  .field-group label {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
  }
  .field-group input[type='range'] {
    width: 100%;
    accent-color: var(--water-accent);
  }
  .seg-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(70px, 1fr));
    gap: 8px;
  }
  .seg-row button {
    border: 1.5px solid var(--border-soft);
    background: var(--bg-subtle);
    color: var(--text-secondary);
    padding: 10px 8px;
    border-radius: 13px;
    font-size: 14px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.25s ease;
  }
  .seg-row button.active {
    border-color: var(--water-accent);
    background: var(--water-light);
    color: var(--water-accent);
  }

  .goal-preview-box {
    background: var(--water-light);
    border: 1.5px solid var(--water-border);
    border-radius: 22px;
    padding: 15px;
    text-align: center;
  }
  .gp-label {
    font-size: 13px;
    font-weight: 700;
    color: var(--water-accent);
  }
  .gp-stepper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 10px;
  }
  .step-btn {
    width: 46px;
    height: 46px;
    border-radius: 15px;
    border: none;
    background: var(--water-accent);
    color: #ffffff;
    font-size: 22px;
    font-weight: 700;
    cursor: pointer;
  }
  .gp-cups {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-size: 22px;
    font-weight: 800;
    color: var(--text-primary);
  }
  .gp-ml {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .reminder-summary-box {
    background: var(--bg-card);
    border: 1px solid var(--border-soft);
    border-radius: 18px;
    padding: 13px 15px;
  }
  .rs-title {
    font-size: 13px;
    font-weight: 800;
    color: var(--amber-accent);
  }
  .rs-desc {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: 4px;
    line-height: 1.4;
  }

  .start-cta-btn {
    margin-top: auto;
    border: none;
    background: linear-gradient(135deg, #55a3d1, #3782b3);
    color: #ffffff;
    padding: 16px;
    border-radius: 20px;
    font-size: 17px;
    font-weight: 800;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    box-shadow: 0 8px 20px rgba(55, 130, 179, 0.22);
  }

  /* TOPBAR */
  .topbar {
    padding: 12px 16px 6px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .day-switcher {
    display: inline-flex;
    background: var(--bg-subtle);
    padding: 3px;
    border-radius: 999px;
    border: 1px solid var(--border-soft);
  }
  .day-btn {
    border: none;
    background: transparent;
    padding: 6px 14px;
    border-radius: 999px;
    font-size: 14px;
    font-weight: 700;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.25s ease;
  }
  .day-btn.active {
    background: var(--bg-card);
    color: var(--text-primary);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.06);
  }
  .topbar-right {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .streak-chip {
    border: 1px solid var(--border-soft);
    background: var(--bg-card);
    color: var(--text-primary);
    padding: 6px 11px;
    border-radius: 999px;
    font-size: 13px;
    font-weight: 800;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 5px;
  }
  .icon-btn {
    width: 36px;
    height: 36px;
    border-radius: 13px;
    border: 1px solid var(--border-soft);
    background: var(--bg-card);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }

  /* MERKEZİ SU HAZNESİ & SAKİN ANİMASYONLAR */
  .main-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 8px 16px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .water-tank {
    position: relative;
    height: 256px;
    border-radius: 30px;
    background: var(--water-light);
    border: 1.5px solid var(--water-border);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: inset 0 4px 18px rgba(55, 130, 179, 0.08);
    transition: transform 0.6s cubic-bezier(0.22, 1, 0.36, 1);
  }
  .water-tank.calm-pulse {
    transform: scale(1.008);
  }
  .tank-layer-water {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(180deg, var(--water-fill-top), var(--water-fill-bot));
    color: var(--water-fill-top);
    transition: height 1.1s cubic-bezier(0.22, 1, 0.36, 1);
    z-index: 1;
  }
  .calm-wave {
    position: absolute;
    top: -16px;
    left: -20%;
    width: 140%;
    height: 18px;
    animation: gentleBreath 9s ease-in-out infinite alternate;
  }
  @keyframes gentleBreath {
    from {
      transform: translateX(-4%);
    }
    to {
      transform: translateX(4%);
    }
  }
  .soft-bubble {
    position: absolute;
    bottom: 10px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.55);
    animation: calmRise 2.4s ease-out forwards;
  }
  .soft-bubble.b1 {
    left: 30%;
  }
  .soft-bubble.b2 {
    left: 52%;
    width: 11px;
    height: 11px;
    animation-delay: 0.25s;
  }
  .soft-bubble.b3 {
    left: 68%;
    animation-delay: 0.5s;
  }
  @keyframes calmRise {
    from {
      transform: translateY(0) scale(0.8);
      opacity: 0.7;
    }
    to {
      transform: translateY(-90px) scale(1.15);
      opacity: 0;
    }
  }

  .tank-layer-other {
    position: absolute;
    left: 0;
    right: 0;
    background: var(--amber-fill);
    transition:
      height 1.1s cubic-bezier(0.22, 1, 0.36, 1),
      bottom 1.1s cubic-bezier(0.22, 1, 0.36, 1);
    border-top: 1.5px dashed rgba(255, 255, 255, 0.75);
    z-index: 2;
  }

  .tank-center-card {
    position: relative;
    z-index: 3;
    background: var(--center-glass);
    backdrop-filter: blur(8px);
    padding: 15px 22px;
    border-radius: 24px;
    text-align: center;
    max-width: 88%;
    border: 1px solid rgba(255, 255, 255, 0.45);
    box-shadow: 0 6px 20px rgba(44, 53, 57, 0.06);
  }
  .tank-pct {
    font-size: 36px;
    font-weight: 800;
    line-height: 1;
    color: var(--text-primary);
  }
  .tank-headline {
    font-size: 16px;
    font-weight: 700;
    color: var(--water-accent);
    margin-top: 6px;
  }
  .tank-summary {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-top: 5px;
  }

  /* DOĞAL KOTA KUTULARI */
  .quota-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }
  .quota-card {
    background: var(--bg-card);
    border: 1px solid var(--border-soft);
    border-radius: 18px;
    padding: 11px 13px;
  }
  .q-head {
    font-size: 12px;
    font-weight: 700;
    color: var(--text-secondary);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .q-title-inline {
    display: inline-flex;
    align-items: center;
    gap: 5px;
  }
  .cap-tag {
    background: #faecd8;
    color: #9c5618;
    padding: 2px 6px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 800;
  }
  .q-val {
    font-size: 15px;
    font-weight: 800;
    margin-top: 5px;
  }
  .water-col {
    color: var(--water-accent);
  }
  .other-col {
    color: var(--amber-accent);
  }
  .q-bar {
    height: 6px;
    background: var(--bg-subtle);
    border-radius: 999px;
    margin-top: 7px;
    overflow: hidden;
  }
  .q-fill {
    height: 100%;
    border-radius: 999px;
    transition: width 0.8s cubic-bezier(0.22, 1, 0.36, 1);
  }
  .water-bg {
    background: #4d9bc9;
  }
  .other-bg {
    background: #db924e;
  }

  .cap-banner {
    background: var(--bg-card);
    border: 1.5px solid #ebcfa8;
    color: var(--amber-accent);
    padding: 11px 13px;
    border-radius: 16px;
    font-size: 13px;
    font-weight: 600;
    line-height: 1.4;
    display: flex;
    align-items: center;
    gap: 10px;
  }

  /* KAYIT LİSTESİ */
  .logs-card {
    background: var(--bg-card);
    border: 1px solid var(--border-soft);
    border-radius: 20px;
    padding: 13px 15px;
  }
  .logs-head {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
    font-weight: 700;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }
  .empty-logs {
    font-size: 13px;
    color: var(--text-secondary);
    padding: 8px 0;
  }
  .log-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 9px 0;
    border-top: 1px solid var(--border-soft);
  }
  .log-row:first-of-type {
    border-top: none;
  }
  .log-info {
    display: flex;
    align-items: center;
    gap: 11px;
  }
  .log-title {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
  }
  .log-time {
    font-size: 11px;
    color: var(--text-secondary);
    font-weight: 600;
    margin-top: 1px;
  }
  .delete-btn {
    border: 1px solid var(--border-soft);
    background: var(--bg-subtle);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 700;
    padding: 6px 12px;
    border-radius: 10px;
    cursor: pointer;
  }

  /* ALT 3'LÜ SABİT HIZLI BAR (DOĞAL & YUMUŞAK) */
  .bottom-dock {
    padding: 10px 16px calc(14px + env(safe-area-inset-bottom, 0px));
    background: var(--bg-card);
    border-top: 1px solid var(--border-soft);
    display: grid;
    grid-template-columns: 1.3fr 1fr 1fr;
    gap: 10px;
  }
  .dock-btn {
    border: none;
    border-radius: 20px;
    padding: 13px 8px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    cursor: pointer;
    transition: transform 0.2s ease;
  }
  .dock-btn:active {
    transform: scale(0.97);
  }
  .btn-glass {
    background: linear-gradient(135deg, #56a3d1, #3882b3);
    color: #ffffff;
    box-shadow: 0 6px 16px rgba(56, 130, 179, 0.22);
  }
  .btn-sip {
    background: var(--water-light);
    color: var(--water-accent);
    border: 1px solid var(--water-border);
  }
  .btn-other {
    background: #f9efe2;
    color: #8c5220;
    border: 1px solid #ecd8be;
  }
  .theme-night .btn-other {
    background: #2f2822;
    color: #efb071;
    border-color: #4a3c31;
  }
  .d-title {
    font-size: 13px;
    font-weight: 800;
  }

  /* ALT ÇEKMECELER */
  .sheet-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(23, 31, 38, 0.42);
    backdrop-filter: blur(2px);
    z-index: 50;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    animation: calmBackdrop 0.25s ease;
  }
  @keyframes calmBackdrop {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  .sheet-panel {
    width: 100%;
    max-width: 480px;
    max-height: 85dvh;
    overflow-y: auto;
    background: var(--bg-card);
    color: var(--text-primary);
    border-radius: 28px 28px 0 0;
    padding: 16px 18px 28px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: calmSheetUp 0.32s cubic-bezier(0.22, 1, 0.36, 1);
  }
  @keyframes calmSheetUp {
    from {
      transform: translateY(40px);
      opacity: 0.6;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
  .sheet-handle {
    width: 42px;
    height: 5px;
    background: var(--border-soft);
    border-radius: 999px;
    margin: 0 auto 4px;
  }
  .sheet-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .sheet-header h2 {
    font-size: 17px;
    font-weight: 800;
  }
  .inline-h2 {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }
  .close-pill {
    border: 1px solid var(--border-soft);
    background: var(--bg-subtle);
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 700;
    padding: 6px 12px;
    border-radius: 999px;
    cursor: pointer;
  }
  .group-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-secondary);
    margin-top: 4px;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .portion-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 9px;
  }
  .portion-card {
    border: 1.5px solid var(--border-soft);
    background: var(--bg-subtle);
    border-radius: 18px;
    padding: 12px 11px;
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    text-align: left;
    transition: transform 0.2s ease;
  }
  .portion-card:active {
    transform: scale(0.97);
  }
  .portion-card.tea-card {
    background: #fcf5ec;
    border-color: #eedbc3;
  }
  .portion-card.coffee-card {
    background: #f7f4f0;
    border-color: #e3dcd3;
  }
  .portion-card.ayran-card {
    background: #f2f8f5;
    border-color: #cfe3da;
  }
  .portion-card.water-card {
    background: #eef6fb;
    border-color: #cbe2f0;
  }
  .theme-night .portion-card {
    background: var(--bg-subtle);
    border-color: var(--border-soft);
  }
  .p-name {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-primary);
  }

  /* TAKVİM */
  .cal-sub {
    font-size: 13px;
    color: var(--text-secondary);
    font-weight: 600;
  }
  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 6px;
  }
  .cal-day {
    border: 1.5px solid var(--border-soft);
    background: var(--bg-subtle);
    color: var(--text-primary);
    border-radius: 13px;
    padding: 8px 2px;
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
  }
  .cal-day.status-completed {
    background: #e3f4e8;
    border-color: #a8dab6;
    color: #235c36;
  }
  .cal-day.status-protected {
    background: #e6f2fa;
    border-color: #b3d7ee;
    color: #275d80;
  }
  .cal-day.status-freeze_used {
    background: #faefe0;
    border-color: #ebd0ab;
    color: #7d4e1d;
  }
  .cal-day.selected {
    outline: 2px solid var(--water-accent);
  }
  .c-num {
    font-size: 13px;
    font-weight: 800;
  }
  .c-pct {
    font-size: 10px;
    font-weight: 600;
    opacity: 0.85;
  }
  .cal-detail-box {
    background: var(--bg-subtle);
    border: 1px solid var(--border-soft);
    border-radius: 16px;
    padding: 13px;
  }
  .cd-date {
    font-size: 13px;
    color: var(--text-secondary);
  }
  .cd-summary {
    font-size: 14px;
    font-weight: 700;
    margin-top: 4px;
  }
</style>
