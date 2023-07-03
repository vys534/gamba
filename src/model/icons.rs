use crate::model::emoji::Emoji;

pub enum Icon {
    Moosh,
    Fabio,
    Reaper,
    Troll,
    WhatsappSpencer,
    Whatsapp,
    WhatsappGold,
    TheDollar,
    JointOLantern,
}

impl Icon {
    pub fn emoji(&self) -> Emoji {
        match *self {
            Icon::Moosh => Emoji::Moosh,
            Icon::Fabio => Emoji::Fabio,
            Icon::Reaper => Emoji::Reaper,
            Icon::Troll => Emoji::Troll,
            Icon::WhatsappSpencer => Emoji::WhatsappSpencer,
            Icon::Whatsapp => Emoji::Whatsapp,
            Icon::WhatsappGold => Emoji::WhatsappGold,
            Icon::TheDollar => Emoji::TheDollar,
            Icon::JointOLantern => Emoji::PackOLantern,
        }
    }
    pub fn name(&self) -> &'static str {
        match *self {
            Icon::Moosh => "Moosh",
            Icon::Fabio => "Fabio",
            Icon::Reaper => "Reaper",
            Icon::Troll => "Troll",
            Icon::WhatsappSpencer => "Whatsapp Spencer",
            Icon::Whatsapp => "Whatsapp",
            Icon::WhatsappGold => "Whatsapp Gold",
            Icon::TheDollar => "the dollar",
            Icon::JointOLantern => "joint-o-lantern",
        }
    }
    // .0 = individual value, .1 = match 3
    pub fn match_data(&self) -> (f64, f64) {
        match *self {
            Icon::Moosh => (0.0, 0.0),
            Icon::Fabio => (0.1, 0.5),
            Icon::Reaper => (0.2, 1.0),
            Icon::TheDollar => (0.3, 0.0),
            Icon::Troll => (0.4, 1.8),
            Icon::WhatsappSpencer => (0.5, 0.0),
            Icon::JointOLantern => (0.6, 3.0),
            Icon::Whatsapp => (0.7, 3.8),
            Icon::WhatsappGold => (1.2, 4.5),
        }
    }
}
