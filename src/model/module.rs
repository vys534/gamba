pub enum Class {
    Core,
    GetItTwisted,
}

pub struct Info {
    pub name: &'static str,
    pub description: &'static str,
}

impl Class {
    pub fn info(&self) -> Info {
        match self {
            Class::Core => Info {
                name: "Core",
                description: "Core commands"
            },
            Class::GetItTwisted => Info {
                name: "Chinese gambling simulator",
                description: "🦍 🗣 GET IT TWISTED 🌪 , GAMBLE ✅ . PLEASE START GAMBLING 👍 . GAMBLING IS AN INVESTMENT 🎰 AND AN INVESTMENT ONLY 👍 . YOU WILL PROFIT 💰 , YOU WILL WIN ❗ ️. YOU WILL DO ALL OF THAT 💯 , YOU UNDERSTAND ⁉ ️ YOU WILL BECOME A BILLIONAIRE 💵 📈 AND REBUILD YOUR FUCKING LIFE 🤯"
            }
        }
    }
}
