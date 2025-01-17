use tinyrand::{Rand, RandRange, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::Context;

/// Function to create a seed, then return it.
pub(crate) async fn gen_num(floor: usize, ceiling: usize) -> usize {
    let mut seed = ClockSeed;

    let mut random_num = StdRand::seed(seed.next_u64());

    random_num.next_range(floor..ceiling)
}

/// Function to simplify, and handle message sending.
pub(crate) async fn speak(context: Context<'_>, message: &str) {
    if let Err(reason) = context.say(message).await {
        println!(
            "An error occurred while trying to send message: {}, with reason: {}",
            message, reason
        );
    } else {
        println!("Message: {} | Successfully sent!", message);
    }
}

/// Function to fetch user input.
pub(crate) async fn _intake(context: Context<'_>, prompt: &str, err_response: &str) -> String {
    speak(context, prompt).await;
    if let Some(answer) = context
        .author()
        .await_reply(context)
        .timeout(std::time::Duration::from_secs(10))
        .await
    {
        answer.content.to_string()
    } else {
        eprintln!("An error occurred trying to fetch intake!");
        speak(context, err_response).await;
        String::from("Noop")
    }
}

#[macro_export]
/// Macro used to define several functions with very similar logic.
///
/// These parameters are in the order that they must be put in.
/// - `attributes` is used to insert attributes.
/// - `privacy` & `func` are for defining our function's privacy and name.
/// - `parameter` contains a parameter name for our function.
/// - `declaration` is used to hold keywords such as `let` or `const`
///     - `storage` & `storage_type` hold a value name and its type.
///     - `definition` is the expression to define `storage`'s value.
/// - `loop` contains a loop expression.
macro_rules! fetch_subcommand {
    (
        $(#[$attributes:meta])*
        $privacy:vis async fn $func:ident
        $(#[$inner_attributes:meta])*
        $parameter:ident: Option<usize>
        $($declaration:ident)+ => $storage:ident: $storage_type:ty,
        $definition:expr;
        $($loop:expr)?
    ) => {
        $(#[$attributes])*
        #[poise::command(slash_command, member_cooldown = 2)]
        $privacy async fn $func(
            context: Context<'_>,
            $(#[$inner_attributes])*
            $parameter: Option<usize>) -> Result<(), Error> {
            $($declaration)+ $storage: $storage_type = $definition;

            $($loop)?;

            speak(context, &format!("{}", $storage)).await;
            Ok(())
        }
    };
}
