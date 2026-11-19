# Rapor: C++'tan Rust'a — Buton + LED

Tarih: 19.11.2026 · Kart: Nucleo-G474RE · Dil: Rust (Embassy) · Firmware sürümü: 0.1.0.1 (alpha)

## 1. Ne yaptık

Nucleo kartındaki mavi **B1** butonuna basınca yeşil **LD2** LED'i yanıyor, bırakınca sönüyor. Bu kadar basit bir iş için
kodu bilerek "büyük proje gibi" yazdık. Amaç, C++'ta kullandığımız yazım stilinin (katmanlar, 400 Hz zamanlayıcı,
`initialize/scheduler`, debounce, sürüm, watchdog) Rust'ta nasıl göründüğünü küçük ve anlaşılır bir örnekte görmek.

| | |
|---|---|
| LED | LD2, PA5, HIGH = yanık |
| Buton | B1, PC13, basınca HIGH (dahili pull-down) |
| Saat | HSI16 → PLL → 170 MHz |
| Program boyutu | 11,3 KB flash, 1,3 KB RAM (release) |
| Log | ST-LINK üzerinden RTT (`defmt`), UART kablosu gerekmez |

## 2. Program nasıl çalışıyor

```
 güç verildi
     │
     ▼
 main.rs ──► bsp::initialize()      saat 170 MHz, PA5 çıkış (LED sönük), PC13 giriş, watchdog
     │
     ▼
 RtScheduler::run()  ── her 2,5 ms'de bir (400 Hz) ──┐
     │                                               │
     ├─ thread100Hz (her 10 ms)                      │
     │     ├─ button.scheduler()   pini oku, 3 ardışık aynı okuma = karar (30 ms debounce)
     │     └─ indicator.scheduler() buton basılı mı? → led.set(true/false)
     │
     ├─ thread1Hz (her 1 s)  "$alive led=.. presses=.. raw=.. load=.." logu
     │
     └─ tick sonu: watchdog beslenir, tick süresi ölçülür
```

Butonu "bastı" saymak için tek okuma yetmiyor. Arka arkaya 3 okuma aynı sonucu vermeli. Buton kontakları
basarken birkaç milisaniye titreşir (bounce); bu yöntem o titreşimi eler. C++'taki NTC ve fan sürücülerinin kullandığı
"ardışık hata sayacı" mantığının aynısı.

## 3. Klasörler — C++ projesiyle karşılaştırma

| C++ (UniBoard firmware) | Rust (bu repo) | Görevi |
|---|---|---|
| `Core/` (CubeMX üretir) + `.ioc` | `src/bsp.rs` | Saat, pinler, hangi pin hangi görevde. Rust'ta CubeMX yok; bunu elle, kodla yazıyoruz |
| `Middleware/` (UART/GPIO soyutlama) | Embassy kütüphanesinin kendisi (`embassy-stm32`) | Donanım soyutlaması hazır geliyor, biz yazmıyoruz |
| `Libraries/` (sürücüler) | `src/drivers/` (`led.rs`, `button.rs`) | Cihaz sürücüleri; pin numarası bilmezler |
| `App/` (iş mantığı) | `src/app/indicator.rs` | "Buton basılıysa LED yansın" kuralı |
| `App/main.cpp` (RT_SCHEDULER) | `src/scheduler.rs` | 400 Hz zamanlayıcı, thread'ler |
| `Common/` | `src/common/` (`version.rs`, `mcu_timer.rs`) | Sürüm, MCU yükü ölçümü |
| `.hpp` + `.cpp` çifti | tek `.rs` dosyası | Rust'ta başlık dosyası yok |
| STM32CubeIDE, make | `cargo` | Derleme, bağımlılıklar, flash |

## 4. Farklar — basit dille

### 4.1 Değişmeyenler (stilimiz aynen duruyor)

- Katmanlar ve bağımlılık yönü aynı: app → drivers → HAL. Pin numaraları tek dosyada (`bsp.rs`).
- 400 Hz zamanlayıcı birebir aynı: aynı bölenler (2, 4, 8, 20…) ve aynı faz kaydırmaları (0, 1, 3, 7, 11, 15, 23, 63).
- Her modül `new` + `scheduler()` + get/set konuşuyor.
- Sabitler isimli ve birimli (`WATCHDOG_TIMEOUT_US`, `TICK_BUDGET_US`); sihirli sayı yok.
- Her `if / else` zinciri yorumlu bir `else` ile bitiyor.
- Dosya başlık blokları, `USAGE INFORMATION` bölümü ve `END OF FILE` satırı aynı.
- Sürüm `major.minor.patch.suffix`, CHANGELOG aynı formatta.
- MCU yükü ve en uzun tick süresi her saniye ölçülüp loga basılıyor (C++'taki `timer.getMcuLoad()` karşılığı).

### 4.2 Değişenler

| Konu | C++'ta | Rust'ta | Ne kazandırıyor / ne kaybettiriyor |
|---|---|---|---|
| **Pin kimin?** | Global nesneler (`extern`), her dosya her pine yazabilir | Pin, onu `new`'e verdiğin nesnenin **malı** olur. Başka hiçbir kod ona dokunamaz, derleyici izin vermez | C++ firmware'deki "test kodu ısıtıcı PWM'ine doğrudan yazıyor" hatası Rust'ta **derlenmez**. En büyük kazanç bu |
| **Tick bayrağı** | TIM6 kesmesi `tick = 1` yazar; ana döngü okur. `volatile` unutulursa optimizasyonda takılır | `ticker.next().await` — paylaşılan değişken yok | "volatile unuttum" türü hata imkânsız |
| **Kesme ile paylaşılan veri** | `volatile` + kesmeyi kapat/aç elle | Atomik tipler veya `critical_section`; unutursan derlenmez | Veri yarışı (data race) derleme hatası |
| **Saat ve pin ayarı** | CubeMX'te tıklayarak (`.ioc`) | `bsp.rs` içinde kodla (`PllMul::MUL85` …) | Görsel araç yok; ama ayar tamamen kodda, git'te diff'i okunur |
| **Hata dönüşü** | `uint8_t` 0/1/2 | `Result<T, Error>`; hatayı kontrol etmeden geçemezsin | Bu örnekte gerekmedi (GPIO hata vermiyor); SPI/I²C'de kullanılacak |
| **Derleme anı kontrolü** | `static_assert` | `const _: () = assert!(...)` | Örnek: butonun örnekleme hızı scheduler'la uyuşmazsa kod **derlenmez** (`scheduler.rs`) |
| **Log** | `printf` → UART → FT232 | `defmt::info!` → ST-LINK üzerinden RTT | Ekstra kablo yok; log mesajları cihazda sıkıştırılmış tutulur, flash'ta az yer kaplar |
| **Derleme / flash** | CubeIDE butonları, post-build `.bat` | `cargo run --release` (derle + flash + log tek komut) | Komut satırından tekrarlanabilir; CI'da da çalışır |
| **Kütüphane sürümü** | HAL, projenin içine kopyalanmış | `Cargo.toml` + `Cargo.lock` sürümleri sabitler, internetten indirir | Proje küçük kalıyor; ama ilk derleme internet ister |
| **Panik / çökme** | HardFault_Handler'da sonsuz döngü | `panic-probe`: hata mesajı ve dosya/satır loga basılır | Hatanın yeri doğrudan görünür |
| **Watchdog** | C++ firmware'de yoktu | IWDG açık, her tick sonunda besleniyor (500 ms) | Bir thread takılırsa kart kendini resetler |

### 4.3 Zorlandığımız yerler (dürüst not)

- **Embassy hızlı değişiyor.** Bu proje embassy-stm32 0.6 ve embassy-executor 0.10 ile yazıldı. Önceki sürümlerdeki bazı
  isimler değişmiş (`arch-cortex-m` → `platform-cortex-m`), G474 için flash bank seçimi (`dual-bank`) zorunlu olmuş.
  İnternetteki eski örnekler birebir çalışmayabilir.
- **CubeMX yok.** Pinin hangi alternatif fonksiyonu (AF) desteklediğini Embassy derleme anında kontrol ediyor; yanlış pin
  verirsen derlenmiyor. Bu iyi, ama hata mesajlarını okumayı öğrenmek zaman alıyor.
- **İlk derleme uzun** (~1 dakika); sonrakiler birkaç saniye.
- **Öğrenme eğrisi:** sahiplik (ownership) ve ödünç alma (borrow) kuralları ilk haftalarda yavaşlatır.

## 5. Kartta doğrulanacaklar

- [ ] **B1 polaritesi:** MB1367 kartında basınca HIGH varsaydık (Embassy'nin kendi G4 örneği de böyle). Logdaki `raw=`
  değeri butona basınca `true` olmalı. Tersse `bsp.rs` içinde sadece `BUTTON_ACTIVE_LEVEL` değişir.
- [ ] **170 MHz:** loglardaki zaman damgaları gerçek saniyeyle uyuşmalı.
- [ ] **Watchdog:** `thread1Hz` içine geçici bir sonsuz döngü koyup kartın ~0,5 s sonra resetlendiğini görmek.
- [ ] **Yük:** `load_centi_percent` çok düşük (%1'in altı) çıkmalı.

## 6. Sonraki adım önerileri

1. UART sürücüsü (`embedded-io-async`) ile C++'taki `terminal.printf("$a b c;")` formatını aynen kurmak.
2. Bir PWM çıkışı (LED parlaklığı) — C++'taki fan sürücüsünün Rust karşılığı için ilk adım.
3. Bir SPI sürücüsü — TMC5240'ın Rust versiyonuna giden yol; `Result` hata dönüşü burada gerçekten devreye girer.
4. Aynı uygulamayı Embassy'nin `async` görevleriyle (scheduler'sız) yazıp iki yaklaşımı karşılaştırmak.
