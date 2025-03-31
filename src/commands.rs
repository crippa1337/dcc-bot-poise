use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Collect a daily reward of tokens
#[poise::command(slash_command, prefix_command)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    let (reward, current_val) = {
        let uid = ctx.author().id.to_string();
        let mut hash = ctx.data().user_hash.lock().unwrap();
        let current_val = hash.entry(uid).or_default();
        let reward = fastrand::u64(250..500);
        *current_val += reward;
        (reward, *current_val)
    };

    let response =
        format!("You collected **{reward}** tokens! Your new balance is: **{current_val}**");
    ctx.say(response).await?;
    Ok(())
}
