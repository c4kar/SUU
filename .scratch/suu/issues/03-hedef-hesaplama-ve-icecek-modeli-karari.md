# Günlük Hedef Formülü, İçecek Katsayıları ve Porsiyon Modeli Kararı

Type: grilling
Status: resolved
Blocked by: 01

## Question

Bilimsel hidrasyon araştırmasının bulguları ışığında; ilk kurulumda (onboarding) kullanıcıdan hangi parametreleri (kilo, yaş, aktivite vb.) isteyeceğiz, günlük hedefi hangi formülle hesaplayacağız, Su/Çay/Kahve/Ayran için varsayılan porsiyon hacimlerini (ml) ve hidrasyon çarpanlarını nasıl sabitleyeceğiz ve kullanıcının tüm hedefini çayla kapatmasını önleyecek "saf su alt sınırı" kuralını nasıl modellendireceğiz?

## Answer

### 1. İlk Kurulum (Onboarding) Parametreleri ve Akışı
- **Tek Ekranlı Hızlı Kurulum + "Atla" Seçeneği**: İlk açılışta kullanıcıdan tek ekranda yalnızca **Kilo (kg)**, **Yaş Grubu (`<30`, `30–55`, `>55`)** ve **Günlük Hareketlilik (`Düşük / Orta / Yüksek`)** istenir; cinsiyet sorulmaz.
- Sağ üstte **"Şimdilik Atla (2.000 ml ile başla)"** seçeneği yer alır; atlandığında varsayılan **Günlük Hedef** `2.000 ml` olarak atanır.

### 2. Günlük Hedef Formülü ve Manuel Müdahale
- **Hesaplama Formülü**: $\text{Günlük Hedef (ml)} = \text{round}_{50}(\text{Kilo (kg)} \times k_{\text{yaş}} + \Delta_{\text{aktivite}})$
  - Yaş katsayısı ($k_{\text{yaş}}$): `<30 yaş` için **35 ml/kg**, `30–55 yaş` için **32 ml/kg**, `>55 yaş` için **28 ml/kg**.
  - Aktivite eklemesi ($\Delta_{\text{aktivite}}$): Düşük `+0 ml`, Orta `+300 ml`, Yüksek `+500 ml`.
- **Serbest Manuel Ayar**: Hesaplanan bilimsel hedef ekranda öneri olarak gösterilir; kullanıcı bu değeri `50 ml` adımlarla **`1.200 ml – 4.500 ml`** güvenli aralığında dilediği gibi artırıp azaltabilir (otomatik haftalık hedef değiştirme yapılmaz).
- **Gün İçi Hedef Güncellemesi**: Gün içinde yapılan hedef değişikliği **bugünün** ilerlemesini ve %30'luk **Diğer İçecek Kotası** tavanını anında yeniden hesaplar; geçmiş günlerin tamamlanma ve seri (streak) kayıtları kendi günkü hedefleriyle mühürlü kalır.

### 3. İçecek Türleri, Hidrasyon Katsayıları ve Porsiyon Sabitleri
Serbest mililitre (klavye) girişi bulunmaz; Türkiye'ye özgü standart bardak porsiyonları ve sabit **Hidrasyon Katsayısı** değerleri kullanılır. Kullanıcı her içecek için kendi **Varsayılan Porsiyon** tercihini değiştirebilir:

| İçecek Türü | Hidrasyon Katsayısı | Standart Porsiyon Seçenekleri (Brüt → Efektif Hidrasyon Katkısı) | İlk Kurulum Varsayılan Porsiyonu |
| :--- | :---: | :--- | :--- |
| **Su** | `×1.00` | 1 Yudum (`50 ml` → `50 ml`), 1 Su Bardağı (`200 ml` → `200 ml`), 1 Kupa (`300 ml` → `300 ml`), 1 Matara/Şişe (`500 ml` → `500 ml`) | **1 Su Bardağı (`200 ml`)** |
| **Çay** | `×0.90` | 1 İnce Belli (`100 ml` → `90 ml`), 1 Ajda (`150 ml` → `135 ml`), 1 Kupa (`200 ml` → `180 ml`) | **1 İnce Belli (`100 ml` → net `90 ml`)** |
| **Kahve** | `×0.80` | 1 Fincan Türk Kahvesi (`70 ml` → `56 ml`), 1 Kupa Filtre/Americano (`200 ml` → `160 ml`) | **1 Fincan Türk Kahvesi (`70 ml` → net `56 ml`)** |
| **Ayran** | `×1.00` | 1 Küçük Bardak/Kutu (`200 ml` → `200 ml`), 1 Büyük Bardak (`300 ml` → `300 ml`) | **1 Küçük Bardak/Kutu (`200 ml`)** |

- **Günlük Dil Odaklı İsimlendirme**: Hem bildirimlerde hem uygulama ekranlarında mililitre rakamları yerine günlük dildeki birim adları (`1 Su Bardağı`, `1 Yudum`, `1 İnce Belli Çay`, `1 Fincan Kahve`) ana etiket olarak gösterilir; ml değeri yalnızca yardımcı bilgi olarak yer alır.
- **Sade Kayıt Akışı**: Kahve eklendiğinde ekstra "yanında su ekle" alt çubuğu/önerisi çıkarılmaz; tüm içeceklerde yalnızca standart kayıt geri bildirimi çalışır.

### 4. Saf Su Alt Sınırı (%70) ve Diğer İçecek Kotası (%30 Sert Tavan)
- Günlük hedefin **en az %70'i** (**Saf Su Alt Sınırı**) doğrudan saf sudan karşılanmalıdır.
- Su dışındaki içeceklerin (Çay, Kahve, Ayran) günlük hedef ilerlemesine toplam katkısı **Günlük Hedef'in en fazla %30'u** (**Diğer İçecek Kotası**) ile sınırlandırılır (Sert Tavan / Hard Cap):
  $$\text{Net Günlük İlerleme} = \text{Saf Su Toplamı} + \min\left(\text{Diğer İçeceklerin Efektif Toplamı},\; \text{Günlük Hedef} \times 0.30\right)$$
- Kota dolduktan sonra içilen çay/kahve/ayran **Tüketim Kaydı** olarak geçmişe eklenmeye devam eder (kayıt engellenmez), ancak hedef halkasını %100'e tamamlamak ve **Günlük Seri**'yi kazanmak için kalan %70'lik kısmın saf su ile doldurulması gerekir.
