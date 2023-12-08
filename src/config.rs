use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cycles_per_frame: Option<u32>,
    pub keys: Option<Keys>,
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    key0: Key,
    key1: Key,
    key2: Key,
    key3: Key,
    key4: Key,
    key5: Key,
    key6: Key,
    key7: Key,
    key8: Key,
    key9: Key,
    #[serde(rename = "keyA")]
    key_a: Key,
    #[serde(rename = "keyB")]
    key_b: Key,
    #[serde(rename = "keyC")]
    key_c: Key,
    #[serde(rename = "keyD")]
    key_d: Key,
    #[serde(rename = "keyE")]
    key_e: Key,
    #[serde(rename = "keyF")]
    key_f: Key,
}

impl Keys {
    pub fn into_arr(self) -> [minifb::Key; 16] {
        [
            self.key0.0,
            self.key1.0,
            self.key2.0,
            self.key3.0,
            self.key4.0,
            self.key5.0,
            self.key6.0,
            self.key7.0,
            self.key8.0,
            self.key9.0,
            self.key_a.0,
            self.key_b.0,
            self.key_c.0,
            self.key_d.0,
            self.key_e.0,
            self.key_f.0,
        ]
    }
}

#[derive(Debug)]
struct Key(minifb::Key);

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyVisitor;

        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an string that corresponsds to keycode")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use minifb::Key as K;
                let value = match v.to_lowercase().as_str() {
                    "0" => K::Key0,
                    "1" => K::Key1,
                    "2" => K::Key2,
                    "3" => K::Key3,
                    "4" => K::Key4,
                    "5" => K::Key5,
                    "6" => K::Key6,
                    "7" => K::Key7,
                    "8" => K::Key8,
                    "9" => K::Key9,
                    "a" => K::A,
                    "b" => K::B,
                    "c" => K::C,
                    "d" => K::D,
                    "e" => K::E,
                    "f" => K::F,
                    "g" => K::G,
                    "h" => K::H,
                    "i" => K::I,
                    "j" => K::J,
                    "k" => K::K,
                    "l" => K::L,
                    "m" => K::M,
                    "n" => K::N,
                    "o" => K::O,
                    "p" => K::P,
                    "q" => K::Q,
                    "r" => K::R,
                    "s" => K::S,
                    "t" => K::T,
                    "u" => K::U,
                    "v" => K::V,
                    "w" => K::W,
                    "x" => K::X,
                    "y" => K::Y,
                    "z" => K::Z,
                    "-" => K::Minus,
                    "=" => K::Equal,
                    "[" => K::RightBracket,
                    "]" => K::LeftBracket,
                    ";" => K::Semicolon,
                    "'" => K::Apostrophe,
                    "," => K::Comma,
                    "." => K::Period,
                    "/" => K::Slash,
                    "up" => K::Up,
                    "down" => K::Down,
                    "left" => K::Left,
                    "right" => K::Right,
                    _ => return Err(serde::de::Error::custom("Invalid key entry")),
                };
                Ok(Key(value))
            }
        }

        deserializer.deserialize_str(KeyVisitor)
    }
}
