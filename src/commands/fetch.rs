use serenity::all::{Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand{
    CreateCommand::new("fetch")
    .description("Gets a count of blocked members.")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {

    let _ = interaction.defer(&ctx.http).await;

    let guild = &interaction.guild_id.unwrap();
    let fetch_bans = guild.bans(&ctx.http, None, None).await;

    let ban_count = match fetch_bans {
        Ok(bans) => {
            let count = bans.len();
            count
        }
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    let embed = CreateEmbed::new()
    .title("Fetched current ban count.")
    .description(format!("```There are currently {} banned users in the server.```", ban_count))
    .color(Colour::from_rgb(245, 91, 91));

    let response = CreateInteractionResponseFollowup::new().embed(embed);
    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond to fetch command: {}", e)
    }

}