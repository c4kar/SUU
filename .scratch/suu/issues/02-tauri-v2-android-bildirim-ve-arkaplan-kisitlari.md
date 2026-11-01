# Tauri v2 Android Bildirim Aksiyonları ve Arka Plan Zamanlayıcı Araştırması

Type: research
Status: resolved
Blocked by: none

## Question

Tauri v2 Android uygulamasında:
1. Resmi `tauri-plugin-notification` eklentisi bildirim üzerinde etkileşimli aksiyon butonlarını (`ActionType` / Notification Actions, örn. "+1 Bardak Su", "+1 Çay") ve uygulama kapalıyken/arka plandayken bu aksiyonların tetiklenip yerel veriye kayıt yazmasını destekliyor mu?
2. Uygulama kullanıcı tarafından kapatıldığında (swipe-away) veya Android Doze / pil optimizasyonu devreye girdiğinde gün içi düzenli hatırlatma bildirimlerinin güvenilir biçimde gösterilmesi için `tauri-plugin-notification` (`Schedule`) yeterli mi, yoksa Kotlin ile özel bir Tauri Android Plugin (`AlarmManager` / `WorkManager` + `BroadcastReceiver`) mi gerekiyor?

## Answer

[Detaylı Araştırma Raporu](../research/02-tauri-v2-android-bildirim-ve-arkaplan-kisitlari.md)

### 1. Bildirim Aksiyon Butonları (`registerActionTypes` / `onAction`) Kısıtları
- **Arka Planda Sessiz Çalışmama (`PendingIntent.getActivity`):** Resmi `tauri-plugin-notification` (v2.3.3), `Action` üzerindeki `foreground: false` seçeneğini **yalnızca iOS'ta** destekler; Android'de (`TauriNotificationManager.kt` satır 541–607) tüm aksiyon butonlarını `PendingIntent.getActivity(...)` ve `Intent.ACTION_MAIN` ile doğrudan `MainActivity`'ye bağlar. Dolayısıyla kullanıcı bildirimden **"+1 Bardak Su"** veya **"+1 Çay"** butonuna bastığında **uygulama her seferinde zorunlu olarak ekrana (foreground) açılır**.
- **Uygulama Kapalıyken (Cold Start) Aksiyon Kaybı (`#3296`):** Uygulama son kullanılanlardan kapatılmışken (swipe-away / killed) bildirim aksiyonuna basıldığında, `NotificationPlugin.load(webView)` içindeki `onIntent` çağrısı WebView JS katmanı (`onAction` dinleyicisi) henüz yüklenmeden `Plugin.trigger("actionPerformed", ...)` metodunu tetikler. `Plugin.kt` olayları kuyruklamadığı için **tıklanan aksiyon olayı tamamen kaybolur** (bkz. `tauri-apps/plugins-workspace#3296`).
- **İzin İsteme Takılması (`#3757`):** Android 13+ (API 33+) üzerinde `POST_NOTIFICATIONS` izni zaten verilmişken `requestPermissions` çağrıldığında `else` dalı eksik olduğu için Promise asla sonlanmaz (`tauri-apps/plugins-workspace#3757`).

### 2. Arka Plan Zamanlayıcı (`Schedule`), Doze Modu ve Reboot Güvenilirliği
- **Reboot Sonrası Tüm Zamanlayıcıların Silinmesi (`#3550`, `#3751`):**
  1. `sendNotification` (`show` komutu) zamanlanan bildirimleri `NotificationStorage` (`SharedPreferences`) içine **hiç kaydetmez** (`#3751`).
  2. `batch` komutu kullanılsa bile `Notification.sourceJson` alanı Jackson deserialization sırasında `null` kaldığı için `SharedPreferences` içine literal `"null"` string'i yazılır (`#3550`). Cihaz yeniden başlatıldığında (`BOOT_COMPLETED` -> `LocalNotificationRestoreReceiver`) **hiçbir alarm geri yüklenemez**, ayrıca `pending()` ve `cancelAll()` çağrıları hata verir.
- **Doze Modu Eksiklikleri:** `Schedule.every` metodu `allowWhileIdle` parametresini tamamen yok sayarak `alarmManager.setRepeating(AlarmManager.RTC, ...)` kullanır (cihazı Doze modunda uyandırmaz). `Schedule.interval` ise ilk tetiklemeden sonraki yeniden kurulumlarda (`TimedNotificationPublisher.rescheduleNotificationIfNeeded`) `allowWhileIdle` ve `RTC_WAKEUP` bayraklarını kaybeder.

### 3. Mimari Karar ve Önerilen Tasarım
- **Karar:** SUU'nun sıfır sürtünmeli içecek kaydı ve güvenilir gündüz hatırlatmaları için resmi `tauri-plugin-notification` **yetersizdir**; proje içinde **hafif ve özel bir Kotlin Tauri Mobil Eklentisi (`suu-notifications`, ~250–300 satır Kotlin)** yazılmalıdır.
- **Önerilen Mimari (`BroadcastReceiver` + `AlarmManager.setAndAllowWhileIdle`):**
  1. **Sıfır Sürtünmeli Hızlı Eylem Butonları:** Bildirim gövdesi `PendingIntent.getActivity` ile uygulamayı açarken, altındaki hızlı içecek butonları (`+200ml Su`, `+1 Çay` vb.) `PendingIntent.getBroadcast` ve `.setShowsUserInterface(false)` ile `BeverageActionReceiver`'ı tetikler (Activity başlatmadığı için Android 12+ Notification Trampoline kısıtlamalarına %100 uyumludur).
  2. **Arka Planda Kayıt ve Senkronizasyon:** `BeverageActionReceiver` uygulama kapalıyken bile WebView'ı açmadan kaydı atomik olarak yerel kuyruğa (`SharedPreferences` / JSONL) veya doğrudan paylaşımlı SQLite veritabanına yazar, bildirimi kapatır/günceller, bir sonraki hatırlatma alarmını `now + interval` olarak öteler ve uygulama açıksa `trigger("quickIntakeLogged", ...)` ile arayüzü canlı günceller.
  3. **İzin Sürtünmesiz Doze Uyumlu Zamanlayıcı:** Google Play'in kısıtladığı `USE_EXACT_ALARM` veya kullanıcının ayarlara gitmesini gerektiren `SCHEDULE_EXACT_ALARM` yerine, ekstra izin gerektirmeyen `AlarmManager.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, ...)` + `BOOT_COMPLETED` alıcısı kullanılarak gündüz saatleri penceresi (örn. 09:00–23:00) içinde güvenilir hatırlatma sağlanır.
