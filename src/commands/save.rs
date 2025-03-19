use std::fs::OpenOptions;
use std::io::Write;
use serenity::all::{Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseFollowup, Permissions};
use colored::Colorize;

pub fn register() -> CreateCommand {

    CreateCommand::new("save")
    .description("Saves the current bans to a local txt file")
    .default_member_permissions(Permissions::ADMINISTRATOR)

}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {

    let _ = interaction.defer(&ctx.http).await;

    let current_guild = interaction.guild_id.unwrap();
    let fetch_bans = current_guild.bans(&ctx.http, None, None).await;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("output.txt").unwrap();

    let banned = match fetch_bans {
        Ok(bans) => {

            let count = bans.len();

            for ban in bans {

                let reason = ban.reason.unwrap_or("No reason provided".to_string())
                .replace("\n", " ")
                .replace("\r", " ")
                .trim()
                .to_string();

                if let Err(e) = writeln!(file, "{}, {}", ban.user.id, reason) {
                    println!("{}", format!("Failed to write line: {}", e).red());
                }

                println!(
                    "{} {} {}",
                    format!("[{}]", ban.user.name).bright_red().bold(),
                    format!("{} -", ban.user.id).bright_red(),
                    format!("{}", reason).bright_red()
                );

            }

            count

        }
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    let embed = CreateEmbed::new()
        .title("💾 Saved!")
        .description(format!("```Succesfully saved {} banned users to output.txt```", banned))
        .color(Colour::from_rgb(245, 91, 91));

    let response = CreateInteractionResponseFollowup::new().embed(embed);
    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond to fetch command: {}", e)
    }

}
