use serenity::all::{Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseFollowup, Timestamp};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
    .description("Pings the bot!")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {

    let interaction_time = interaction.id.created_at().timestamp_millis();
    let ping = Timestamp::now().timestamp_millis() - interaction_time;
    let _ = interaction.defer_ephemeral(&ctx.http).await;

    let embed = CreateEmbed::new()
        .description(format!("Pong! ~ ``{}ms`` 🏓", ping))
        .color(Colour::from_rgb(245, 91, 91));

    let response = CreateInteractionResponseFollowup::new().embed(embed);
    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond to ping command: {}", e)
    }

}