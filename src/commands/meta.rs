/// External crates brought into scope.
use tinyrand::{Rand, Seeded, StdRand};
use tinyvec::*;

/// Local crates brought into scopes.
use crate::assets::{
    common::{get_seed, speak},
    equipment::{Armour, Material, Weapon},
    loot_tables::{ARMOUR_LOOT, WEAPON_LOOT},
};
use crate::{Context, Error};

/// Fetches x amount of pieces of y.
/// If left empty, fetches only once.
#[poise::command(slash_command, subcommands("armour", "weapon"))]
pub(crate) async fn fetch(context: Context<'_>) -> Result<(), Error> {
    speak(context, "Simple subcommand test for Ayuyan.").await;
    Ok(())
}

/// Subcommand of `fetch` to get armour instead of weapons.
#[poise::command(slash_command)]
pub(crate) async fn armour(
    context: Context<'_>,
    #[description = "Get Armour from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    roll_count: Option<usize>,
) -> Result<(), Error> {
    let mut armours: TinyVec<[Material<Armour>; 20]> = tiny_vec!();

    for _ in 0..roll_count.unwrap_or(1) {
        let mut rand = StdRand::seed(get_seed().await);
        armours.push(ARMOUR_LOOT[rand.next_lim_usize(ARMOUR_LOOT.len())]);
    }

    speak(context, format!("{:?}", armours).as_str()).await;

    Ok(())
}

/// Subcommmand of `fetch` to get weapons instead of armour.
#[poise::command(slash_command)]
pub(crate) async fn weapon(
    context: Context<'_>,
    #[description = "Get Weapon from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut weapons: TinyVec<[Material<Weapon>; 20]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        let mut rand = StdRand::seed(get_seed().await);
        weapons.push(WEAPON_LOOT[rand.next_lim_usize(WEAPON_LOOT.len())]);
    }

    speak(context, format!("{:?}", weapons).as_str()).await;

    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn roll(
    context: Context<'_>,
    #[description = "Times die will be rolled."]
    #[min = 1_usize]
    #[max = 100_usize]
    count: Option<usize>,
    #[description = "Number of sides on die."]
    #[min = 1_usize]
    #[max = 200_usize]
    sides: Option<usize>,
    #[description = "Specify if rolls should be summed up."] sum: Option<bool>,
) -> Result<(), Error> {
    let mut results: TinyVec<[usize; 128]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        let mut rand_num = StdRand::seed(get_seed().await);
        results.push(rand_num.next_lim_usize(sides.unwrap_or(20)))
    }

    if sum.unwrap_or(false) == true {
        speak(
            context,
            format!("{:?}", results.into_iter().sum::<usize>()).as_str(),
        )
        .await;
    } else {
        speak(context, format!("{:?}", results).as_str()).await;
    }

    Ok(())
}

/// Lists off available commands for Ayuyan.
#[poise::command(track_edits, slash_command)]
pub(crate) async fn help(
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

/// Simple command to check to see if Ayuyan is online.
#[poise::command(slash_command)]
pub(crate) async fn ping(context: Context<'_>) -> Result<(), Error> {
    context.say("Pong!").await?;
    Ok(())
}
