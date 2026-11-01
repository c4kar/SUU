# Yerel Veri Saklama, Arka Plan Senkronizasyonu ve Nihai Spesifikasyon Kararı

Type: grilling
Status: resolved
Blocked by: 02, 04, 05, 06

## Question

Samsung One UI üzerinde çalışan özel Kotlin bildirim eklentisi (`suu-notifications` — `BeverageActionReceiver` ve `AlarmManager.setExactAndAllowWhileIdle`) ile Tauri v2 (Rust) / Svelte 5 ön yüz katmanı aynı yerel veriyi yarış durumu (race condition) olmadan nasıl paylaşacak (SQLite WAL vs. atomik JSONL kuyruğu + SQLite) ve tüm kararları birleştiren nihai Ürün & Teknik Spesifikasyon (`.scratch/suu/spec.md`) şeması nasıl yapılandırılacak?

## Answer

[Nihai Ürün & Teknik Spesifikasyon (`spec.md`)](../spec.md)

### 1. Paylaşımlı Tekil SQLite Veritabanı (`suu.db` — `WAL` Modu)
- Uygulamanın dahili veri dizininde (`context.filesDir/suu.db`) `PRAGMA journal_mode = WAL;` ve `PRAGMA busy_timeout = 5000;` ile yapılandırılmış **tek bir SQLite veritabanı** kullanılır.
- **Tauri (Rust `rusqlite`)** ve **Android (`android.database.sqlite.SQLiteDatabase`)** aynı `suu.db` dosyasına doğrudan erişir.
- Uygulama kapalıyken bildirimdeki **`+Su (1 Su Bardağı)`** veya **`+Su (1 Yudum)`** butonuna basıldığında `BeverageActionReceiver` kaydı doğrudan `intake_logs` tablosuna yazar, o günkü efektif ilerlemeyi SQL üzerinden hesaplar; hedef `%100` olduysa o günün hatırlatmalarını susturup alarmı ertesi sabahın başlangıç saatine (`09:00`) kurar, olmadıysa `şimdi + sıklık` olarak öteler.

### 2. Çift Yönlü Anlık Senkronizasyon ve `"Diğer"` Intent Akışı
- Uygulama açık/arka plandayken bildirimden kayıt yapıldığında Kotlin eklentisi `intakeUpdated` olayı fırlatır; ayrıca Svelte 5 her `visibilitychange` / `focus` geçişinde `get_dashboard_state` komutunu çağırarak **Canlı Su Haznesi**'ni anında günceller.
- Bildirimdeki **`Diğer`** butonuna basıldığında `MainActivity` `ACTION_OPEN_QUICK_DRAWER` intent'i ile başlatılır; Svelte 5 açılır açılmaz doğrudan **Hızlı Seçim Çekmecesi** açık halde ekrana gelir.

### 3. Ultra Sade Ayarlar Ekranı
- Teknolojiden uzak kullanıcı profili için Ayarlar ekranında yalnızca **1) Günlük Hedef (ve Kilo/Yaş/Hareketlilik hesabı)** ve **2) Hatırlatma Saatleri (`09:00–22:00`) ile Sıklığı** yer alır; yedekleme/dışa aktarma gibi teknik seçenekler bulunmaz.
