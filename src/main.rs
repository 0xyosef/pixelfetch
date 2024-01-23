#![allow(unused_parens)]
#![allow(non_snake_case)]
mod modules;

use crate::modules::fetch::{
    display_default, display_image_with_info, get_env_variable, run_command,
};
use std::process::exit;
use std::{env, fs};

fn main() {
    // Information
    let args = env::args().nth(1).unwrap();
    let Arg1 = args.as_str();
    if Arg1 == "-help" || Arg1 == "--help" || Arg1 == "-h" {
        println!(
            r#"Usage: ./pixelfetch <path to a picture> or -d to default
    Examples:
            ./pixelfetch ~/Pictures/face.png
            ./pixelfetch Pictures/meet.jpg
            ./pixelfetch ./pic.png
            ./pixelfetch -d
    #these are all valid examples.
        "#
        );
        exit(1);
    }

    let distro = run_command("uname", &["-n"]);
    let mut user_name = format!(
        "  \x1b[38;5;2musername \x1b[38;5;3m➔ \x1b[0m{}",
        run_command("whoami", &[])
    );
    let session = format!(
        "  \x1b[38;5;2mWM \x1b[38;5;3m➔ \x1b[0m{}",
        get_env_variable("DESKTOP_SESSION")
    );
    let mut user_uptime = format!(
        "  \x1b[38;5;2mUptime \x1b[38;5;3m➔ \x1b[0m{}",
        run_command("uptime", &["-p"])
    );
    let mut user_distro = format!("  \x1b[38;5;2mDistro \x1b[38;5;3m➔ \x1b[0m{}", distro);
    let user_term = format!(
        "  \x1b[38;5;2mTerm \x1b[38;5;3m➔ \x1b[0m{}",
        get_env_variable("TERM")
    );
    let user_shell = format!(
        "  \x1b[38;5;2mShell \x1b[38;5;3m➔ \x1b[0m{}",
        env::var("SHELL").unwrap()
    );
    let mut song_artist = format!(
        "  \x1b[38;5;2mArtist \x1b[38;5;3m➔\x1b[0m {}",
        run_command("playerctl", &["-p", "spotify", "metadata", "artist"])
    );
    let mut song_album = format!(
        "  \x1b[38;5;2mAlbum \x1b[38;5;3m➔\x1b[0m {}",
        run_command("playerctl", &["-p", "spotify", "metadata", "album"])
    );
    let mut song_name = format!(
        "  \x1b[38;5;2mSong \x1b[38;5;3m➔\x1b[0m {}",
        run_command("playerctl", &["-p", "spotify", "metadata", "title"])
    );

    // popping '\n' character
    user_distro.pop();
    user_uptime.pop();
    user_name.pop();
    song_name.pop();
    song_album.pop();
    song_artist.pop();

    // Order of printing information to the screen
    let info = [
        user_distro,
        user_name,
        session,
        user_uptime,
        user_shell,
        user_term,
        song_artist,
        song_album,
        song_name,
    ];
    if Arg1 == "-d" {
        let banner = fs::read_to_string("./demo").expect("Should have been able to read the file");
        display_default(banner, &info);
    } else {
        let img = image::open(Arg1).expect("failed to open image");
        display_image_with_info(&img, &info);
    }
}
