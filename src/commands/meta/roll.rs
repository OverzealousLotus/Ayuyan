use crate::{Context, Error};
use crate::assets::common::{gen_num, speak};

use tinyvec::{TinyVec, tiny_vec};


/// Rolls a die or dice upon user request.
#[poise::command(slash_command, member_cooldown = 2)]
pub(crate) async fn roll(
    context: Context<'_>,
    #[description = "Times die will be rolled."]
    #[max = 100_usize]
    count: Option<usize>,
    #[description = "Number of sides on die."]
    #[max = 200_usize]
    sides: Option<usize>,
    #[description = "Specify if rolls should be summed up."] summable: Option<bool>,
    #[description = "Modifier to be applied to rolls."] modifier: Option<i8>,
) -> Result<(), Error> {
    let mut results: TinyVec<[usize; 128]> = tiny_vec!();
    let summable = summable.unwrap_or(false);

    for _ in 0..count.unwrap_or(1) {
        results.push(gen_num(0, sides.unwrap_or(20)).await);
    }

    println!("Roll Count: {count:?}, Sides: {sides:?}, Summation Enabled: {summable:?}, Modifier: {modifier:?}");
    println!("Untouched roll(s): {results:?}");
    match modifier {
        None if summable => {
            speak(context, &format!("{:?}", results.iter().sum::<usize>())).await;
        },
        None if !summable => {
            speak(context, &format!("{results:?}")).await;
        },
        Some(modifier) => {
            modify_results(context, results, modifier.into(), summable).await;
        },
        None => todo!()
    }
    Ok(())
}

#[allow(clippy::cast_possible_wrap)] // Roll will never exceed isize/usize maximum.
async fn modify_results(context: Context<'_>, results: TinyVec<[usize; 128]>, modifier: isize, summable: bool) {
    let new_results: TinyVec<[isize; 128]>;
    if summable {
        new_results = results
            .iter()
            .map(|roll| *roll as isize + modifier)
            .collect::<TinyVec<[isize; 128]>>();
        speak(context,&format!("{:?}", new_results.iter().sum::<isize>())).await;
    } else {
        new_results = results
        .iter()
        .map(|roll| *roll as isize + modifier)
        .collect::<TinyVec<[isize; 128]>>();
        speak(context, &format!("{new_results:?}")).await;
    }
}