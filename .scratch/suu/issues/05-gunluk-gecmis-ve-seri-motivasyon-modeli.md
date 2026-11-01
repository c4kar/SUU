# Günlük Kayıt Geçmişi, Seri (Streak) Kuralları ve Motivasyon Geri Bildirimi

Type: grilling
Status: resolved
Blocked by: 03

## Question

Gün dönümü (gece 00:00 vs. esnek uyku saati), geçmiş günlerin takvim/grafik görünümünde hangi metriklerin (toplam hidrasyon vs. saf su / çay / kahve / ayran dağılımı) gösterileceği, "Günlük Seri (Streak)" kazanma/koruma koşulu ve geçmişe dönük kayıt ekleme/silme kuralları nasıl olacak?

## Answer

### 1. Uyku Odaklı Gün Dönümü (`04:00`)
- Yeni takip günü gece `00:00`'da değil, sabaha karşı **`04:00`**'te başlar (**Gün Dönümü**).
- Gece `00:00 – 03:59` arasında yatmadan önce içilen su ve içecekler bir önceki (uyanık olunan) günün hedefine dahil edilir; böylece gece yarısını geçtiği için hedefin yanması önlenir.

### 2. İki Katmanlı Esnek Günlük Seri (Streak) Modeli (`A + C`)
Kullanıcının motivasyonunu kırmamak ve "serim bozuldu, uygulamayı bıraktım" sendromunu önlemek için üç kademeli seri kuralı uygulanır:
1. **`%100` Tamamlama (Seri Artışı)**: Günlük Hedef (`%70` Saf Su Alt Sınırı kuralına uygun olarak) `%100`'e ulaştığında **Günlük Seri `+1` artar**.
2. **`%80 – %99` Koruma Eşiği**: Gün sonunda ilerleme `%80 – %99` arasında kalırsa seri artmaz ancak **sıfırlanmaz da** (mevcut sayısında korunur ve haftalık mola hakkını harcamaz).
3. **`<%80` Haftalık Mola Hakkı (Haftada 1 Gün Kalkan)**: İlerleme `%80`'in altında kalırsa, haftada **1 gün** otomatik **Haftalık Mola Hakkı** devreye girer ve seri yine sıfırlanmaz (*"Dün mola verdin ama serin güvende, bugün devam!"*). Aynı hafta içinde ikinci kez `%80` altında kalınırsa seri `0`'a sıfırlanır.

### 3. Yanlış Kaydı Silme ve "Dün" İçin Telafi Eklemesi
- **Bugün**: Ana ekrandaki "Bugün İçtiklerim" listesinden yanlışlıkla eklenen herhangi bir **Tüketim Kaydı** tek dokunuşla silinebilir.
- **Sadece "Dün" İçin Telafi**: Kullanıcı "Dün" gününe geçerek unuttuğu içecek kaydını ekleyebilir veya silebilir; bu işlem dünün tamamlanma yüzdesini, **Günlük Seri** değerini ve harcanmışsa **Haftalık Mola Hakkı**'nı otomatik olarak yeniden hesaplar. Daha eski günler (2+ gün öncesi) değiştirilemez.

### 4. Geçmiş Görünümü: Sade Takvim ve "Bardak Diliyle" Günlük Özet
- Karmaşık mililitre ve yüzde grafikleri kullanılmaz.
- Üstte son 7 günün ve ayın tamamlanma halkalarını gösteren sade bir takvim yer alır.
- Seçilen günün altında ise tüketilen içecekler doğrudan **günlük bardak diliyle** özetlenir (örn. *"💧 6 Su Bardağı + 2 Yudum Su · 🫖 3 İnce Belli Çay · ☕ 1 Fincan Kahve"*).
