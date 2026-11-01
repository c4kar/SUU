# SUU

Bireylerin günlük sıvı tüketim alışkanlıklarını bilimsel hedefler, kişiselleştirilmiş hatırlatıcılar ve hızlı kayıtlarla geliştirmeyi amaçlayan kişisel hidrasyon takip bağlamı.

## Language

**Günlük Hedef**:
Kullanıcının fiziksel ve çevresel parametrelerine göre bilimsel olarak hesaplanan, bir gün içinde ulaşması gereken toplam net hidrasyon miktarı (ml).
_Avoid_: Kota, limit, su borcu

**İçecek Türü**:
Tüketilen sıvının kategorisi (Su, Çay, Kahve, Ayran) ve birim hacminin günlük hedefe ne oranda katkı sağladığını belirleyen hidrasyon profili.
_Avoid_: Ürün, menü öğesi

**Hidrasyon Katsayısı**:
Bir içecek türünün birim hacminin, su tutulumu ve kafein/solüt etkisine göre net hidrasyona dönüşüm çarpanı (`Su: 1.00`, `Çay: 0.90`, `Kahve: 0.80`, `Ayran: 1.00`).
_Avoid_: Puan çarpanı, kalori katsayısı

**Porsiyon**:
Her bir içecek türü için kullanıcının tek seferde tükettiği standart kap/bardak hacmi (örn. ince belli çay bardağı, su bardağı, kupa).
_Avoid_: Doz, birim

**Varsayılan Porsiyon**:
Her içecek türü için kullanıcının tek dokunuşla hızlı kayıt yaparken kullandığı seçili standart bardak hacmi.
_Avoid_: Sabit hacim

**Efektif Hidrasyon Katkısı**:
Tüketilen porsiyon hacminin ilgili içeceğin hidrasyon katsayısı ile çarpılmasıyla elde edilen net sıvı karşılığı (ml).
_Avoid_: Brüt hacim, ham miktar

**Saf Su Alt Sınırı**:
Günlük hedefin en az %70'inin doğrudan saf sudan karşılanmasını zorunlu kılan klinik kural.
_Avoid_: Su tavsiyesi

**Diğer İçecek Kotası**:
Su dışındaki içeceklerin (Çay, Kahve, Ayran) günlük hedef ilerlemesine yapabileceği en yüksek toplam efektif katkı tavanı (günlük hedefin en fazla %30'u).
_Avoid_: Çay/kahve yasağı, günlük içecek limiti

**Tüketim Kaydı**:
Kullanıcının belirli bir anda seçtiği içecek türünden ve porsiyondan ne kadar tükettiğini gösteren zaman damgalı kayıt.
_Avoid_: İşlem, sipariş, giriş

**Hatırlatma Planı**:
Kullanıcının aktif olduğu saat aralığında hangi sıklıkla bildirim alacağını belirleyen ve her yeni tüketim kaydında sayacı yeniden başlatan zamanlama yapısı.
_Avoid_: Alarm listesi, sabit zamanlayıcı

**Aktif Saat Aralığı**:
Kullanıcının uyanık olduğu ve hatırlatma bildirimlerinin gönderilmesine izin verilen günlük zaman penceresi (varsayılan `09:00 – 22:00`).
_Avoid_: Mesai saati, çalışma süresi

**Hızlı Seçim Çekmecesi**:
Bildirimdeki "Diğer" aksiyonuna veya ana ekrandaki içecek ekleme alanına dokunulduğunda açılan, tüm içecek türlerini ve porsiyonlarını günlük dildeki adlarıyla tek dokunuşla kaydetmeyi sağlayan görsel panel.
_Avoid_: Alt menü, ayar ekranı

**Gün Dönümü**:
Yeni bir takip gününün başladığı ve günlük ilerleme sayacının sıfırlandığı sabaha karşı zaman sınırı (`04:00`).
_Avoid_: Gece yarısı sıfırlaması

**Günlük Seri**:
Kullanıcının günlük hidrasyon hedefini (%100) tamamladığı ardışık gün sayısı; koruma eşiği ve haftalık mola hakkı ile desteklenir.
_Avoid_: Puan, seviye, rozet

**Koruma Eşiği**:
Günlük hedefin `%80 – %99` aralığında tamamlandığı günlerde, haftalık mola hakkını harcamadan günlük serinin sıfırlanmadan sabit kalmasını sağlayan alt sınır.
_Avoid_: Yarım başarı

**Haftalık Mola Hakkı**:
Kullanıcının bir hafta içinde günlük hedefinin `%80`'inin altında kaldığı tek bir günde serisinin sıfırlanmasını önleyen otomatik koruma kalkanı.
_Avoid_: Ceza affı, joker puanı
