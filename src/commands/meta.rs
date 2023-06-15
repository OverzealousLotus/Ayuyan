/// External crates brought into scope.
use tinyvec::*;

/// Local crates brought into scopes.
use crate::assets::{
    common::{gen_num, get_armour, get_elixir, get_weapon, speak},
    loot_tables::{ARMOUR_LOOT, ELIXIR_LOOT, WEAPON_LOOT},
};
use crate::serenity;
use crate::{Context, Error};

/// Fetches x amount of pieces of y.
/// If left empty, fetches only once.
#[poise::command(
    slash_command,
    member_cooldown = 2,
    subcommands("armour", "weapon", "elixir", "generic", "coin")
)]
pub(crate) async fn fetch(context: Context<'_>) -> Result<(), Error> {
    speak(context, "Simple subcommand test for Ayuyan.").await;
    Ok(())
}

/// Subcommand of `fetch` to get coins.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn coin(
    context: Context<'_>,
    #[description = "Limit of randomized coin count."]
    #[min = 1_usize]
    #[max = 1000_usize]
    limit: Option<usize>,
) -> Result<(), Error> {
    let result = gen_num(limit.unwrap_or(20)).await;

    speak(context, &format!("{result}")).await;
    Ok(())
}

/// Subcommand of `fetch` to get armour.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn armour(
    context: Context<'_>,
    #[description = "Get Armour from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    roll_count: Option<usize>,
) -> Result<(), Error> {
    let mut armours: TinyVec<[&str; 20]> = tiny_vec!();

    for _ in 0..roll_count.unwrap_or(1) {
        armours.push(get_armour(ARMOUR_LOOT[gen_num(ARMOUR_LOOT.len()).await]).await);
    }

    speak(context, format!("{armours:#?}").as_str()).await;

    Ok(())
}

/// Subcommmand of `fetch` to get weaponry.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn weapon(
    context: Context<'_>,
    #[description = "Get Weapon from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut weapons: TinyVec<[&str; 20]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        weapons.push(get_weapon(WEAPON_LOOT[gen_num(WEAPON_LOOT.len()).await]).await);
    }

    speak(context, format!("{weapons:#?}").as_str()).await;

    Ok(())
}

/// Subcommmand of `fetch` to get elixirs.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn elixir(
    context: Context<'_>,
    #[description = "Get Elixir from table with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut elixirs: TinyVec<[&str; 20]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        elixirs.push(get_elixir(ELIXIR_LOOT[gen_num(ELIXIR_LOOT.len()).await]).await);
    }

    speak(context, format!("{elixirs:#?}").as_str()).await;

    Ok(())
}

/// Subcommmand of `fetch` to get elixirs.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn generic(
    context: Context<'_>,
    #[description = "Get generic loot from tables with set roll count."]
    #[min = 1_usize]
    #[max = 20_usize]
    count: Option<usize>,
) -> Result<(), Error> {
    let mut loot: TinyVec<[&str; 20]> = tiny_vec!();

    for _ in 0..count.unwrap_or(1) {
        let table = gen_num(3).await;
        match table {
            0 => loot.push(get_armour(ARMOUR_LOOT[gen_num(ARMOUR_LOOT.len()).await]).await),
            1 => loot.push(get_weapon(WEAPON_LOOT[gen_num(WEAPON_LOOT.len()).await]).await),
            2 => loot.push(get_elixir(ELIXIR_LOOT[gen_num(ELIXIR_LOOT.len()).await]).await),
            _ => loot.push("Error"),
        }
    }

    speak(context, format!("{loot:#?}").as_str()).await;

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
        results.push(gen_num(sides.unwrap_or(20)).await)
    }

    if modifier.is_none() {
        if sum.unwrap_or(false) {
            speak(
                context,
                format!("{:?}", results.iter().sum::<usize>()).as_str(),
            )
            .await;
        } else {
            speak(context, format!("{results:?}").as_str()).await;
        }
    } else if sum.unwrap_or(false) {
        let modified_results = results
            .iter()
            .map(|roll| *roll as isize + modifier.unwrap())
            .collect::<TinyVec<[isize; 128]>>();
        speak(
            context,
            format!("{:?}", modified_results.iter().sum::<isize>()).as_str(),
        )
        .await;
    } else {
        let modified_results = results
            .iter()
            .map(|roll| *roll as isize + modifier.unwrap())
            .collect::<TinyVec<[isize; 128]>>();
        speak(context, format!("{modified_results:?}").as_str()).await;
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
    context.say("Pong!").await?;
    Ok(())
}

/// Testing out some poise features.
#[poise::command(prefix_command, track_edits, member_cooldown = 2)]
pub(crate) async fn boop(context: Context<'_>) -> Result<(), Error> {
    let uuid_boop = context.id();

    context
        .send(|message: &mut poise::CreateReply| {
            message.content("I want some boops!").components(|c| {
                c.create_action_row(|ar| {
                    ar.create_button(|b| {
                        b.style(serenity::ButtonStyle::Primary)
                            .label("Boop me!")
                            .custom_id(uuid_boop)
                    })
                })
            })
        })
        .await?;

    let mut boop_count = 0;
    while let Some(mci) = serenity::CollectComponentInteraction::new(context)
        .author_id(context.author().id)
        .channel_id(context.channel_id())
        .timeout(tokio::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid_boop.to_string())
        .await
    {
        boop_count += 1;

        let mut msg = mci.message.clone();
        msg.edit(context, |m| {
            m.content(format!("Boop count: {}", boop_count))
        })
        .await?;

        mci.create_interaction_response(context, |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}
