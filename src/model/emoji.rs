pub enum Emoji {
    Cp,
    EpicFail,
    Moosh,
    Fabio,
    Reaper,
    Troll,
    WhatsappSpencer,
    Whatsapp,
    WhatsappGold,
    Fire,
    TheDollar,
    PackOLantern,
}

impl std::fmt::Display for Emoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Emoji::Cp => "<:points:1098027068152873092>",
            Emoji::EpicFail => "<:fail:1098060687101673552>",
            Emoji::Moosh => "<:moosher:1098433526141046825>",
            Emoji::Fabio => "<:fabio:1098433048518860800>",
            Emoji::Reaper => "<:reaper:1098433046480425070>",
            Emoji::Troll => "<:troll:1098433050662162532>",
            Emoji::WhatsappSpencer => "<:wappspencer:1098434636721422346>",
            Emoji::Whatsapp => "<:wapp:1098433043175313448>",
            Emoji::WhatsappGold => "<:wappgold:1098433045339586601>",
            Emoji::Fire => ":fire:",
            Emoji::TheDollar => ":dollar:",
            Emoji::PackOLantern => "<:packolantern:1110735361644183643>",
        };
        write!(f, "{}", s)
    }
}
