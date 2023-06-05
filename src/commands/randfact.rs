use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

const RANDOM_FACTS:[&str;16] = ["The 3^4 has 216 stickers",
                "The 3^4 has 8 1c pieces, 24 2c pieces, 32 3c pieces, and 16 4c pieces",
                "The formula to calculate the number of states for the 3x3x3x3 is (24!x32!)&2 x 16!&2 x 2^23 x (3!)^31 x 3 x (4!&2)^15 x 4",
                "Melinda's Physical 2x2x2x2 is the only physical 4D puzzle that you can actually buy",
                "The 4th dimension in hypercubing programs is represented as inwards vs outwards, due to the projection",
                "Many standard 3x3x3 methods can be scaled up and used on the 3x3x3x3, such as Roux and CFOP",
                "There are 6 platonic solids in 4D: Simplex, Tesseract, 16-cell, 24-cell, 120-cell, 600-cell",
                "During 2022, Grant made 4 new physical 4D puzzles",
                "During 2022, the 3^4 speedsolving record was lowered by over 7 minutes, thanks to the Hyperspeedcube program's piece filters and keyboard controls",
                "RKT is a technique that lets you manipulate a single cell of the 3^4 like a 3^3",
                "There are theoretical designs for physical simplex, and many types of hypercuboids",
                "In 5D, Double RKT lets you manipulate a side like a cube by using RKT to manipulate that side like a tesseract",
                "The Hall Of Fame for 3^4 solutions was closed on December 7th, 2022 when it reached 500 entries",
                "There's no OLL parity in 4 dimensions (or higher)",
                "No one has ever done a speedsolve of a 5D puzzle",
                "In even dimensions, any sized cube can be checkerboarded, but in odd dimensions only oddxodd sized cubes can be fully checkerboarded"];

pub fn run(_options: &[CommandDataOption]) -> String {
    let mut rng = rand::thread_rng();
    RANDOM_FACTS[rng.gen_range(0..16)].to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("randfact").description("tells you a random fact")
}