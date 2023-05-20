use tinyrand::Rand;
use tinyrand_std::ClockSeed;

use crate::Context;

// Function to create a seed, then return it.
pub async fn get_seed() -> u64 {
    let mut seed = ClockSeed::default();

    seed.next_u64()
}

// Simple function to simplify, and handle message sending.
pub async fn speak(context: Context<'_>, message: &str) {
    if let Err(reason) = context.say(message).await {
        println!(
            "An error occurred while trying to send message: {}, with reason: {}",
            message, reason
        );
    } else {
        println!("Message: {} | Successfully sent!", message);
    }
}
