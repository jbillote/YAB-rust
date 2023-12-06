use serenity::{
    builder::{CreateCommand, CreateCommandOption},
    model::application::{CommandOptionType, ResolvedOption, ResolvedValue},
};

pub fn run(options: &[ResolvedOption]) -> String {
    let character = if let Some(ResolvedOption {
        value: ResolvedValue::String(character),
        ..
    }) = options.first()
    {
        character.to_string()
    } else {
        "shouldn't happen".to_string()
    };

    if let Some(ResolvedOption {
        value: ResolvedValue::String(input),
        ..
    }) = options.get(1)
    {
        return format!("{},{}", character, input);
    } else {
        return character;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mbtl")
        .description("Get MBTL move information")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "character", "Character name")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "input", "Move input")
                .required(false),
        )
}
