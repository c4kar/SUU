# SUU — Wayfinder Haritası

Label: `wayfinder:map`

## Destination

Tauri v2 (Rust) + Svelte 5 + Vite+ ile geliştirilecek, yerel (offline-first) Android hidrasyon ve su alışkanlığı uygulaması için hiçbir açık ürün veya mimari kararı kalmamış eksiksiz **Ürün & Teknik Spesifikasyon (`.scratch/suu/spec.md`)** ve **Domain Sözlüğü (`CONTEXT.md`)**.

## Notes

- **Hedef Kitle & Problem**: Günde 1 litreden az su içen yetişkin kullanıcılar (anne ve abla); sıfır sürtünmeli, hızlı ve alışkanlık kazandırıcı deneyim.
- **Hedef Platform**: Android (Tauri v2 Mobile — özellikle **Samsung One UI** cihazlar), tamamen yerel/bireysel kullanım (sunucu veya hesap sistemi yok).
- **Teknoloji Yığını**: Tauri v2 (Rust backend) + Svelte 5 + Vite+ (Frontend).
- **Domain Kapsamı**: Yalnızca saf su değil; **Su, Çay, Kahve ve Ayran** tüketimi porsiyonlarına ve bilimsel hidrasyon katsayılarına göre kaydedilir.
- **Her oturumda başvurulacak beceriler (Skills)**: `grilling`, `domain-modeling` (araştırma biletleri için `research`, arayüz doğrulaması için `prototype`).
- **Dil Tercihi**: Tüm dokümantasyon, biletler ve kullanıcı iletişimi Türkçe yürütülür.

## Decisions so far

<!-- the index: one line per closed ticket, enough to judge relevance, then zoom the link for the detail the ticket holds -->

- [Bilimsel Hidrasyon Formülü ve İçecek Katsayıları Araştırması](./issues/01-bilimsel-hidrasyon-ve-icecek-katsayilari.md): Günlük sıvı hedefi için `Kilo × YaşKatsayısı (28–35 ml/kg) + Aktivite` (varsayılan 2000 ml) modeli, Maughan BHI tabanlı içecek katsayıları (Su `1.00`, Çay `0.90`, Kahve `0.80`, Ayran `1.00`), standart Türkiye bardak porsiyonları ve en az %70 saf su tabanı kuralı belirlendi.
- [Tauri v2 Android Bildirim Aksiyonları ve Arka Plan Zamanlayıcı Araştırması](./issues/02-tauri-v2-android-bildirim-ve-arkaplan-kisitlari.md): Resmi `tauri-plugin-notification` v2 Android'de aksiyon butonlarında `MainActivity`'yi zorunlu açtığı, cold-start'ta aksiyonları düşürdüğü (`#3296`) ve reboot sonrası zamanlayıcıları kaybettiği (`#3550`/`#3751`) için; uygulama açılmadan arka planda kayıt (`PendingIntent.getBroadcast`) ve Doze uyumlu hatırlatma (`AlarmManager.setAndAllowWhileIdle` + `BOOT_COMPLETED`) sağlayan hafif özel Kotlin Tauri eklentisi (`suu-notifications`) mimarisi belirlendi.
- [Günlük Hedef Formülü, İçecek Katsayıları ve Porsiyon Modeli Kararı](./issues/03-hedef-hesaplama-ve-icecek-modeli-karari.md): Tek ekranlı hızlı kurulum (Kilo, Yaş Grubu, Hareketlilik + 2000 ml ile Atla), `1.200–4.500 ml` bandında 50 ml adımlı serbest hedef ayarı, günlük dil odaklı Türkiye porsiyon seti (1 Yudum Su 50 ml, 1 Su Bardağı 200 ml, 1 İnce Belli Çay 100 ml, 1 Fincan Türk Kahvesi 70 ml, 1 Bardak Ayran 200 ml vb.), kahvede ek su önerisi gösterilmemesi ve %30 Diğer İçecek Kotası sert tavanı (%70 Saf Su Alt Sınırı) kararlaştırıldı.
- [Hatırlatma Seçenekleri ve Bildirimden Hızlı Kayıt Akışı Kararı](./issues/04-hatirlatma-stratejisi-ve-bildirimden-hizli-kayit-akisi.md): `09:00–22:00` aktif saat aralığında her kayıtta sıfırlanan akıllı hatırlatma sayacı (varsayılan 2 saat), hedef %100 olunca günü otomatik susturma, bildirim üzerinde `+Su (1 Su Bardağı)`, `+Su (1 Yudum)` ve `Diğer` (Hızlı Seçim Çekmecesi açar) 3 buton kurgusu ve Samsung One UI tam uyum paketi (`ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS` + `USE_EXACT_ALARM` + `IMPORTANCE_HIGH`) kararlaştırıldı.
- [Günlük Kayıt Geçmişi, Seri (Streak) Kuralları ve Motivasyon Geri Bildirimi](./issues/05-gunluk-gecmis-ve-seri-motivasyon-modeli.md): Sabaha karşı `04:00` uyku odaklı Gün Dönümü, iki katmanlı esnek Günlük Seri modeli (`%100` seri artışı, `%80–%99` Koruma Eşiği, `<%80` için haftada 1 gün Haftalık Mola Hakkı), bugün ve sadece "Dün" için kayıt düzenleme/telafi izni ve bardak diliyle sade takvim özeti kararlaştırıldı.
- [Arayüz, Hızlı Kayıt Etkileşimi ve Tepkisel Animasyon Prototipi](./issues/06-arayuz-ve-hizli-kayit-prototipi.md): Teknolojiden uzak kullanıcı profili için teknik ml/katsayı jargonunu gizleyen, merkezde "Hedefe X Su Bardağı kaldı" diyen Canlı Su Haznesi (Varyant A), en altta bildirimle birebir aynı 3 dev buton (`+1 Su Bardağı`, `+1 Yudum`, `Diğer`) ve büyük ikonlu Hızlı Seçim Çekmecesi tasarımı doğrulandı.
- [Yerel Veri Saklama, Arka Plan Senkronizasyonu ve Nihai Spesifikasyon Kararı](./issues/07-yerel-veri-senkronizasyon-ve-nihai-spec.md): Hem Tauri (Rust `rusqlite`) hem de Android `BeverageActionReceiver` (`android.database.sqlite`) tarafından paylaşılan tekil `suu.db` (SQLite `WAL` modu) mimarisi, çift yönlü senkronizasyon + `ACTION_OPEN_QUICK_DRAWER` intent akışı ve ultra sade ayarlar kapsamı kararlaştırılarak nihai [Ürün ve Teknik Spesifikasyon (`spec.md`)](./spec.md) tamamlandı.

## Not yet specified

<!-- see "Fog of war": in-scope fog you can't ticket yet; graduates as the frontier advances -->
*(Tüm sis dağıldı ve açık karar biletlerine dönüştürüldü.)*

## Out of scope

- **Android Ana Ekran Widget'ı (Home Screen Widget)**: İlk sürümde odak bildirim aksiyonları ve uygulama içi hızlı kayıt üzerinde tutulduğu için kapsam dışı.
- **Hava Durumu / Sıcaklığa Göre Otomatik Hedef Güncelleme**: Harici ağ/konum bağımlılığı getirdiği ve tamamen yerel (offline) sadeliği bozduğu için kapsam dışı.
- **Çoklu Kullanıcı / Bulut Senkronizasyonu / Sosyal Takip**: Her cihazda tekil ve yerel kullanım hedeflendiği için kapsam dışı.
