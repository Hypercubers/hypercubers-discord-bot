mod commands;
use std::env;

use std::time::Duration;
use tokio::{task, time};

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::framework::standard::{StandardFramework};
use serenity::model::gateway::Activity;

struct Handler;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let (embed_title, embed_main) =  match command.data.name.as_str() {
                "randfact" => ("Random Fact".to_string(), commands::randfact::run(&command.data.options)),
                "scramble" => ("Physical 2^4 Scrambles".to_string(), commands::phys24scram::run(&command.data.options)),
                &_ => todo!(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.embed(|e| {e.title(embed_title).description(embed_main)}))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::randfact::register(command))
                .create_application_command(|command| commands::phys24scram::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let lines = lines_from_file("statuses.txt");
        let num_lines = lines.len();
        let forever = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(10000));
            loop {
                interval.tick().await;
                let random_val:usize = rand::random::<usize>();
                ctx.set_activity(Activity::playing(lines[random_val % num_lines].to_string())).await;
            }
        });
    
        let _ = forever.await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let framework = StandardFramework::new();
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}