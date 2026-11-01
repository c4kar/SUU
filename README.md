# 💧 SUU — Kişisel Hidrasyon ve Su Alışkanlığı Uygulaması

Günde 1 litreden az su tüketen kullanıcılar için tasarlanmış, sıfır sürtünmeli, tamamen yerel (offline-first) çalışan Android hidrasyon takip uygulaması.

---

## Temel Özellikler

- **Bilimsel Günlük Hedef**: Kilo, yaş grubu (`<30`, `30–55`, `>55`) ve hareketlilik düzeyine göre günlük sıvı ihtiyacını hesaplar ($\text{round}_{50}(\text{Kilo} \times k_{\text{yaş}} + \Delta_{\text{aktivite}})$); `1.200 – 4.500 ml` aralığında `50 ml` adımlarla serbestçe özelleştirilebilir.
- **Türkiye Standart Bardak Porsiyonları ve Hidrasyon Katsayıları**:
  - **Su (`×1.00`)**: `1 Su Bardağı (200 ml)`, `1 Yudum Su (50 ml)`, `1 Kupa Su (300 ml)`, `1 Şişe / Matara (500 ml)`
  - **Çay (`×0.90`)**: `1 İnce Belli Çay (100 ml → net 90 ml)`, `1 Ajda Çay (150 ml → net 135 ml)`, `1 Kupa Çay (200 ml → net 180 ml)`
  - **Kahve (`×0.80`)**: `1 Fincan Türk Kahvesi (70 ml → net 56 ml)`, `1 Kupa Filtre Kahve (200 ml → net 160 ml)`
  - **Ayran (`×1.00`)**: `1 Küçük Bardak Ayran (200 ml)`, `1 Büyük Bardak Ayran (300 ml)`
- **%70 Saf Su Alt Sınırı / %30 Diğer İçecek Kotası**: Çay, kahve ve ayran tüketimi günlük hedefin en fazla **%30**'unu karşılayabilir; kalan **%70** doğrudan saf su ile tamamlanır.
- **Sıfır Sürtünmeli Samsung One UI Bildirimleri**:
  - Bildirim üzerindeki **`+Su (1 Su Bardağı)`** ve **`+Su (1 Yudum)`** butonları uygulamayı açmadan (`BroadcastReceiver`) arka planda anında kayıt yapar ve hatırlatma sayacını sıfırlar.
  - **`Diğer`** butonu uygulamayı doğrudan **Hızlı Seçim Çekmecesi** açık halde başlatır.
  - Günlük hedef `%100` tamamlandığında o günün kalan bildirimleri otomatik olarak susturulur.
- **Uyku Odaklı Gün Dönümü (`04:00`) ve İki Katmanlı Esnek Seri (Streak)**:
  - Gece `00:00 – 03:59` arasındaki kayıtlar bir önceki güne dahil edilir.
  - `%100` hedefle seri `+1` artar, `%80–99` **Koruma Eşiği**'nde seri korunur, `<%80` durumunda ise haftada 1 kez **Haftalık Mola Hakkı** devreye girerek serinin sıfırlanmasını önler.

---

## Teknik Mimari

| Katman | Teknoloji | Sorumluluk |
| :--- | :--- | :--- |
| **Ön Yüz (UI)** | Svelte 5 (Runes) + Vite + TypeScript | Canlı Su Haznesi, iOS stil vektörel ikonlar (`IosEmoji.svelte`), Hızlı Seçim Çekmecesi, Açık/Gece teması |
| **Uygulama Çekirdeği** | Tauri v2 (Rust) | Domain kuralları, seri algoritması ve Tauri IPC komutları (`src-tauri/src/`) |
| **Mobil Bildirim Eklentisi** | Kotlin (`tauri-plugin-suu-notifications`) | `AlarmManager.setExactAndAllowWhileIdle`, `BOOT_COMPLETED`, `BeverageActionReceiver` ve Samsung Deep Sleep muafiyeti |
| **Yerel Veri Katmanı** | SQLite (`WAL` Modu — `suu.db`) | Hem Rust (`rusqlite`) hem Android (`android.database.sqlite`) tarafından paylaşılan tekil gerçeklik kaynağı |

Detaylı kavram sözlüğü için [CONTEXT.md](./CONTEXT.md), tam ürün ve veritabanı spesifikasyonu için [.scratch/suu/spec.md](./.scratch/suu/spec.md) dosyalarına bakabilirsiniz.

---

## Geliştirme ve Derleme Komutları

```bash
# Bağımlılıkları yükle
bun install

# Ön yüz birim testlerini çalıştır
bun test

# Rust ve SQLite WAL entegrasyon testlerini çalıştır
cd src-tauri && cargo test

# Masaüstü / Tarayıcı geliştirme sunucusu
bun run dev

# Tauri masaüstü penceresinde çalıştır
bun run tauri dev

# Samsung (ARM64) imzalı Android APK üret (suu-samsung-arm64.apk)
bun run android:apk
```
