use tinyrand::{Rand, Seeded, StdRand};
use tinyvec::*;

use crate::assets::common::{get_seed, speak};
use crate::assets::equipment::{Armour, ARMOUR_LOOT};
use crate::{Context, Error};

// Fetch Armour Loot Table.
#[poise::command(slash_command)]
pub async fn fetch_armour(
    context: Context<'_>,
    #[description = "Get Armour from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    roll_count: Option<usize>,
) -> Result<(), Error> {
    let mut armours: TinyVec<[Armour; 20]> = tiny_vec!();

    for _ in 0..roll_count.unwrap_or(1) {
        let mut rand = StdRand::seed(get_seed().await);
        let armour = ARMOUR_LOOT[rand.next_lim_usize(35)];
        armours.push(armour);
    }

    speak(context, format!("{:?}", armours).as_str()).await;

    Ok(())
}

// Show help menu.
#[poise::command(track_edits, slash_command)]
pub async fn help(
    context: Context<'_>,
    #[description = "List commands for Ayuyan."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        context,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "Consider the following...",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

// Ping Ayuyan.
#[poise::command(slash_command)]
pub async fn ping(
    context: Context<'_>,
    #[description = "Is Ayuyan online?"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    _command: Option<String>,
) -> Result<(), Error> {
    context.say("Pong!").await?;
    Ok(())
}
