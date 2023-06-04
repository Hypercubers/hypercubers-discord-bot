use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};


const RMOVES:[&str;24] = ["Ry", "Ry'", "Ry2", "Rx2", "Rz2", "Rx2,y", "Rx2,y'", "Rx", "Rx,y", "Rx,y'", "Rx,y2", "Rx'", "Rx',y", "Rx',y'", "Rx',y2", "Rz", "Rz,y", "Rz,y'", "Rz,y2", "Rz'", "Rz',y", "Rz',y'", "Rz',y2", ""];
const LMOVES:[&str;24] = ["Ly", "Ly'", "Ly2", "Lx2", "Lz2", "Lx2,y", "Lx2,y'", "Lx", "Lx,y", "Lx,y'", "Lx,y2", "Lx'", "Lx',y", "Lx',y'", "Lx',y2", "Lz", "Lz,y", "Lz,y'", "Lz,y2", "Lz'", "Lz',y", "Lz',y'", "Lz',y2", ""];

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options.get(0)
    .expect("Expected user option")
    .resolved
    .as_ref()
    .expect("Expected user object");
    let mut rng = rand::thread_rng();
    let mut scram:String = "".to_string();  
    if let CommandDataOptionValue::Integer(value) = option {
        for j in 0..*value {
            scram = (scram + j.to_string().as_str()) + ". ";
            let length = 12 + rng.gen_range(0..3) * 2;
            for i in 0..length {
                if i != 0 {
                    scram += " # "
                }
                scram = ((scram + RMOVES[rng.gen_range(0..24)]) + " ") + LMOVES[rng.gen_range(0..24)];
            }
            scram += "\n";
        }
    }
    else {
        scram = "invalid input".to_string();
    }
    scram
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("scramble")
            .description("generates physical 2^4 scrambles")
            .create_option(|option| {option
                .name("number")
                .description("Number of scrambles to be generated")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(12)
                .required(true)
    })
}