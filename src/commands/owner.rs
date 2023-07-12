use crate::assets::common::speak;
use crate::{Context, Error};

/// Shutdown Ayuyan for update, or maintenance.
/// ***Owner only***
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub(crate) async fn rest(context: Context<'_>) -> Result<(), Error> {
    speak(
        context,
        "It always ends the same way... With me observing...",
    )
    .await;
    context
        .framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}
