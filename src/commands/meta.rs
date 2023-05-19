use rand::Rng;

use crate::{Context, Error};

enum Material {
    Bronze(&'static str),
}
const ARMOUR_LOOT: [&str; 5] = [
    "Bronze Helmet",
    "Bronze Chestplate",
    "Bronze Gauntlets",
    "Bronze Leggings",
    "Bronze Grieves",
];

// Fetch Armour Loot Table.
#[poise::command(prefix_command, slash_command)]
pub async fn fetch_armour(
    context: Context<'_>,
    #[description = "Fetch a randomly selected item from Loot Table of type 'armour'"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    _command: Option<String>,
) -> Result<(), Error> {
    let armour = ARMOUR_LOOT[rand::thread_rng().gen_range(0..5)];
    context.say(format!("{:?}", armour)).await?;
    Ok(())
}

// Show help menu.
#[poise::command(prefix_command, track_edits, slash_command)]
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

/// Vote for something
///
/// Enter `~vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    context: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = context.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    context.say(response).await?;
    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// ~getvotes
/// ~getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    context: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *context
            .data()
            .votes
            .lock()
            .unwrap()
            .get(&choice)
            .unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        context.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in context.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        context.say(response).await?;
    };

    Ok(())
}
