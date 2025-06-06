use esp_idf_hal::gpio::{PinDriver, Output};
use esp_idf_hal::peripherals::Peripherals;
use std::collections::HashMap;
use anyhow::{Result, anyhow};

/// Перечисление сегментов семисегментного дисплея
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Segment {
    A, B, C, D, E, F, G,
}

/// Структура для управления одним сегментом
struct SegmentPin {
    pin: PinDriver<'static, Output>,
}

impl SegmentPin {
    fn set_on(&mut self) -> Result<()> {
        self.pin.set_high().map_err(|e| anyhow!("Failed to set pin high: {:?}", e))
    }

    fn set_off(&mut self) -> Result<()> {
        self.pin.set_low().map_err(|e| anyhow!("Failed to set pin low: {:?}", e))
    }
}

/// Управление одной цифрой табло через пины сегментов
struct DigitDisplay {
    segments: HashMap<Segment, SegmentPin>,
}

impl DigitDisplay {
    fn new(peripherals: &Peripherals, pins_map: &[(Segment, u8)]) -> Result<Self> {
        let mut segments = HashMap::new();
        for (segment, gpio_num) in pins_map {
            // Инициализируем пин, например gpio2, gpio4 и т.п.
            let pin = PinDriver::output(peripherals.pins.get(*gpio_num).ok_or_else(|| anyhow!("Pin not found"))?)?;
            segments.insert(*segment, SegmentPin { pin });
        }
        Ok(Self { segments })
    }

    /// Включить необходимые сегменты для отображения цифры 0-9
    fn display_digit(&mut self, digit: u8) -> Result<()> {
        let pattern = match digit {
            0 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F],
            1 => vec![Segment::B, Segment::C],
            2 => vec![Segment::A, Segment::B, Segment::D, Segment::E, Segment::G],
            3 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::G],
            4 => vec![Segment::B, Segment::C, Segment::F, Segment::G],
            5 => vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G],
            6 => vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G],
            7 => vec![Segment::A, Segment::B, Segment::C],
            8 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G],
            9 => vec![Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G],
            _ => return Err(anyhow!("Invalid digit")),
        };

        // Включаем нужные сегменты и выключаем остальные
        for (segment, seg_pin) in &mut self.segments {
            if pattern.contains(segment) {
                seg_pin.set_on()?;
            } else {
                seg_pin.set_off()?;
            }
        }
        Ok(())
    }
}

/// Основной контроллер табло с несколькими цифрами
pub struct DisplayController {
    digits: Vec<DigitDisplay>,
    // Если мультиплексирование — пины для выбора активной цифры
    // digit_select_pins: Vec<PinDriver<Output>>,
}

impl DisplayController {
    pub fn new(peripherals: &Peripherals) -> Result<Self> {
        // Пример: 4 цифры, каждая с одним набором пинов
        let digits_pins = vec![
            vec![(Segment::A, 2), (Segment::B, 4), (Segment::C, 5), (Segment::D, 18), (Segment::E, 19), (Segment::F, 21), (Segment::G, 22)],
            vec![(Segment::A, 23), (Segment::B, 25), (Segment::C, 26), (Segment::D, 27), (Segment::E, 32), (Segment::F, 33), (Segment::G, 34)],
            // и так далее для каждой цифры
        ];

        let mut digits = Vec::new();
        for pins_map in digits_pins {
            digits.push(DigitDisplay::new(peripherals, &pins_map)?);
        }

        Ok(DisplayController {
            digits,
            // digit_select_pins: vec![],
        })
    }

    /// Обновить весь дисплей с цифрами, например счёт "1234"
    pub fn display_score(&mut self, score: &str) -> Result<()> {
        for (i, ch) in score.chars().enumerate() {
            if i >= self.digits.len() {
                break;
            }
            let digit = ch.to_digit(10).ok_or_else(|| anyhow!("Invalid digit char"))? as u8;
            self.digits[i].display_digit(digit)?;
        }
        Ok(())
    }

    /// Обновление табло на основе пришедшего с сервера JSON (пример)
    pub fn update(&mut self, message: &str) -> Result<()> {
        // Пример: парсим JSON с ключом "score": "1234"
        let v: serde_json::Value = serde_json::from_str(message)?;
        if let Some(score) = v.get("score").and_then(|s| s.as_str()) {
            self.display_score(score)?;
        }
        // Аналогично для времени, событий и т.п.
        Ok(())
    }
}
