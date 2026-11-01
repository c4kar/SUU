# Tauri v2 Android Bildirim Aksiyonları ve Arka Plan Zamanlayıcı Araştırması

**Tarih:** 2026-11-01  
**İlgili Bilet:** [`issues/02-tauri-v2-android-bildirim-ve-arkaplan-kisitlari.md`](../issues/02-tauri-v2-android-bildirim-ve-arkaplan-kisitlari.md)  
**Birincil Kaynaklar:**
- [`tauri-apps/plugins-workspace` (`v2` dalı) — `plugins/notification` kaynak kodu](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/notification)
- [`tauri-apps/tauri` (`dev`/`v2` dalı) — `Plugin.kt` Android eklenti köprüsü](https://github.com/tauri-apps/tauri/blob/dev/crates/tauri/mobile/android/src/main/java/app/tauri/plugin/Plugin.kt)
- [Tauri v2 Resmi Dokümantasyonu — Notification Plugin](https://v2.tauri.app/plugin/notification/) & [Mobile Plugin Development](https://v2.tauri.app/develop/plugins/develop-mobile/)
- [Android Developers — Notification Actions, Trampoline Restrictions & Schedule Exact Alarms](https://developer.android.com/develop/ui/views/notifications/build-notification#Actions)

---

## Yönetici Özeti (TL;DR)

1. **Bildirim Aksiyon Butonları (`registerActionTypes` / `onAction`):**
   - Resmi `tauri-plugin-notification` (v2.3.3) eklentisi Android üzerinde bildirim aksiyon butonlarını (`NotificationCompat.Action`) **destekler**, ancak **arka planda sessiz çalıştırmayı (`foreground: false`) desteklemez**.
   - `TauriNotificationManager.kt` içerisindeki `createActionIntents` metodu her aksiyon butonunu `PendingIntent.getActivity(...)` ve `Intent.ACTION_MAIN` ile doğrudan `MainActivity`'ye bağlar. Dolayısıyla kullanıcı bildirimden **"+1 Bardak Su"** veya **"+1 Çay"** butonuna bastığında **uygulama zorunlu olarak ön plana (foreground) açılır**.
   - Daha da kritik olarak, uygulama tamamen kapalıyken (swipe-away / cold start) aksiyon butonuna basıldığında `NotificationPlugin.load(webView)` metodu, WebView içindeki JS `onAction` dinleyicisi henüz bağlanmadan `trigger("actionPerformed", ...)` çağrısını yapar ve `Plugin.kt` olayları kuyruklamadığı için **tıklanan aksiyon olayı sessizce kaybolur** (bkz. açık PR [`#3296`](https://github.com/tauri-apps/plugins-workspace/pull/3296)).

2. **Arka Plan Zamanlayıcı (`Schedule`) ve Yeniden Başlatma (`BOOT_COMPLETED`) Güvenilirliği:**
   - Eklenti Android'de `AlarmManager` ve `TimedNotificationPublisher` (`BroadcastReceiver`) kullanır; ancak v2.3.3 sürümünde **üretim ortamında hatırlatıcıları bozan 4 yapısal hata** mevcuttur:
     1. `sendNotification` / `show` komutu zamanlanan bildirimleri `NotificationStorage` (`SharedPreferences`) içine **hiç kaydetmez** (sadece JS API'de dışa aktarılmayan `batch` komutu kaydeder — bkz. [`#3751`](https://github.com/tauri-apps/plugins-workspace/pull/3751)).
     2. `batch` kullanılsa dahi `Notification.sourceJson` alanı Jackson deserialization sırasında doldurulmadığından `SharedPreferences` içine literal `"null"` string'i yazılır; cihaz yeniden başlatıldığında (`LocalNotificationRestoreReceiver`) **tüm zamanlanmış bildirimler silinir/kaybolur** ve `pending()` / `cancelAll()` çağrıları hata fırlatır (bkz. [`#3550`](https://github.com/tauri-apps/plugins-workspace/issues/3550), [`#3728`](https://github.com/tauri-apps/plugins-workspace/pull/3728)).
     3. `Schedule.every` (periyodik tekrar), `allowWhileIdle` parametresini tamamen yok sayarak `alarmManager.setRepeating(AlarmManager.RTC, ...)` çağırır (`RTC_WAKEUP` bile değildir); Android Doze modunda cihazı uyandırmaz.
     4. `Schedule.interval` (`DateMatch`), ilk tetiklemeden sonraki yeniden kurulumlarda (`TimedNotificationPublisher.rescheduleNotificationIfNeeded`) `allowWhileIdle` ve `RTC_WAKEUP` bayraklarını kaybeder.

3. **Mimari Karar (Önerilen Çözüm):**
   - SUU'nun temel değer önerisi olan **"bildirimden tek dokunuşla sıfır sürtünmeli içecek kaydı"** ve **"gün içi güvenilir hatırlatma"** için `tauri-plugin-notification` tek başına **yetersizdir**.
   - Bunun yerine proje içinde **hafif, özel bir Kotlin Tauri Mobil Eklentisi (`SuuNotificationPlugin` + `BeverageActionReceiver` + `ReminderSchedulerReceiver`)** yazılmalıdır (~250–300 satır Kotlin). Bu mimaride aksiyon butonları `PendingIntent.getBroadcast()` kullanır; uygulama kapalıyken bile Activity/WebView açılmadan arka planda kaydı yazar, bildirimi günceller/kapatır ve bir sonraki hatırlatmayı Doze-uyumlu (`setAndAllowWhileIdle`) olarak kurar.

---

## Bölüm 1: `tauri-plugin-notification` v2 Android Aksiyon Butonları Analizi

### 1.1. Desteklenen `Action` Alanları ve `foreground: false` Kısıtı

TypeScript API tanımında (`guest-js/index.ts`, satır 391–430) ve Rust mobil köprüsünde (`src/mobile.rs`, satır 1480–1483) açıkça belirtildiği üzere:

> *"On Android only the identifier, the title and the input flag are used."*
> `foreground?: boolean` — *"Whether the app is brought to the foreground when the action is triggered. **iOS only.**"*

Android tarafındaki veri modeli (`plugins/notification/android/src/main/java/NotificationPlugin.kt`, satır 50–55):

```kotlin
@InvokeArg
class NotificationAction {
  lateinit var id: String
  var title: String? = null
  var input: Boolean? = null
}
```

Görüldüğü üzere `foreground` alanı Android tarafında parse bile edilmemektedir.

### 1.2. Aksiyon Butonuna Tıklandığında Ne Olur? (`PendingIntent.getActivity` Zorunluluğu)

`TauriNotificationManager.kt` (satır 528–607) incelendiğinde, hem bildirimin kendisi (`DEFAULT_PRESS_ACTION = "tap"`) hem de `registerActionTypes` ile kaydedilen tüm aksiyon butonları aynı `buildIntent` metodunu ve `PendingIntent.getActivity` çağrısını kullanır:

```kotlin
// TauriNotificationManager.kt (satır 541-558)
val actionTypeId = notification.actionTypeId
if (actionTypeId != null) {
  val actionGroup = storage.getActionGroup(actionTypeId)
  for (notificationAction in actionGroup) {
    val actionIntent = buildIntent(notification, notificationAction!!.id)
    val actionPendingIntent = PendingIntent.getActivity(
      context,
      (notification.id) + notificationAction.id.hashCode(),
      actionIntent,
      flags
    )
    val actionBuilder: NotificationCompat.Action.Builder = NotificationCompat.Action.Builder(
      R.drawable.ic_transparent,
      notificationAction.title,
      actionPendingIntent
    )
    ...
    mBuilder.addAction(actionBuilder.build())
  }
}
```

Ve `buildIntent` metodu (`TauriNotificationManager.kt`, satır 591–607):

```kotlin
private fun buildIntent(notification: Notification, action: String?): Intent {
  val intent = if (activity != null) {
    Intent(context, activity.javaClass)
  } else {
    val packageName = context.packageName
    context.packageManager.getLaunchIntentForPackage(packageName)!!
  }
  intent.action = Intent.ACTION_MAIN
  intent.addCategory(Intent.CATEGORY_LAUNCHER)
  intent.flags = Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP
  intent.putExtra(NOTIFICATION_INTENT_KEY, notification.id)
  intent.putExtra(ACTION_INTENT_KEY, action)
  intent.putExtra(NOTIFICATION_OBJ_INTENT_KEY, notification.sourceJson)
  ...
  return intent
}
```

**Sonuç:**
- Kullanıcı bildirim çekmecesinden **"+200 ml Su"** butonuna bastığı anda Android işletim sistemi `MainActivity`'yi başlatır ve uygulamayı ekrana getirir.
- Kullanıcı başka bir uygulamada çalışırken veya kilit ekranındayken uygulamayı açmadan "tek tıkla su ekleme" deneyimi `tauri-plugin-notification` ile **mümkün değildir**.

### 1.3. Uygulama Kapalıyken (Cold Start / Swipe-Away) Aksiyon Kaybı Hatası

Kullanıcı uygulamayı son kullanılanlardan kaydırarak kapattığında (killed state) ve daha sonra bildirimdeki bir aksiyon butonuna bastığında şu yaşam döngüsü çalışır:

1. Android, `PendingIntent.getActivity` ile `MainActivity`'yi sıfırdan başlatır (Cold Start).
2. Tauri, `NotificationPlugin.load(webView)` metodunu çağırır (`NotificationPlugin.kt`, satır 99–137):
   ```kotlin
   override fun load(webView: WebView) {
     ...
     val intent = activity.intent
     intent?.let {
       onIntent(it)
     }
   }

   fun onIntent(intent: Intent) {
     if (Intent.ACTION_MAIN != intent.action) {
       return
     }
     val dataJson = manager.handleNotificationActionPerformed(intent, notificationStorage)
     if (dataJson != null) {
       trigger("actionPerformed", dataJson)
     }
   }
   ```
3. `load(webView)` çalıştığı anda WebView henüz oluşturulmuştur; HTML/JS paketi yüklenmemiş ve frontend'deki `await onAction(...)` (`registerListener`) çağrısı henüz gerçekleşmemiştir.
4. Tauri'nin temel `Plugin.kt` sınıfındaki `trigger` metodu (`crates/tauri/mobile/android/src/main/java/app/tauri/plugin/Plugin.kt`, satır 212–220):
   ```kotlin
   fun trigger(event: String, payload: JSObject) {
     val eventListeners = listeners[event]
     if (!eventListeners.isNullOrEmpty()) {
       val listeners = CopyOnWriteArrayList(eventListeners)
       for (channel in listeners) {
         channel.send(payload)
       }
     }
   }
   ```
   `listeners["actionPerformed"]` boş olduğu için olay **hiçbir yere kaydedilmeden çöpe atılır**, üstelik `handleNotificationActionPerformed` bildirimi çoktan sildiği için telafisi de olmaz.
   *(Not: Bu sorun Tauri deposunda açık olan [PR #3296 — Handle Cold-Start Notification Actions and Delivery Reliability on Android](https://github.com/tauri-apps/plugins-workspace/pull/3296) ile doğrulanmıştır ve v2.3.3 itibarıyla henüz ana dala birleştirilmemiştir).*

### 1.4. Android 13+ İzin İsteme Takılma Hatası (`#3757`)

`NotificationPlugin.kt` (satır 258–267) içerisinde Android 13+ (`TIRAMISU`) cihazlarda `POST_NOTIFICATIONS` izni zaten verilmişken `requestPermission()` çağrıldığında `else` bloğu unutulduğu için `invoke` asla sonlandırılmaz (`resolve` edilmez) ve JS Promise sonsuza dek asılı kalır ([Issue #3757](https://github.com/tauri-apps/plugins-workspace/issues/3757)).

---

## Bölüm 2: Arka Plan Zamanlayıcı (`Schedule`), Doze Modu ve Reboot Davranışı

### 2.1. `tauri-plugin-notification` `Schedule` Türlerinin Android İmplementasyonu

`TauriNotificationManager.kt` (satır 615–686) içerisinde üç farklı zamanlama modu `AlarmManager` üzerinden şu şekilde uygulanmıştır:

| Zamanlama Türü | Android `AlarmManager` Çağrısı | `allowWhileIdle` Desteği | Doze Modu ve Güvenilirlik Durumu |
| :--- | :--- | :--- | :--- |
| **`Schedule.at(date, false, allowWhileIdle)`** | `setExactIfPossible(...)` | **Var** (`setExactAndAllowWhileIdle` veya `setAndAllowWhileIdle` + `RTC_WAKEUP`) | Tek seferlik bildirim için çalışır; ancak `SCHEDULE_EXACT_ALARM` yoksa sessizce inexact alarma düşer. |
| **`Schedule.at(date, true, ...)`** | `alarmManager.setRepeating(AlarmManager.RTC, ...)` | **Yok** (Yok sayılır) | `RTC` (cihazı uyandırmaz) ve API 19+ üzerinde inexact çalışır; Doze modunda ertelenir. |
| **`Schedule.every(interval, count, allowWhileIdle)`** | `alarmManager.setRepeating(AlarmManager.RTC, startTime, everyInterval, pendingIntent)` | **Yok** (`TauriNotificationManager.kt` satır 657–661'de `allowWhileIdle` hiç okunmaz) | `AlarmManager.RTC` kullanıldığı için ekran kapalı/Doze modundayken cihazı uyandırmaz, gece/gündüz penceresi ayırt edemez. |
| **`Schedule.interval(dateMatch, allowWhileIdle)`** | İlk tetiklemede `setExactIfPossible(...)`, sonraki tetiklemelerde `TimedNotificationPublisher.rescheduleNotificationIfNeeded` | **Sadece ilk tetiklemede var!** | İkinci ve sonraki tetiklemelerde (`satır 825–829`) `allowWhileIdle` ve `RTC_WAKEUP` unutulmuş olup doğrudan `AlarmManager.RTC` kullanılır. |

İlgili kaynak kod (`TauriNotificationManager.kt`, satır 825–829 — `TimedNotificationPublisher.rescheduleNotificationIfNeeded`):

```kotlin
if (SDK_INT >= Build.VERSION_CODES.S && !alarmManager.canScheduleExactAlarms()) {
  alarmManager[AlarmManager.RTC, trigger] = pendingIntent
} else {
  alarmManager.setExact(AlarmManager.RTC, trigger, pendingIntent)
}
```

### 2.2. Uygulama Kapatıldığında (Swipe-Away) ve Yeniden Başlatıldığında (`BOOT_COMPLETED`) Ne Olur?

1. **Swipe-Away (Son Uygulamalardan Kaydırarak Kapatma):**
   - `triggerScheduledNotification` metodu, oluşturulmuş `android.app.Notification` nesnesini `Parcelable` olarak doğrudan `PendingIntent` (`TimedNotificationPublisher`) içine gömer (`notificationIntent.putExtra(TimedNotificationPublisher.NOTIFICATION_KEY, notification)`).
   - Standart Android (AOSP, Pixel, Samsung OneUI) cihazlarda uygulama son kullanılanlardan kaydırılarak kapatılsa bile `AlarmManager`'daki `PendingIntent` işletim sistemi seviyesinde yaşar ve zamanı geldiğinde `TimedNotificationPublisher` (`BroadcastReceiver`) süreci uyandırıp bildirimi gösterir.
   - *(İstisna: Kullanıcı "Ayarlar -> Uygulamalar -> Zorla Durdur (Force Stop)" yaparsa veya Xiaomi/Huawei/Oppo gibi agresif OEM pil kısıtlayıcıları uygulamayı "Force Stop" durumuna sokarsa tüm alarmlar iptal edilir).*

2. **Cihaz Yeniden Başlatıldığında (`BOOT_COMPLETED`) — Kritik Veri Kaybı (`#3550` & `#3751`):**
   - Android'de `AlarmManager` kayıtları cihaz yeniden başlatıldığında işletim sistemi tarafından temizlenir.
   - Eklentinin `AndroidManifest.xml` dosyasında `RECEIVE_BOOT_COMPLETED` izni ve `LocalNotificationRestoreReceiver` tanımlıdır.
   - **Ancak iki kritik hata nedeniyle reboot sonrası geri yükleme tamamen bozuktur:**
     - **Hata 1 ([PR #3751](https://github.com/tauri-apps/plugins-workspace/pull/3751)):** `NotificationPlugin.kt` içindeki `@Command fun show(invoke: Invoke)` metodu (satır 139–145), `notificationStorage.appendNotifications(...)` metodunu **çağırmaz**. Dolayısıyla `sendNotification({ schedule: ... })` ile kurulan hiçbir bildirim diske kaydedilmez.
     - **Hata 2 ([Issue #3550](https://github.com/tauri-apps/plugins-workspace/issues/3550)):** `NotificationStorage.appendNotifications` (satır 1380) `editor.putString(key, request.sourceJson.toString())` yazar. Oysa `request.sourceJson` alanı hiçbir yerde atanmadığı için her zaman `null`'dır ve `SharedPreferences` XML dosyasına `"null"` metni yazılır. Cihaz yeniden başladığında `LocalNotificationRestoreReceiver` hiçbir bildirimi okuyamaz ve tüm hatırlatıcılar sessizce kaybolur.

### 2.3. Android İzin Gereksinimleri (`POST_NOTIFICATIONS`, `SCHEDULE_EXACT_ALARM`, `USE_EXACT_ALARM`)

`tauri-plugin-notification` eklentisinin `AndroidManifest.xml` dosyası şu izinleri içerir:
```xml
<uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED" />
<uses-permission android:name="android.permission.WAKE_LOCK" />
<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
```

- **`POST_NOTIFICATIONS` (Android 13 / API 33+):** Çalışma zamanında (runtime) kullanıcıdan onay alınması zorunludur.
- **`USE_EXACT_ALARM` (Android 13 / API 33+):** Google Play politikaları gereği **yalnızca** ana işlevi çalar saat, zamanlayıcı veya takvim olan uygulamalara izin verilir. Bir su/hidrasyon takip uygulaması Play Store'da `USE_EXACT_ALARM` kullanırsa **onay sürecinde reddedilir** (uygulama mağaza dışı / APK olarak dağıtılsa bile Android 14+ üzerinde `SCHEDULE_EXACT_ALARM` varsayılan olarak kapalı gelir).
- **`SCHEDULE_EXACT_ALARM` (Android 12 / API 31+):** Android 14+ (API 34+) itibarıyla yeni kurulan uygulamalarda varsayılan olarak **reddedilmiş (`denied`)** başlar; kullanıcının sistem ayarlarındaki özel "Alarmlar ve Hatırlatıcılar" ekranına yönlendirilip elle açması gerekir (`ACTION_REQUEST_SCHEDULE_EXACT_ALARM`).
- **Hidrasyon Uygulaması İçin Doğru Alarm Stratejisi:**
  - Su hatırlatıcıları saniyesi saniyesine çalar saat hassasiyeti gerektirmez; ±5–10 dakikalık esneklik (Doze bakım penceresi) tamamen doğaldır ve pil dostudur.
  - Bu nedenle `SCHEDULE_EXACT_ALARM` izni istemeden **`AlarmManager.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, triggerAtMillis, pendingIntent)`** (veya `WorkManager` periyodik işçisi) kullanmak hem sıfır izin sürtünmesi sağlar hem de Doze modunda cihazı uyandırarak bildirimi güvenle teslim eder.

---

## Bölüm 3: SUU İçin Mimari Seçenekler ve Önerilen Tasarım

### 3.1. Mimari Seçeneklerin Karşılaştırması

| Kriter | Seçenek A: Resmi `tauri-plugin-notification` | Seçenek B (Önerilen): Özel Kotlin Tauri Plugin (`BroadcastReceiver` + `AlarmManager`) | Seçenek C: Özel Kotlin Tauri Plugin + Şeffaf Hızlı Eylem Overlay (`Activity`) |
| :--- | :--- | :--- | :--- |
| **Bildirimden "+1 Su" / "+1 Çay" Tıklaması** | ❌ Tam `MainActivity` + WebView açılır (yüksek sürtünme) | ✅ `PendingIntent.getBroadcast` ile arka planda **0 ms UI açılmadan** anında kaydedilir | ⚠️ Küçük/şeffaf bir native Activity açılır (özel miktar seçimi için yararlı, tek tık için gereksiz) |
| **Uygulama Kapalıyken (Killed) Aksiyon Kaydı** | ❌ Cold-start race condition (`#3296`) nedeniyle olay kaybolur | ✅ `BroadcastReceiver` WebView'a ihtiyaç duymadan doğrudan yerel depoya yazar | ✅ Native Activity doğrudan yerel depoya yazar |
| **Gündüz Saatleri Penceresi (Örn. 09:00–23:00)** | ❌ Tek tek onlarca `Schedule.interval` kurmak gerekir | ✅ `ReminderSchedulerReceiver` bir sonraki hatırlatma saatini pencereye göre dinamik hesaplar | ✅ Aynı |
| **Akıllı Erteleme (Kullanıcı Az Önce Su İçtiyse)** | ❌ Tüm bekleyen alarmları iptal edip yeniden kurmak gerekir | ✅ Son içim zamanı kontrol edilip bir sonraki alarm otomatik `now + interval` olarak ötelenir | ✅ Aynı |
| **Cihaz Reboot (`BOOT_COMPLETED`) Dayanıklılığı** | ❌ `#3550` ve `#3751` hataları yüzünden tamamen bozuk | ✅ `SharedPreferences` içindeki JSON yapılandırmasından tek satırda yeniden kurulur | ✅ Aynı |
| **Geliştirme Maliyeti** | Düşük (hazır paket) | Orta (~250 satır Kotlin kodu, `src-tauri/gen/android` veya yerel plugin crate) | Orta-Yüksek |

### 3.2. Android 12+ Bildirim Trampolini (Notification Trampoline) Kurallarına Uyum

Android 12 (API 31) ile gelen **Notification Trampoline** kısıtlaması şu durumu yasaklar:
> Bir bildirim veya bildirim aksiyonu `PendingIntent.getBroadcast()` veya `PendingIntent.getService()` tetikledikten sonra, o alıcı/servis içerisinden `context.startActivity()` çağrılarak bir ekran açılması engellenir.

**SUU İçin Anlamı:**
- Bildirimin **gövdesine (body)** tıklandığında uygulamanın ana ekranı açılacağı için gövde `contentIntent`'i doğrudan **`PendingIntent.getActivity(...)`** kullanmalıdır.
- Bildirimin altındaki **hızlı aksiyon butonları** (örn. **`+200ml Su`**, **`+1 Çay`**, **`+1 Kahve`**) ise uygulama ekranını **açmayacağı** (sadece arka planda veriyi kaydedip bildirimi kapatacağı veya güncelleyeceği) için **`PendingIntent.getBroadcast(...)`** ve `NotificationCompat.Action.Builder.setShowsUserInterface(false)` kullanması **Android 12–16 standartlarına %100 uygundur** ve Google'ın resmi olarak önerdiği yöntemdir.

### 3.3. Arka Planda Veri Yazma Deseni: SQLite vs. Paylaşımlı Kuyruk (`SharedPreferences` / JSONL)

Uygulama kapalıyken (WebView ve Rust süreci çalışmıyorken) kullanıcı bildirimdeki **"+200 ml Su"** butonuna bastığında `BeverageActionReceiver` (`BroadcastReceiver`) tetiklenir. Bu noktada verinin kalıcı hale getirilmesi için iki temiz desen vardır:

#### Desen B1: Doğrudan Paylaşımlı SQLite Veritabanına Yazma (`android.database.sqlite.SQLiteDatabase`)
- Tauri uygulaması (Rust `rusqlite` / `tauri-plugin-sql` veya WebView) ile Android `BroadcastReceiver` aynı uygulama sanal alanında (`context.filesDir` veya `context.getDatabasePath("suu.db")`) çalışır.
- SQLite `WAL` (Write-Ahead Logging) modunda açıldığında, Kotlin `BroadcastReceiver` doğrudan `SQLiteDatabase.openDatabase(...)` ile `intake_logs` tablosuna `INSERT` yapabilir.
- Ayrıca bugünün toplam hidrasyon miktarını `SELECT SUM(effective_ml) ...` ile तुरंत hesaplayıp, bildirim metnini veya özetini ("Harika! Bugün 1400 / 2200 ml tamamlandı") anında güncelleyebilir.

#### Desen B2 (Daha Basit ve Şema Bağımsız): Atomik `SharedPreferences` / JSONL Kuyruğu + Canlı Olay Köprüsü
- Eğer tüm iş mantığı (BHI katsayı hesaplaması, günlük hedef, istatistikler) tek bir yerde (Rust veya TypeScript katmanında) tutulmak isteniyorsa:
  1. Frontend/Rust katmanı, kullanıcının seçtiği hızlı içecek butonlarını (örn. `[{ id: "water_200", label: "+200ml Su", beverageId: "water", amountMl: 200, factor: 1.0 }, { id: "tea_150", label: "+1 Çay", beverageId: "tea", amountMl: 150, factor: 0.9 }]`) ve güncel günlük ilerlemeyi (`currentMl`, `goalMl`) Kotlin eklentisine (`SuuNotificationPlugin.configure(...)`) bildirir.
  2. Kullanıcı bildirimden **"+200ml Su"** butonuna bastığında `BeverageActionReceiver`:
     - Kaydı `SharedPreferences` (veya `pending_intakes.jsonl`) içindeki bekleyenler listesine `{ id, beverageId, amountMl, timestampMs }` olarak ekler.
     - `SharedPreferences` içindeki önbelleklenmiş `currentMl` değerini artırır (`currentMl += 200 * 1.0`), bildirimi kapatır (veya kısa bir onay bildirimi / güncellenmiş ilerleme çubuğu gösterir).
     - Bir sonraki hatırlatma alarmını `now + intervalMinutes` olarak yeniden kurar (böylece su içtikten hemen 2 dakika sonra tekrar bildirim gelmez!).
     - Eğer Tauri `MainActivity` o sırada açıksa (`SuuNotificationPlugin.instance != null`), `trigger("quickIntakeLogged", payload)` ile UI'ı anında canlandırır.
     - Uygulama kapalıysa, kullanıcı uygulamayı bir sonraki açışında (`onResume` / başlangıç) `drainPendingIntakes` komutuyla bekleyen kayıtlar ana veritabanına aktarılır.

> **Mimari Tavsiye:** **Desen B2** (veya Rust SQLite kullanılıyorsa B1+B2 hibriti), şema göçü (migration) çakışmalarını sıfıra indirdiği, içecek katsayılarını tek merkezden yönettiği ve "su içildiğinde bir sonraki alarmı otomatik öteleme" mantığını çok temiz çözdüğü için en düşük riskli ve en bakımı kolay yaklaşımdır.

---

## Bölüm 4: Önerilen Özel Kotlin Eklentisi Taslağı (Referans Mimari)

Aşağıdaki yapı, harici bir npm paketi yayınlamaya gerek kalmadan doğrudan `src-tauri/gen/android/app/src/main/java/...` altında veya yerel bir `src-tauri/plugins/suu-notifications` eklentisi olarak uygulanabilir:

```kotlin
// 1. Bildirim Aksiyon Butonları (TauriNotificationManager yerine özel oluşturucu)
val logIntent = Intent(context, BeverageActionReceiver::class.java).apply {
    action = "com.suu.app.ACTION_LOG_BEVERAGE"
    putExtra("beverage_id", actionItem.beverageId)
    putExtra("amount_ml", actionItem.amountMl)
    putExtra("notification_id", notificationId)
}
val actionPendingIntent = PendingIntent.getBroadcast(
    context,
    actionItem.hashCode(),
    logIntent,
    PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
)

val action = NotificationCompat.Action.Builder(
    0,
    actionItem.label, // Örn: "+200ml Su" veya "+1 Çay (150ml)"
    actionPendingIntent
)
    .setShowsUserInterface(false) // Activity açmayacağını Android'e bildirir
    .build()
```

```kotlin
// 2. Doze Modu Uyumlu Hatırlatma Kurucu (SCHEDULE_EXACT_ALARM gerektirmez)
fun scheduleNextReminder(context: Context, triggerAtMillis: Long) {
    val alarmManager = context.getSystemService(Context.ALARM_SERVICE) as AlarmManager
    val intent = Intent(context, ReminderAlarmReceiver::class.java)
    val pendingIntent = PendingIntent.getBroadcast(
        context,
        REMINDER_REQUEST_CODE,
        intent,
        PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
    )
    // Doze modunda bile bakım penceresinde cihazı uyandırır, özel izin gerektirmez
    alarmManager.setAndAllowWhileIdle(
        AlarmManager.RTC_WAKEUP,
        triggerAtMillis,
        pendingIntent
    )
}
```

### Özet Karar Tablosu

1. **`tauri-plugin-notification` kullanılmalı mı?**
   - **Hayır** (en azından Android hatırlatma zamanlayıcısı ve aksiyon butonları için). Mevcut v2.3.3 sürümündeki `PendingIntent.getActivity` zorunluluğu, `#3296` cold-start olay kaybı, `#3550`/`#3751` reboot sonrası alarm silinmesi ve `#3757` izin takılması hataları nedeniyle SUU'nun ana kullanım senaryosunu doğrudan bozmaktadır.
2. **Ne yapılmalı?**
   - Android tarafında `NotificationCompat`, `BroadcastReceiver` (`PendingIntent.getBroadcast` + `setShowsUserInterface(false)`), `AlarmManager.setAndAllowWhileIdle(RTC_WAKEUP)` ve `BOOT_COMPLETED` alıcısını içeren **yerel bir Tauri Android Plugin (`suu-notifications`)** yazılmalıdır.
