use crate::models::attack::Attack;

use reqwest::Error;
use serenity::{
    builder::{CreateCommand, CreateCommandOption, CreateEmbed},
    model::application::{CommandOptionType, ResolvedOption, ResolvedValue},
};

pub async fn run(options: &[ResolvedOption<'_>]) -> CreateEmbed {
    let character = if let Some(ResolvedOption {
        value: ResolvedValue::String(character),
        ..
    }) = options.first()
    {
        character.to_string()
    } else {
        "".to_string()
    };

    let input = if let Some(ResolvedOption {
        value: ResolvedValue::String(input),
        ..
    }) = options.get(1)
    {
        input.to_string()
    } else {
        "".to_string()
    };

    let attack: Attack = get_attack(character, input).await;
    let embed = CreateEmbed::new()
        .title(attack.name)
        .fields(vec![
            ("Input", attack.input, true),
            ("", "".to_string(), true),
            ("", "".to_string(), true),
            ("Damage", attack.damage, true),
            ("Block", attack.block, true),
            ("Cancel", attack.cancel, true),
            ("Property", attack.property, true),
            ("Cost", attack.cost, true),
            ("Attribute", attack.attribute, true),
            ("Startup", attack.startup, true),
            ("Active", attack.active, true),
            ("Recovery", attack.recovery, true),
            ("Overall", attack.overall, true),
            ("Advantage", attack.advantage, true),
            ("Invuln", attack.invuln, true), 
        ]);

    return embed;
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
                .required(true),
        )
}

async fn get_attack(character: String, input: String) -> Attack {
    let url = format!(
        "http://127.0.0.1:8080/character/{character}/{input}",
        character = character,
        input = input
    );
    let response = reqwest::get(&url).await.unwrap();
    let attack: Attack = response.json().await.unwrap();

    return attack;
}
