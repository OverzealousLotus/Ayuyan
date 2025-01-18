use rand_simple::{Uniform, generate_seeds};

use crate::Context;

/// Function to create a random number, then return it.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // This is fine :)
pub(crate) async fn gen_num(floor: usize, ceiling: usize) -> usize {
    let seed: u32 = generate_seeds!(2)[1];
    let mut dist = Uniform::new(seed);
    dist.try_set_params(floor as f64, ceiling as f64).expect("Failed to set range.");
    let random_num = dist.sample().round();

    random_num as usize
}

/// Function to simplify, and handle message sending.
pub(crate) async fn speak(context: Context<'_>, message: &str) {
    if let Err(reason) = context.say(message).await {
        println!("An error occurred while trying to send message: {message}, with reason: {reason}");
    } else {
        println!("Message: {message} | Successfully sent!");
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
        $name:ident
        $(#[$inner_attributes:meta])*
        $parameter:ident: Option<usize>
        $($declaration:ident)+ => $storage:ident: $storage_type:ty,
        $definition:expr;
        $($cycle:expr)+
    ) => {
        $(#[$attributes])*
        #[poise::command(slash_command, member_cooldown = 2)]
        pub(crate) async fn $name(
            context: Context<'_>,
            $(#[$inner_attributes])*
            $parameter: Option<usize>) -> Result<(), Error> {
            $($declaration)+ $storage: $storage_type = $definition;

            $($cycle)+;

            speak(context, &format!("{}", $storage)).await;
            Ok(())
        }
    };
}
