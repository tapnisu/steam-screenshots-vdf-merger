use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use steam_screenshots_vdf_joiner::Cli;

#[derive(Serialize, Deserialize, Debug)]
struct ScreenshotsVDF {
    screenshots: HashMap<String, Game>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Game(Vec<Screenshot>);

#[derive(Serialize, Deserialize, Debug)]
struct Screenshot {
    #[serde(rename = "type")]
    screenshot_type: String,
    filename: String,
    thumbnail: String,
    imported: String,
    externalfilename: Option<String>,
    taggedpublishedfiles: Option<TaggedPublishedFiles>,
    width: String,
    height: String,
    #[serde(rename = "gameid")]
    game_id: String,
    creation: String,
    caption: Option<String>,
    #[serde(rename = "Permissions")]
    permissions: String,
    hscreenshot: String,
    publishedfileid: Option<String>,
    timelineid: Option<String>,
    timelinetime: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaggedPublishedFiles(HashMap<String, String>);

fn main() {
    let cli = Cli::parse();

    let screenshots_vdfs: Vec<ScreenshotsVDF> = cli
        .input
        .into_iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .filter_map(|file| better_vdf::from_str(&file).ok())
        .collect();

    let mut output_vdf = ScreenshotsVDF {
        screenshots: HashMap::new(),
    };

    for vdf in screenshots_vdfs {
        for (game_id, game) in vdf.screenshots {
            output_vdf
                .screenshots
                .entry(game_id)
                .or_insert(Game(Vec::new()))
                .0
                .extend(game.0);
        }
    }

    let output = better_vdf::to_string(&output_vdf).unwrap();
    fs::write(cli.output, output).unwrap();
}
