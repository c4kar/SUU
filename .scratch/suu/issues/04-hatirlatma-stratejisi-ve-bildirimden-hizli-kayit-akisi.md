# Hatırlatma Seçenekleri ve Bildirimden Hızlı Kayıt Akışı Kararı

Type: grilling
Status: resolved
Blocked by: 02, 03

## Question

Tauri v2 Android bildirim/arka plan araştırması ve içecek porsiyon modeline dayanarak; kullanıcıya sunulacak hatırlatma planı seçenekleri (sabit zaman aralığı, hedef odaklı akıllı tempo, aktif saat aralığı/uyku modu) neler olacak ve bildirim geldiği anda en az dokunuşla sıvı kaydetme (bildirim üzerindeki aksiyon butonları ve/veya bildirime dokununca açılan hızlı kayıt ekranı) akışı nasıl çalışacak?

## Answer

### 1. Hatırlatma Planı: Aktif Saat Aralığı ve Kayıt Sıfırlamalı Akıllı Sıklık
- **Aktif Saat Aralığı (Uyku Modu Koruması)**: Varsayılan olarak **`09:00 – 22:00`** arasıdır (kullanıcı tarafından değiştirilebilir). Bu saatler dışında bildirim gönderilmez.
- **Kayıt Sıfırlamalı Sabit Sıklık**: Kullanıcı `1 saat`, `1.5 saat`, `2 saat` (varsayılan) veya `3 saat` sıklık seçer. Kullanıcı bildirimden veya uygulama içinden herhangi bir **Tüketim Kaydı** eklediğinde, bir sonraki hatırlatma sayacı o andan itibaren otomatik olarak `şimdi + sıklık` zamanına ötelenir (yeni su içmiş kullanıcıya hemen ardından bildirim gitmez).
- **Hedef (%100) Tamamlandığında Otomatik Susturma**: Günlük Hedef %100'e ulaştığı anda o günün kalan hatırlatmaları iptal edilir ve zamanlayıcı doğrudan **ertesi sabahın başlangıç saatine** (örn. `09:00`) kurulur.

### 2. Bildirim Üzerindeki 3 Hızlı Aksiyon Butonu ve "Diğer" Akışı
Anlaşılırlık ve kullanım kolaylığı en yüksek öncelik olduğundan, bildirimlerde teknik mililitre ifadeleri yerine günlük dildeki karşılıklar kullanılır. Standart hatırlatma bildiriminde şu 3 aksiyon butonu yer alır:
1. **`+Su (1 Su Bardağı)`** (`200 ml` — `PendingIntent.getBroadcast` ile uygulamayı açmadan arka planda anında kaydeder).
2. **`+Su (1 Yudum)`** (`50 ml` — `PendingIntent.getBroadcast` ile uygulamayı açmadan arka planda anında kaydeder).
3. **`Diğer`** (`PendingIntent.getActivity` ile uygulamayı doğrudan **Hızlı Seçim Çekmecesi** açık halde başlatır; burada Çay [İnce Belli, Ajda, Kupa], Kahve [Fincan, Kupa], Ayran [Küçük/Büyük Bardak] ve diğer Su birimleri [Kupa, Şişe] büyük ve anlaşılır kartlarla tek dokunuşla seçilebilir).

### 3. Diğer İçecek Kotası (%30) Dolduğunda Bildirim Davranışı
- Gün içinde %30'luk **Diğer İçecek Kotası** dolmuşsa, öğleden sonra/akşam gelen bildirimlerin metni *"Çay/kahve kotan doldu! Kalan hedefin için saf su zamanı 💧"* şeklinde uyarlanır ve bildirim altında yalnızca saf su aksiyonları (**`+Su (1 Su Bardağı)`** ve **`+Su (1 Yudum)`**) gösterilir.

### 4. İlk Kurulum İzinleri ve Samsung (One UI) Özel Uyum Paketi
Hedef cihazlar **Samsung (One UI)** olduğu ve uygulama Play Store dışı yerel APK olarak kurulacağı için:
- **Kurulumda Varsayılan Plan + İzin Akışı**: Onboarding ekranında *"Hatırlatmalar: 09:00 – 22:00, her 2 saatte bir"* hazır gösterilir; **"Başla"** butonuna basıldığında sırasıyla:
  1. Android 13+ bildirim izni (`POST_NOTIFICATIONS`),
  2. Samsung One UI'ın uygulamayı "Derin Uyku (Deep Sleep)" listesine almasını önleyen tek dokunuşluk **"Pil optimizasyonunu yoksay (Kısıtlanmamış)"** (`ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`) sistem izni istenir.
- **Kesin Zamanlama ve Açık Buton Görünümü**: Manifest'te `USE_EXACT_ALARM` tanımlanarak alarmlar `AlarmManager.setExactAndAllowWhileIdle` ile kurulur; bildirim kanalı `IMPORTANCE_HIGH` (ve `BigTextStyle`) olarak yapılandırılarak Samsung One UI'da 3 aksiyon butonunun daraltılmadan doğrudan görünmesi garanti altına alınır.
