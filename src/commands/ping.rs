use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseFollowup};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
    .description("Pings the bot!")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) {

    let _ = interaction.defer_ephemeral(&ctx.http).await;

    let response = CreateInteractionResponseFollowup::new().content("Pong! 🏓");
    let msg = interaction.create_followup(&ctx.http, response).await;

    if let Err(e) = msg {
        println!("Failed to respond  to ping command: {}", e)
    }

}