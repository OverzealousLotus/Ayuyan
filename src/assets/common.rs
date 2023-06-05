use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::ClockSeed;

use crate::Context;

// Function to create a seed, then return it.
pub(crate) async fn gen_num(limit: usize) -> usize {
    let mut seed = ClockSeed::default();

    let mut random_num = StdRand::seed(seed.next_u64());

    random_num.next_lim_usize(limit)
}

// Simple function to simplify, and handle message sending.
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
