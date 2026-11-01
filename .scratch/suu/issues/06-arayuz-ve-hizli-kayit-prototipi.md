# Arayüz, Hızlı Kayıt Etkileşimi ve Tepkisel Animasyon Prototipi

Type: prototype
Status: resolved
Blocked by: 03, 04, 05

## Question

Netleşen içecek/porsiyon modeli (1 Yudum Su, 1 Su Bardağı, İnce Belli/Ajda Çay, Türk Kahvesi, Ayran; %70 Saf Su / %30 Diğer İçecek Kotası), bildirimden açılan **Hızlı Seçim Çekmecesi** ve bardak diliyle geçmiş/seri (`%80` Koruma Eşiği + Haftalık Mola Hakkı) kuralları doğrultusunda; anne ve abla profilinin tek elle, sıfır sürtünmeyle kullanabileceği ana ekran, hızlı içecek ekleme çekmecesi ve tepkisel su dolum animasyonu nasıl görünmeli ve davranmalıdır?

## Answer

[Etkileşimli Arayüz Prototipi](../prototype/ui-prototype.html)

### Kararlaştırılan Arayüz Mimarisi: Varyant A ("Canlı Su Haznesi + Alt 3'lü Hızlı Bar")
Teknolojiden uzak, hız ve sadelik arayan genç-yetişkin kullanıcı profili için **Varyant A** temel alınarak şu tasarım ilkeleri kesinleştirildi:

1. **Merkezi "Canlı Su Haznesi" (Görsel Dolum ve Bardak Dili)**:
   - Ekranın merkezinde her kayıtla birlikte dalgalanarak yükselen büyük bir **Su Haznesi** yer alır (Saf Su mavi katmanla, Çay/Kahve/Ayran kehribar katmanla görselleştirilir).
   - Kullanıcıya teknik mililitre veya katsayı matematiği gösterilmez; haznenin ortasında en okunaklı haliyle **"Hedefe X Su Bardağı kaldı"** (ve tamamlanma yüzdesi) yazar.
2. **Bildirim ile Birebir Aynı Alt 3'lü Sabit Bar (Sıfır Öğrenme Maliyeti)**:
   - Ana ekranın en altında, başparmak hizasında bildirimdeki butonlarla birebir aynı 3 dev buton sabit durur:
     1. **`🥛 +1 Su Bardağı`** (Tek dokunuşla anında ekler)
     2. **`💧 +1 Yudum`** (Tek dokunuşla anında ekler)
     3. **`🫖☕ Diğer`** (Dokununca **Hızlı Seçim Çekmecesi** açılır)
3. **Jargonsuz Hızlı Seçim Çekmecesi**:
   - `Diğer` butonuna (bildirimden veya ana ekrandan) basıldığında açılan çekmecede `×0.90` veya `net 135 ml` gibi kafa karıştırıcı hesaplar **gösterilmez**; yalnızca büyük ikonlar ve günlük dildeki isimler (**`1 İnce Belli Çay`**, **`1 Ajda Çay`**, **`1 Fincan Türk Kahvesi`**, **`1 Bardak Ayran`**, **`1 Şişe Su`**) yer alır. Katsayı ve %30 kota hesabı tamamen arka planda sessizce yapılır.
4. **Sade Günlük Kayıt Listesi ve "Dün" Sekmesi**:
   - Üst bardaki **"Bugün / Dün"** geçişi ve haznenin hemen altındaki sade **"Bugün İçtiklerim"** listesi ile yanlış eklenen bir bardak tek dokunuşla (`Sil`) geri alınabilir.
