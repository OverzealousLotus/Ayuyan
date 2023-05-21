use crate::{Context, Error};

/// Shutdown Ayuyan for update, or maintenance.
/// ***Owner only***
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub(crate) async fn shutdown(context: Context<'_>) -> Result<(), Error> {
    context.say("I am inevitable...").await?;
    context
        .framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}
