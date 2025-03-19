use std::fs::File;
use std::io::{BufReader, BufRead};

use colored::Colorize;
use serenity::all::{Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseFollowup, Permissions, UserId};

pub fn register () -> CreateCommand{

    CreateCommand::new("load")
    .description("Loads the current output.txt file")
    .default_member_permissions(Permissions::ADMINISTRATOR)

}

pub async fn run (ctx: &Context, interaction: &CommandInteraction) {

    let _ = interaction.defer(&ctx.http).await;

    let current_guild = interaction.guild_id.unwrap();

    match File::open("output.txt") {

        Ok(file) => {
            
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {

            match line {
                Ok(line) => {
                    
                    let data: Vec<&str> = line.split(", ").collect();
                    let user = UserId::new(match data[0].parse::<u64>() {
                        Ok(number) => number,
                        Err(_) => 0
                    });

                    let ban = current_guild.ban_with_reason(&ctx.http, user, 0, data[1]).await;

                    match ban {
                        Ok(_) => {

                            println!(
                                "{} {}",
                                format!("[BANNED]").bright_red().bold(),
                                format!(" {}", data[0]).bright_red()
                            );

                            count += 1;

                        }
                        Err(e) => {
                            println!("Failed to ban {} : {}", data[0], e);
                        }
                    }

                }
                Err(_) => println!("err")
            }

        }

        let embed = CreateEmbed::new()
            .title("✅ SUCCESS")
            .description(format!("```Succesfully banned {} users```", count))
            .color(Colour::from_rgb(245, 91, 91));

        let response = CreateInteractionResponseFollowup::new().embed(embed);
        let msg = interaction.create_followup(&ctx.http, response).await;

        if let Err(e) = msg {
            println!("Failed to respond to fetch command: {}", e)
        }

        }

        Err(e) => {

            println!("Failed to load file {}", e);

            let embed = CreateEmbed::new()
            .title("😟 ERR")
            .description(format!("```Failed to load output.txt```"))
            .color(Colour::from_rgb(245, 91, 91));

            let response = CreateInteractionResponseFollowup::new().embed(embed);
            let msg = interaction.create_followup(&ctx.http, response).await;

            if let Err(e) = msg {
                println!("Failed to respond to fetch command: {}", e)
            }
        }

    };

}
