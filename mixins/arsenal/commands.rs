// The command Arsenal adds: /arsenal.
fn arsenal_commands(game: &crate::game::Game<'_>, commands: &mut Vec<&'static str>) {
    use crate::game::arsenal_native as m;
    if game.native_on(m::ID) {
        commands.push("arsenal");
    }
}
