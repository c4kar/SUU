# SUU — Ürün ve Teknik Spesifikasyon (`spec.md`)

> **Durum**: Kararlaştırıldı (Wayfinder Haritası Tamamlandı)  
> **Hedef Platform**: Android (Özellikle Samsung One UI cihazlar — yerel APK kurulumu)  
> **Teknoloji Yığını**: Tauri v2 (Rust + Özel Kotlin Mobil Eklentisi `suu-notifications`) + Svelte 5 + Vite+ + SQLite (`WAL` Modu)  
> **Domain Sözlüğü**: [`CONTEXT.md`](../../CONTEXT.md)  
> **Etkileşimli Arayüz Prototipi**: [`.scratch/suu/prototype/ui-prototype.html`](./prototype/ui-prototype.html)

---

## 1. Vizyon ve Hedef Kitle İlkeleri

- **Hedef Kullanıcı Profili**: Günde 1 litreden az su içen, teknolojiden uzak, karmaşık grafik ve ayarlarla uğraşmak istemeyen genç-yetişkin kullanıcılar (anne ve abla profili).
- **Temel Tasarım İlkesi (Sıfır Sürtünme ve Günlük Dil)**:
  1. Kullanıcı arayüzünde ve bildirimlerde `×0.90 katsayı`, `net 135 ml`, `efektif hidrasyon` gibi teknik veya matematiksel ifadeler **gösterilmez**.
  2. Tüm porsiyonlar ve kalan hedef günlük bardak diliyle ifade edilir: **`1 Su Bardağı`**, **`1 Yudum`**, **`1 İnce Belli Çay`**, **`1 Fincan Türk Kahvesi`**, **"Hedefe 4 Su Bardağı kaldı"**.
  3. Katsayı ve kota matematiği tamamen arka planda sessizce çalışır.

---

## 2. Domain Kuralları ve Matematiksel Model

### 2.1. İlk Kurulum (Onboarding) ve Günlük Hedef Hesabı
- **Tek Ekranlı Hızlı Kurulum**: İlk açılışta kullanıcıdan tek ekranda yalnızca 3 bilgi istenir:
  1. **Kilo (kg)** (Varsayılan: `65 kg`)
  2. **Yaş Grubu**: `<30` (`35 ml/kg`), `30–55` (`32 ml/kg`), `>55` (`28 ml/kg`)
  3. **Günlük Hareketlilik**: `Düşük` (`+0 ml`), `Orta` (`+300 ml`), `Yüksek` (`+500 ml`)
- **"Şimdilik Atla" Seçeneği**: Sağ üstte *"Şimdilik Atla (2.000 ml ile başla)"* butonu bulunur; basılırsa **Günlük Hedef** doğrudan `2.000 ml` (10 Su Bardağı) olarak atanır.
- **Formül**:
  $$\text{Önerilen Hedef (ml)} = \text{round}_{50}\left(\text{Kilo} \times k_{\text{yaş}} + \Delta_{\text{aktivite}}\right)$$
- **Serbest Manuel Ayar**: Kullanıcı önerilen hedefi kurulumda veya Ayarlar'da `50 ml` adımlarla **`1.200 ml – 4.500 ml`** aralığında dilediği gibi değiştirebilir.
- **Gün İçi Hedef Değişikliği**: Gün içinde hedef değiştirildiğinde **bugünün** ilerlemesi ve %30'luk **Diğer İçecek Kotası** anında yeni hedef üzerinden hesaplanır; geçmiş günlerin özetleri ise kendi günkü hedefleriyle mühürlü kalır.

### 2.2. İçecek Türleri, Porsiyonlar ve Hidrasyon Katsayıları
Serbest mililitre (klavye) girişi yoktur. Sabit porsiyon tablosu:

| Porsiyon Kimliği (`portion_id`) | İçecek Türü (`category`) | Ekran / Bildirim Etiketi | Brüt Hacim (`gross_ml`) | Hidrasyon Katsayısı (`factor`) | Efektif Katkı (`effective_ml`) |
| :--- | :--- | :--- | :---: | :---: | :---: |
| `water_glass` | `water` | **1 Su Bardağı** *(Varsayılan)* | `200 ml` | `1.00` | **`200 ml`** |
| `water_sip` | `water` | **1 Yudum Su** | `50 ml` | `1.00` | **`50 ml`** |
| `water_mug` | `water` | **1 Kupa Su** | `300 ml` | `1.00` | **`300 ml`** |
| `water_bottle` | `water` | **1 Şişe / Matara Su** | `500 ml` | `1.00` | **`500 ml`** |
| `tea_ince` | `tea` | **1 İnce Belli Çay** *(Varsayılan)* | `100 ml` | `0.90` | **`90 ml`** |
| `tea_ajda` | `tea` | **1 Ajda Çay** | `150 ml` | `0.90` | **`135 ml`** |
| `tea_mug` | `tea` | **1 Kupa Çay** | `200 ml` | `0.90` | **`180 ml`** |
| `coffee_turk` | `coffee` | **1 Fincan Türk Kahvesi** *(Varsayılan)* | `70 ml` | `0.80` | **`56 ml`** |
| `coffee_mug` | `coffee` | **1 Kupa Filtre Kahve** | `200 ml` | `0.80` | **`160 ml`** |
| `ayran_small` | `ayran` | **1 Küçük Bardak Ayran** *(Varsayılan)* | `200 ml` | `1.00` | **`200 ml`** |
| `ayran_large` | `ayran` | **1 Büyük Bardak Ayran** | `300 ml` | `1.00` | **`300 ml`** |

### 2.3. Saf Su Alt Sınırı (%70) ve Diğer İçecek Kotası (%30 Sert Tavan)
- Günlük hedefin **en az %70'i** (**Saf Su Alt Sınırı**) doğrudan saf sudan (`category = 'water'`) karşılanmalıdır.
- Su dışındaki içeceklerin (`tea`, `coffee`, `ayran`) günlük hedef ilerlemesine toplam katkısı **Günlük Hedef'in en fazla %30'u** (**Diğer İçecek Kotası**) ile sınırlandırılır:
  $$\text{Sayılan Diğer İçecek (ml)} = \min\left(\sum \text{effective\_ml}_{\text{diğer}},\; \text{round}(\text{Günlük Hedef} \times 0.30)\right)$$
  $$\text{Toplam Net İlerleme (ml)} = \sum \text{effective\_ml}_{\text{su}} + \text{Sayılan Diğer İçecek (ml)}$$
  $$\text{Kalan Su Bardağı Sayısı} = \left\lceil \frac{\max(0,\; \text{Günlük Hedef} - \text{Toplam Net İlerleme})}{200} \right\rceil$$
- Kota dolduktan sonra içilen çay/kahve/ayran **Tüketim Kaydı** olarak geçmişe eklenmeye devam eder (kayıt engellenmez), ancak hedef ilerlemesi yalnızca saf su eklendikçe artar.

### 2.4. Gün Dönümü (`04:00`), Günlük Seri (Streak) ve "Dün" Telafisi
- **Uyku Odaklı Gün Dönümü (`04:00`)**: Bir takip günü (`tracking_date: YYYY-MM-DD`) sabaha karşı **`04:00`**'te başlar ve ertesi gece `03:59:59`'da biter. Yerel saat `00:00 – 03:59` arasındayken yapılan kayıtlar bir önceki takvim gününe yazılır:
  $$\text{tracking\_date}(t) = \text{ local\_date}(t - 4\text{ saat})$$
- **İki Katmanlı Esnek Günlük Seri Kuralı**:
  1. **`%100` ve üzeri**: Günlük Seri **`+1` artar**.
  2. **`%80 – %99` (Koruma Eşiği)**: Seri artmaz ancak **sıfırlanmaz** (mevcut değerinde korunur ve haftalık mola hakkını harcamaz).
  3. **`<%80` (Haftalık Mola Hakkı)**: Pazartesi–Pazar haftası içinde **ilk kez** `<%80` altında kalındığında **Haftalık Mola Hakkı** otomatik devreye girer ve seri korunur. Aynı hafta içinde ikinci kez `<%80` altında kalınırsa seri `0`'a sıfırlanır.
- **Kayıt Silme ve "Dün" Telafisi**: Kullanıcı hem **Bugün** hem de **Dün** sekmesindeki kayıtları silebilir veya Dün'e unuttuğu içeceği ekleyebilir. Dün'de yapılan değişiklik dünün tamamlanma durumunu ve seriyi anında yeniden hesaplar.

---

## 3. Arayüz Tasarımı ve Ekran Akışları (Varyant A)

1. **Ana Ekran (Canlı Su Haznesi + Alt 3'lü Bar)**:
   - **Üst Bar**: Sol tarafta **`Bugün | Dün`** sekme anahtarı, sağ tarafta **`🔥 X Gün`** seri rozeti ve Takvim/Ayarlar ikonları.
   - **Merkez (Canlı Su Haznesi)**: İçtikçe dalga animasyonuyla yükselen büyük su haznesi (mavi katman: Saf Su, kehribar katman: Çay/Kahve/Ayran). Ortadaki kartta büyük puntolarla **`%XX`** ve **"Hedefe X Su Bardağı kaldı"** yazar.
   - **Kota Bilgi Şeridi**: Haznenin altında sade dilde **İçilen Saf Su (X Bardak)** ve **Çay/Kahve/Ayran Hakkı (%XX)** kutuları yer alır. %30 kota dolduğunda nazik bilgi bandı gösterilir: *"Bugünkü çay/kahve hakkın doldu! Kalan hedefini tamamlamak için Su içmelisin 💧"*.
   - **Bugün İçtiklerim Listesi**: O gün eklenen içeceklerin saat ve bardak adıyla listesi; yanında yanlış basmaları geri almak için **`Sil`** butonu.
   - **Alt Sabit 3'lü Hızlı Bar (Thumb Zone)**:
     1. **`🥛 +1 Su Bardağı`** (Tek dokunuşla anında `water_glass` ekler)
     2. **`💧 +1 Yudum`** (Tek dokunuşla anında `water_sip` ekler)
     3. **`🫖☕ Diğer`** (Dokununca **Hızlı Seçim Çekmecesi** açılır)
2. **Hızlı Seçim Çekmecesi (Bottom Sheet)**:
   - Kategorilere ayrılmış büyük, ikonlu, tek dokunuşluk kartlar:
     - **Çay**: `1 İnce Belli Çay`, `1 Ajda Çay`, `1 Kupa Çay`
     - **Kahve**: `1 Fincan Türk Kahvesi`, `1 Kupa Filtre Kahve`
     - **Ayran**: `1 Küçük Bardak Ayran`, `1 Büyük Bardak Ayran`
     - **Su**: `1 Su Bardağı`, `1 Yudum Su`, `1 Kupa Su`, `1 Şişe / Matara Su`
   - Kartlardan birine dokunulduğunda kayıt anında eklenir, çekmece kapanır ve su haznesi yükselir.
3. **Takvim (Geçmiş) Görünümü**:
   - Haftalık/aylık tamamlama halkaları ve seçilen gün için **bardak diliyle özet** (örn. *"🥛 6× Su Bardağı · 🫖 3× İnce Belli Çay · ☕ 1× Fincan Türk Kahvesi"*).
4. **Ultra Sade Ayarlar Ekranı**:
   - Yalnızca **Günlük Hedef** (`[- 50 ml +]` ve Kilo/Yaş/Hareketlilik hesaplayıcı) ile **Hatırlatma Planı** (`Aktif Saat Aralığı: 09:00–22:00` ve `Sıklık: 1 / 1.5 / 2 / 3 saat`) yer alır.

---

## 4. Bildirim Stratejisi ve Samsung One UI Mimarisi (`suu-notifications`)

Resmi `tauri-plugin-notification` eklentisi Android'de aksiyon butonlarında uygulamayı zorla öne açtığı ve yeniden başlatmada alarmları kaybettiği için, proje içinde özel Kotlin Tauri eklentisi (**`suu-notifications`**) kullanılır:

### 4.1. Zamanlayıcı ve Kayıt Sıfırlama Kuralları
- **Aktif Saat Aralığı**: Varsayılan `09:00 – 22:00`. Gece saatlerinde bildirim gönderilmez.
- **Kayıt Sıfırlamalı Sıklık**: Varsayılan `2 saat` (`120 dk`). Kullanıcı uygulama içinden veya bildirimden herhangi bir içecek kaydettiğinde, bir sonraki alarm otomatik olarak `şimdi + sıklık` zamanına ötelenir (eğer bu zaman `22:00` sonrasına taşarsa ertesi sabah `09:00`'a kurulur).
- **Hedef %100 Olduğunda Susturma**: Günlük ilerleme `%100`'e ulaştığında o günün kalan alarmları iptal edilir ve alarm doğrudan ertesi sabah `09:00`'a kurulur.

### 4.2. Bildirim Görünümü ve 3 Aksiyon Butonu
- **Kanal Yapılandırması**: `IMPORTANCE_HIGH` + `NotificationCompat.BigTextStyle()` (Samsung One UI'da butonların daraltılmadan doğrudan görünmesini sağlar).
- **Normal Durumda (%30 Kota Dolmamışken) 3 Buton**:
  1. **`+Su (1 Su Bardağı)`** → `PendingIntent.getBroadcast` (`BeverageActionReceiver`, `portion_id = "water_glass"`, `.setShowsUserInterface(false)`). Uygulamayı **açmadan** arka planda kaydeder ve bildirimi kapatır.
  2. **`+Su (1 Yudum)`** → `PendingIntent.getBroadcast` (`BeverageActionReceiver`, `portion_id = "water_sip"`, `.setShowsUserInterface(false)`). Uygulamayı **açmadan** arka planda kaydeder ve bildirimi kapatır.
  3. **`Diğer`** → `PendingIntent.getActivity` (`MainActivity`, `Intent.ACTION_VIEW` + `extra: OPEN_QUICK_DRAWER = true`). Uygulamayı doğrudan **Hızlı Seçim Çekmecesi** açık halde başlatır.
- **%30 Diğer İçecek Kotası Dolmuşken Bildirim**:
  - Başlık: *"Çay/kahve hakkın doldu! Saf su zamanı 💧"*
  - Gövde: *"Hedefin için kalan X su bardağını saf su ile tamamlamalısın."*
  - Butonlar: Yalnızca **`+Su (1 Su Bardağı)`** ve **`+Su (1 Yudum)`** gösterilir.

### 4.3. Samsung One UI Uyum ve İzin Paketi (`AndroidManifest.xml`)
- `android.permission.POST_NOTIFICATIONS` (İlk kurulum sonunda istenir)
- `android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` (İlk kurulumda `Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` ile Samsung "Derin Uyku / Deep Sleep" muafiyeti tek dokunuşla alınır)
- `android.permission.USE_EXACT_ALARM` (`AlarmManager.setExactAndAllowWhileIdle` ile Doze modunda dakik zamanlama)
- `android.permission.RECEIVE_BOOT_COMPLETED` (`BootCompletedReceiver` ile telefon yeniden başladığında sıradaki alarmı `suu.db` üzerinden yeniden kurar)

---

## 5. Yerel Veritabanı Şeması (`suu.db` — SQLite WAL)

Hem Tauri Rust (`rusqlite`) hem de Android Kotlin (`android.database.sqlite.SQLiteDatabase`) `<filesDir>/suu.db` dosyasına `PRAGMA journal_mode = WAL; PRAGMA busy_timeout = 5000;` ile bağlanır.

```sql
-- 1. Kullanıcı Profili ve Hatırlatma Ayarları (Tek satır: id = 1)
CREATE TABLE IF NOT EXISTS user_settings (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  onboarding_completed INTEGER NOT NULL DEFAULT 0,
  weight_kg INTEGER NOT NULL DEFAULT 65,
  age_bracket TEXT NOT NULL DEFAULT '30_55',     -- 'under_30' | '30_55' | 'over_55'
  activity_level TEXT NOT NULL DEFAULT 'low',    -- 'low' | 'medium' | 'high'
  daily_goal_ml INTEGER NOT NULL DEFAULT 2000,   -- 1200..4500 (50 ml adım)
  active_start_min INTEGER NOT NULL DEFAULT 540, -- 09:00 (dakika cinsinden: 9*60)
  active_end_min INTEGER NOT NULL DEFAULT 1320,  -- 22:00 (22*60)
  interval_min INTEGER NOT NULL DEFAULT 120,     -- 60 | 90 | 120 | 180
  next_alarm_epoch_ms INTEGER,                   -- Kurulu bir sonraki alarm zamanı
  updated_at_ms INTEGER NOT NULL
);

-- 2. Tüketim Kayıtları
CREATE TABLE IF NOT EXISTS intake_logs (
  id TEXT PRIMARY KEY,                           -- UUID v4
  tracking_date TEXT NOT NULL,                   -- 'YYYY-MM-DD' (04:00 gün dönümüne göre)
  logged_at_ms INTEGER NOT NULL,                 -- Unix epoch ms
  category TEXT NOT NULL,                        -- 'water' | 'tea' | 'coffee' | 'ayran'
  portion_id TEXT NOT NULL,                      -- 'water_glass', 'water_sip', 'tea_ince' vb.
  gross_ml INTEGER NOT NULL,
  effective_ml INTEGER NOT NULL,
  source TEXT NOT NULL DEFAULT 'app'             -- 'app' | 'notification'
);
CREATE INDEX IF NOT EXISTS idx_intake_tracking_date ON intake_logs(tracking_date);

-- 3. Günlük Mühürlü Hedef Snapshot'ı (Geçmiş günlerin hedefi değişmesin diye)
CREATE TABLE IF NOT EXISTS daily_goals (
  tracking_date TEXT PRIMARY KEY,                -- 'YYYY-MM-DD'
  goal_ml INTEGER NOT NULL
);
```

---

## 6. Katmanlar Arası Arayüz (Tauri Komutları ve Olayları)

### 6.1. Rust (`src-tauri`) → Svelte 5 Komutları
- `get_dashboard_state(tracking_date?: string) -> DashboardState`: Seçili günün (`today` veya `yesterday`) kayıtlarını, saf su / diğer içecek toplamlarını, kalan su bardağı sayısını, aktif **Günlük Seri** ve **Haftalık Mola Hakkı** durumunu döner.
- `log_intake(portion_id: string, tracking_date?: string) -> DashboardState`: Kaydı `intake_logs` tablosuna yazar, `daily_goals` kaydını günceller ve Kotlin `suu-notifications` eklentisini tetikleyerek bir sonraki hatırlatma alarmını yeniden planlar.
- `delete_intake(log_id: string) -> DashboardState`: Bugün veya Dün'e ait kaydı siler ve durumu yeniden hesaplar.
- `update_settings(payload: UserSettingsInput) -> DashboardState`: Hedef veya hatırlatma ayarlarını günceller, bugünün `daily_goals` kaydını yeniler ve alarmı yeniden kurar.
- `get_calendar_history(year_month: string) -> MonthHistory`: Seçili ayın günlerini tamamlanma yüzdeleri ve bardak dili özetleriyle döner.

### 6.2. Kotlin (`suu-notifications`) ↔ Rust / Svelte 5 Köprüsü
- **Kotlin Komutları**:
  - `rescheduleNextReminder()`: `suu.db` içindeki son kaydı, bugünkü ilerlemeyi ve `user_settings` tablosunu okuyarak bir sonraki `AlarmManager.setExactAndAllowWhileIdle` alarmını kurar veya hedef `%100` ise ertesi sabaha öteler.
  - `requestOneUiPermissions()`: `POST_NOTIFICATIONS` ve `ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` izin akışını başlatır.
  - `consumeLaunchIntent() -> { openQuickDrawer: boolean }`: Bildirimdeki **`Diğer`** butonundan açıldıysa `true` döner; Svelte 5 açılışta çekmeceyi otomatik açar.
- **WebView Olayları**:
  - Uygulama açık/arka plandayken bildirimden kayıt yapıldığında `BeverageActionReceiver` → `Plugin.trigger("intakeUpdated", ...)` gönderir; ayrıca Svelte 5 `document.addEventListener("visibilitychange", ...)` ile her ön yüze gelişte `get_dashboard_state()` çağırır.
