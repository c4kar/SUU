# Bilimsel Hidrasyon Formülü ve İçecek Katsayıları Araştırması

Type: research
Status: resolved
Blocked by: none

## Question

Yetişkin kullanıcılar (özellikle günde 1 litrenin altında su içen kadın profili) için günlük sıvı ihtiyacını belirleyen birincil bilimsel/klinik modeller (EFSA, IOM/NAM, ağırlık/yaş/aktivite tabanlı denklemler) nelerdir? Ayrıca **Su, demleme siyah Çay, Kahve (Türk kahvesi / filtre kahve) ve Ayran** için bilimsel literatürdeki Beverage Hydration Index (BHI — Maughan vd. 2016 ve ilgili klinik kaynaklar) katsayıları, porsiyon standartları ve günlük hedefin en az ne kadarının **saf sudan** karşılanması gerektiğine dair klinik ilkeler nelerdir?

## Answer

[Detaylı Araştırma Raporu](../research/01-bilimsel-hidrasyon-ve-icecek-katsayilari.md)

### Temel Bulgular ve Uygulama Kararları

1. **Günlük Sıvı Hedefi Formülü (EFSA 2010, TÜBER 2022 ve ESPEN Ağırlık Modeli):**
   * **Varsayılan Hedef (Profil girilmezse):** **2.000 ml/gün** (EFSA ve TÜBER yetişkin kadın referansı; erkekler için 2.500 ml/gün).
   * **Kişiselleştirilmiş Denklem:** $\text{Hedef (ml)} = \text{round}_{50}(\text{Kilo (kg)} \times k_{\text{yaş}} + \Delta_{\text{aktivite}} + \Delta_{\text{cinsiyet}})$
     * Yaş katsayısı ($k_{\text{yaş}}$): `<30 yaş` için **35 ml/kg**, `30–55 yaş` için **32 ml/kg**, `>55 yaş` için **28 ml/kg**.
     * Aktivite eklemesi ($\Delta_{\text{aktivite}}$): Sedanter `+0 ml`, Hafif aktif `+300 ml`, Orta aktif `+500 ml`, Yüksek aktif `+800 ml` (Erkek ise `+200 ml`; alt/üst güvenlik sınırı `1.500 – 4.500 ml`).
     * Günde `<1 L` su içen kullanıcılar için hesaplanan bilimsel hedef gösterilirken, kullanıcının hedefini dilerse kademeli başlangıç için manuel olarak (örn. `1.500 ml`) ayarlayabilmesine olanak tanınmalıdır.

2. **İçecek Hidrasyon Katsayıları (Maughan vd. 2016 BHI & Kafein/Sodyum Düzeltmesi):**
   * **Saf Su (`1.00`):** Referans hidrasyon kaynağı (BHI = 1.00), sıfır kalori ve sıfır böbrek solüt yükü.
   * **Demleme Siyah Çay (`0.90`):** Deneysel BHI 1.03–1.07 (su ile eşdeğer tutulum); ancak tanenler ve porsiyon başına ~35–50 mg kafein nedeniyle kümülatif tüketimde güvenli çarpan **`0.90`**.
   * **Kahve — Türk Kahvesi / Filtre Kahve (`0.80`):** Deneysel BHI 0.91; yüksek kafein konsantrasyonunun (Türk kahvesinde ~850 mg/L) hafif natriüretik etkisi ve günlük >250–300 mg eşiğinde diürezi artırması (Maughan & Griffin 2003; Killer vd. 2014) nedeniyle güvenli efektif çarpan **`0.80`**.
   * **Ayran (`1.00`):** Elektrolit ($\text{Na}^+, \text{K}^+$) ve süt proteini matrisi sayesinde fizyolojik BHI değeri 1.25–1.50 bandındadır; ancak tuz (~1–1.5 g/bardak) ve kalori yükü nedeniyle suyun yerini aşmaması için uygulamada efektif katsayısı **`1.00`** ile sınırlandırılır.

3. **Türkiye'ye Özgü Standart Porsiyonlar (Tek Dokunuşla Ekleme):**
   * **Su (`×1.00`):** Klasik Su Bardağı **`200 ml`**, Büyük Bardak/Kupa **`300 ml`**, Matara/Şişe **`500 ml`**.
   * **Çay (`×0.90`):** İnce Belli Çay Bardağı **`100 ml`** *(net 90 ml)*, Ajda Çay Bardağı **`150 ml`** *(net 135 ml)*, Kupa Çay **`200 ml`** *(net 180 ml)*.
   * **Kahve (`×0.80`):** Türk Kahvesi Fincanı **`70 ml`** *(net 56 ml; yanında +150 ml su hızlı ekleme önerisiyle)*, Kupa Filtre Kahve/Americano **`200 ml`** *(net 160 ml)*.
   * **Ayran (`×1.00`):** Küçük Kutu/Bardak **`200 ml`** *(net 200 ml)*, Büyük Bardak/Şişe **`300 ml`** *(net 300 ml)*.

4. **Saf Su Kotası (Pure Water Floor — %70 / %30 Kuralı):**
   * Böbrek solüt yükünü (oksalat, sodyum, üre) dengelemek, demir emilimini korumak ve EFSA günlük 400 mg kafein sınırını aşmamak için (Popkin vd. 2006; TÜBER 2022) günlük sıvı hedefinin **en az %70'i saf sudan** karşılanmalı; çay, kahve ve ayranın günlük hedef halkasına toplam katkısı **maksimum %30** ile sınırlandırılmalıdır.
