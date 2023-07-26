use std::fmt::Write;
/// External crates brought into scope.
use tinyvec::*;

/// Local crates brought into scopes.
use crate::assets::{
    common::{gen_num, speak},
    equipment::*,
    loot_tables::*,
};
use crate::fetch_subcommand;
use crate::{Context, Error};

/// Parent command of various subcommands with similar function.
#[poise::command(
    slash_command,
    member_cooldown = 2,
    subcommands(
        "armour",
        "weapon",
        "elixir",
        "generic",
        "coin",
        "condition",
        "tincture"
    )
)]
pub(crate) async fn fetch(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fetch_subcommand!(
    /// Grab a randomized amount of coins.
    pub(crate) async fn coin
    #[description = "Maximum amount of coins."]
    #[min = 1_usize]
    #[max = 1000_usize]
    limit: Option<usize>
    let => result: usize,
    gen_num(0, limit.unwrap_or(20)).await;
);

fetch_subcommand!(
    /// Grab Armour(s) from a table.
    pub(crate) async fn armour
    #[description = "Maximum amount of rolls."]
    #[min = 1_usize]
    #[max = 10_usize]
    count: Option<usize>
    let mut => armours: TinyVec<[Armour; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        armours.push(ARMOUR_LOOT[gen_num(0, ARMOUR_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Weapon(s) from a table.
    pub(crate) async fn weapon
    #[description = "Maximum amount of rolls."]
    #[min = 1_usize]
    #[max = 10_usize]
    count: Option<usize>
    let mut => weapons: TinyVec<[Weapon; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        weapons.push(WEAPON_LOOT[gen_num(0, WEAPON_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Elixir(s) from a table.
    pub(crate) async fn elixir
    #[description = "Maximum amount of rolls."]
    #[min = 1_usize]
    #[max = 10_usize]
    count: Option<usize>
    let mut => elixirs: TinyVec<[Elixir; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        elixirs.push(ELIXIR_LOOT[gen_num(0, ELIXIR_LOOT.len()).await]);
    }
);

fetch_subcommand!(
    /// Grab Tincture(s) from a table.
    pub(crate) async fn tincture
    #[description = "Maximum amount of rolls."]
    #[min = 1_usize]
    #[max = 10_usize]
    count: Option<usize>
    let mut => tinctures: TinyVec<[Tincture; 20]>,
    tiny_vec!();
    for _ in 0..count.unwrap_or(1) {
        tinctures.push(TINCTURE_LOOT[gen_num(0, TINCTURE_LOOT.len()).await]);
    }
);

/// Grab randomized loot from a table.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn generic(
    context: Context<'_>,
    #[description = "Get generic loot from tables with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut loot = String::new();

    for _ in 0..count.unwrap_or(1) {
        let table = gen_num(0, 4).await;
        match table {
            0 => write!(
                loot,
                "{}, ",
                ARMOUR_LOOT[gen_num(0, ARMOUR_LOOT.len()).await]
            )?,
            1 => write!(
                loot,
                "{}, ",
                WEAPON_LOOT[gen_num(0, WEAPON_LOOT.len()).await]
            )?,
            2 => write!(
                loot,
                "{}, ",
                ELIXIR_LOOT[gen_num(0, ELIXIR_LOOT.len()).await]
            )?,
            3 => write!(
                loot,
                "{}, ",
                TINCTURE_LOOT[gen_num(0, TINCTURE_LOOT.len()).await]
            )?,
            _ => write!(loot, "Error")?,
        }
    }

    speak(context, &format!("{loot:#?}")).await;

    Ok(())
}

/// Simple command to roll a die.
#[poise::command(slash_command, member_cooldown = 2)]
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
    #[description = "Modifier to be applied to rolls."]
    #[min = -100_isize]
    #[max = 100_isize]
    modifier: Option<isize>,
) -> Result<(), Error> {
    let mut results: TinyVec<[usize; 128]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        results.push(gen_num(0, sides.unwrap_or(20)).await)
    }

    if modifier.is_none() {
        if sum.unwrap_or(false) {
            speak(context, &format!("{:?}", results.iter().sum::<usize>())).await;
        } else {
            speak(context, &format!("{results:?}")).await;
        }
    } else if sum.unwrap_or(false) {
        let modified_results = results
            .iter()
            .map(|roll| *roll as isize + modifier.unwrap())
            .collect::<TinyVec<[isize; 128]>>();
        speak(
            context,
            &format!("{:?}", modified_results.iter().sum::<isize>()),
        )
        .await;
    } else {
        let modified_results = results
            .iter()
            .map(|roll| *roll as isize + modifier.unwrap())
            .collect::<TinyVec<[isize; 128]>>();
        speak(context, &format!("{modified_results:?}")).await;
    }
    Ok(())
}

/// Grab randomized conditions for successful dice roll.
#[poise::command(slash_command, owners_only, member_cooldown = 2)]
pub(crate) async fn condition(
    context: Context<'_>,
    #[description = "If a Threshold should be required. Default is True."] threshold: Option<bool>,
    #[description = "If a Parity should be required. Default is False."] parity: Option<bool>,
    #[description = "Limits lower Threshold amount for result. Default is 0."]
    #[min = 1_usize]
    #[max = 100_usize]
    floor: Option<usize>,
    #[description = "Limits upper Threshold amount for result. Default is 20."]
    #[min = 1_usize]
    #[max = 100_usize]
    ceiling: Option<usize>,
) -> Result<(), Error> {
    let threshold_amount = if floor.unwrap_or(0) > ceiling.unwrap_or(20) {
        speak(
            context,
            "The floor can't be greater than the ceiling, silly!",
        )
        .await;
        gen_num(0, 20).await
    } else {
        gen_num(floor.unwrap_or(0), ceiling.unwrap_or(20)).await
    };

    let parity_type = match gen_num(0, 2).await {
        0 => "even",
        1 => "odd",
        _ => "Error",
    };

    if threshold.unwrap_or(true) && !parity.unwrap_or(false) {
        speak(
            context,
            &format!(
                "You must reach/surpass a threshold of **{}**!",
                threshold_amount
            ),
        )
        .await;
    } else if parity.unwrap_or(false) && !threshold.unwrap_or(true) {
        speak(
            context,
            &format!("You must have an **{}** parity!", parity_type),
        )
        .await
    } else if parity.unwrap_or(false) && threshold.unwrap_or(true) {
        speak(
            context,
            &format!(
                "You must reach/surpass a threshold of **{}** and have an **{}** parity!",
                threshold_amount, parity_type
            ),
        )
        .await
    } else {
        speak(context, "You're so silly! I can't do nothing!").await
    }

    Ok(())
}

/// Lists off available commands for Ayuyan.
#[poise::command(track_edits, slash_command, member_cooldown = 2)]
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
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn ping(context: Context<'_>) -> Result<(), Error> {
    speak(context, "Pong!").await;
    Ok(())
}
