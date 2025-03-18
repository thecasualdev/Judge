use std::vec;
use serenity::{all::{ClientBuilder, Context, EventHandler, GuildId, GatewayIntents, Interaction, Ready}, async_trait};

mod commands;

struct Handler;
#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, ready: Ready) {

        println!("🚀 {} is online!", ready.user.name);

        let guild_id = GuildId::new(
            dotenv::var("GUILD_ID")
            .unwrap()
            .parse::<u64>()
            .expect("Guild is not a integer.")
        );

        let commands = guild_id.set_commands(
            &ctx.http,
            vec![
               commands::ping::register()
            ]
        ).await.unwrap();

        println!("💡 Successfully registered the following commands: {}", commands.into_iter().map(|c| c.name).collect::<Vec<String>>().join(", "));

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            match cmd.data.name.as_str() {
                "ping" =>{
                    commands::ping::run(&ctx, &cmd).await;
                }
                _ => {}
            }
        }
    }

}

#[tokio::main]
async fn main() {

    let discord_token: String = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES;
    let mut client = ClientBuilder::new(&discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client.");

    if let Err(e) = client.start().await {
        println!("Failed to initialize client: {}", e)
    }

}
