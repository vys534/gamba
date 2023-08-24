pub enum Emoji {
    Credits,
    EpicFail,
    Moosh,
    Reddit,
    Discord,
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
            Emoji::Credits => "<:points:1098027068152873092>",
            Emoji::EpicFail => "<:fail:1098060687101673552>",
            Emoji::Moosh => "<:moosher:1098433526141046825>",
            Emoji::Reddit => "<:reddit:1144084645332402297>",
            Emoji::Discord => "<:discord:1144085223806599218>",
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
