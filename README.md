# Nucleo-G474RE — Rust ile Buton + LED

Nucleo-G474RE kartında **B1** butonuna basınca **LD2** LED'i yanan, Rust + [Embassy](https://embassy.dev) ile yazılmış
örnek firmware. Kod, C++ gömülü projelerimizde kullandığımız katmanlı yapı ve 400 Hz işbirlikçi zamanlayıcı stiliyle
yazıldı. C++ ile farkların basit dille anlatımı: **[RAPOR.md](RAPOR.md)**.

## Donanım

| | Pin | Not |
|---|---|---|
| LD2 (yeşil LED) | PA5 | HIGH = yanık |
| B1 (mavi buton) | PC13 | Basınca HIGH, dahili pull-down |
| Bağlantı | USB (ST-LINK) | Flash + log aynı kablodan |

## Kurulum (Windows, bir kez)

```powershell
# 1) Rust (kuruluysa atla)
winget install Rustlang.Rustup
rustup target add thumbv7em-none-eabihf

# 2) probe-rs (flash + RTT log)
powershell -ExecutionPolicy Bypass -c "irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex"
```

`cargo` komutu bulunamazsa `%USERPROFILE%\.cargo\bin` klasörünü PATH'e ekle.

## Çalıştırma

```powershell
cargo run --release      # derle + karta yükle + logu göster
cargo build --release    # sadece derle
```

Beklenen log:

```
INFO  nucleo-g474re-button-led v0.1.0.1
INFO  $alive led=false presses=0 raw=false LoadReport { load_centi_percent: …, max_tick_us: …, overruns: 0 }
INFO  button pressed -> LED on
INFO  button released -> LED off
```

## Klasör yapısı

```
src/
├── main.rs              Giriş: kartı başlat, zamanlayıcıyı çalıştır
├── bsp.rs               Kart: saat (170 MHz), pinler, watchdog — pin numarası bilen TEK dosya
├── scheduler.rs         400 Hz zamanlayıcı (thread400Hz … thread1Hz), MCU yükü, watchdog besleme
├── app/indicator.rs     Uygulama kuralı: buton basılı → LED yanık
├── drivers/led.rs       LED sürücüsü (polarite içeride)
├── drivers/button.rs    Buton sürücüsü (3 ardışık okuma = 30 ms debounce)
└── common/
    ├── version.rs       Sürüm (major.minor.patch.suffix)
    └── mcu_timer.rs     Tick süresi, MCU yükü, bütçe aşımı sayacı
```

Dokümantasyon: `cargo doc --open`.

## Sürümler

| Paket | Sürüm |
|---|---|
| Rust | 1.100 (stable) |
| embassy-stm32 | 0.6.0 (`stm32g474re`, `dual-bank`) |
| embassy-executor | 0.10.0 |
| embassy-time | 0.5.1 |
| defmt / defmt-rtt | 1.2 / 1.3 |

Değişiklik geçmişi: [CHANGELOG.md](CHANGELOG.md).
