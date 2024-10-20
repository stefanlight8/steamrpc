use discord_rich_presence::activity::{Activity, Timestamps};

pub fn get_default_activity<'a>() -> Activity<'a> {
    return Activity::new()
        .state("No current game")
        .timestamps(Timestamps::new());
}