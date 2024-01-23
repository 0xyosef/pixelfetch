#![allow(unused_parens)]
#![allow(non_snake_case)]


#![allow(unused_parens)]
#![allow(non_snake_case)]

mod modules;
use modules::fetch::popen;
use modules::fetch::get_env_variable;
static DESCRIPTION_LEN:usize = 13;

use std::env;
use std::process::exit;


fn main() {

    let _Arg1 = env::args().nth(1).unwrap();
    let Arg1 = _Arg1.as_str();

    if Arg1 == "-help" || Arg1 == "--help" {
        println!(
            r#"Usage: ./xfcfet <path to a picture>
    Examples:
        ./xfcfet ~/Pictures/face.png
        ./xfcfet Pictures/meet.jpg
        ./xfcfet ./pic.png
    #these are all valid examples.

    Requiremts:
        chafa   ->   /usr/bin/chafa"#
        );
        exit(1);
    };
    let _banner: String = popen("chafa", &[Arg1, "-s", "30"]);

    let banner = _banner.split("\n").collect::<Vec<&str>>();

    // removing 'Description: ' from lsb_release output
    let mut Distro = popen("lsb_release", &["-d"]);
    for _i in 0..DESCRIPTION_LEN {
        Distro.remove(0);
    }

    // Information
    let mut user_name = format!(
        "  \x1b[38;5;2mName \x1b[38;5;3m➔ \x1b[0m{}",
        popen("whoami", &[])
    );
    let user_wm = format!(
        "  \x1b[38;5;2mWM \x1b[38;5;3m➔ \x1b[0m{}",
        get_env_variable("GDMSESSION")
    );
    let mut user_uptime = format!(
        "  \x1b[38;5;2mUptime \x1b[38;5;3m➔ \x1b[0m{}",
        popen("uptime", &["-p"])
    );
    let mut user_distro = format!("  \x1b[38;5;2mDistro \x1b[38;5;3m➔ \x1b[0m{}", Distro);
    let user_term = format!(
        "  \x1b[38;5;2mTerm \x1b[38;5;3m➔ \x1b[0m{}",
        get_env_variable("TERM")
    );
    let user_shell = format!(
        "  \x1b[38;5;2mShell \x1b[38;5;3m➔ \x1b[0m{}", env::var("SHELL").unwrap()
    );
    let seperator = "  \x1b[38;5;1m======================".to_string();
    let mut song_artist = format!(
        "  \x1b[38;5;2mArtist \x1b[38;5;3m➔\x1b[0m {}",
        popen("playerctl", &["-p", "spotify", "metadata", "artist"])
    );
    let mut song_album = format!(
        "  \x1b[38;5;2mAlbum \x1b[38;5;3m➔\x1b[0m {}",
        popen("playerctl", &["-p", "spotify", "metadata", "album"])
    );
    let mut song_name = format!(
        "  \x1b[38;5;2mSong \x1b[38;5;3m➔\x1b[0m {}",
        popen("playerctl", &["-p", "spotify", "metadata", "title"])
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
        user_wm,
        user_uptime,
        user_shell,
        user_term,
        seperator,
        song_artist,
        song_album,
        song_name,
    ];

    // Printing everything to the screen
    let mut i = 0;
    let mut InfoLine: &str;
    for line in &banner {
        if (info.len() <= i) {
            InfoLine = "";
        } else {
            InfoLine = &info[i];
        };
        if (i == banner.len() - 1) {
            break;
        };
        println!("{} {}", line, InfoLine);
        i += 1;
    }
}
