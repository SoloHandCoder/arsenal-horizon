// /arsenal: Server Notes, Server Profiles and Crash Reports.
fn arsenal_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::arsenal_native as m;
    if *ran || !game.native_on(m::ID) {
        return;
    }
    let args = match command.strip_prefix("arsenal") {
        Some(rest) if rest.is_empty() || rest.starts_with(' ') => rest,
        _ => return,
    };
    *ran = true;
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    let raw = {
        let game = &*game;
        m::raw_settings(|key| game.native_setting(m::ID, key))
    };
    let server = game.arsenal_server();
    let answer = {
        let game = &*game;
        m::command(&settings, &raw, args, server.as_deref(), &|key| game.native_setting(m::ID, key))
    };
    let dir = game.native_data_dir(m::ID);
    match answer {
        m::Command::Say(text) => game.native_chat(&text),
        m::Command::Store(key, value, text) => {
            game.native_set_setting(m::ID, &key, &value);
            game.native_chat(&text);
        }
        m::Command::SendCrash => {
            m::send_crash(&dir);
            game.native_chat("Crash report sent. Thanks!");
        }
        m::Command::Web(up, say) => {
            let signed = game.arsenal_signed(up.prefix, &up.extra);
            match signed {
                Some(signed) => {
                    m::send_upload(up, signed);
                    game.native_chat(&say);
                }
                None => game.native_chat("That needs your Minecraft account (offline accounts can't use the Arsenal website)."),
            }
        }
        m::Command::Cloud(save) => {
            m::cloud_ask(save);
            game.native_chat(if save { "Saving to your Arsenal cloud backup..." } else { "Restoring from your Arsenal cloud backup..." });
        }
        m::Command::ForgetCrash => {
            let _ = std::fs::remove_file(dir.join("crash.txt"));
            game.native_chat("Crash report dropped.");
        }
    }
}
