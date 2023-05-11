use std::collections::HashMap;

use crate::{
    prelude::{StrictPath, ENV_DEBUG},
    resource::{config::RootsConfig, manifest::Os},
    scan::{LauncherGame, TitleFinder},
};

/// https://github.com/lutris/lutris/blob/e4ae3d7193da777ebb370603a9e20c435f725300/docs/installers.rst
#[derive(serde::Deserialize)]
struct LutrisGame {
    game: GameSection,
    /// ID of the game itself.
    game_slug: String,
    /// Human-readable.
    name: String,
}

#[derive(serde::Deserialize)]
struct GameSection {
    prefix: Option<StrictPath>,
    working_dir: StrictPath,
}

pub fn scan(root: &RootsConfig, title_finder: &TitleFinder) -> HashMap<String, LauncherGame> {
    let mut games = HashMap::new();

    log::trace!("Scanning Lutris root for games: {}", root.path.interpret());

    for spec in root.path.joined("games/*.y*ml").glob() {
        log::debug!("Inspecting Lutris game file: {}", spec.render());

        let Some(content) = spec.read() else {
            log::warn!("Unable to read Lutris game file: {}", spec.render());
            continue;
        };
        let Ok(game) = serde_yaml::from_str::<LutrisGame>(&content) else {
            log::warn!("Unable to parse Lutris game file: {}", spec.render());
            continue;
        };

        let official_title = title_finder.find_one(&[game.name.to_owned()], &None, &None, true, true, false);
        let prefix = game.game.prefix;
        let platform = Some(match &prefix {
            Some(_) => Os::Windows,
            None => Os::HOST,
        });

        let title = match official_title {
            Some(title) => {
                log::trace!(
                    "Recognized Lutris game: '{title}' from '{}' (slug: '{}')",
                    &game.name,
                    &game.game_slug
                );
                title
            }
            None => {
                let log_message = format!(
                    "Unrecognized Lutris game: '{}' (slug: '{}')",
                    &game.name, &game.game_slug
                );
                if std::env::var(ENV_DEBUG).is_ok() {
                    eprintln!("{log_message}");
                }
                log::info!("{log_message}");
                game.name
            }
        };

        games.insert(
            title,
            LauncherGame {
                install_dir: game.game.working_dir,
                prefix,
                platform,
            },
        );
    }

    log::trace!("Finished scanning Lutris root for games: {}", root.path.interpret());

    games
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        resource::{
            manifest::{Manifest, Store},
            ResourceFile,
        },
        testing::repo,
    };

    fn manifest() -> Manifest {
        Manifest::load_from_string(
            r#"
            windows-game:
              files:
                <base>/file1.txt: {}
            "#,
        )
        .unwrap()
    }

    fn title_finder() -> TitleFinder {
        TitleFinder::new(&manifest(), &Default::default())
    }

    #[test]
    fn scan_finds_nothing_when_folder_does_not_exist() {
        let root = RootsConfig {
            path: StrictPath::new(format!("{}/tests/nonexistent", repo())),
            store: Store::Lutris,
        };
        let games = scan(&root, &title_finder());
        assert_eq!(HashMap::new(), games);
    }

    #[test]
    fn scan_finds_all_games() {
        let root = RootsConfig {
            path: StrictPath::new(format!("{}/tests/launchers/lutris", repo())),
            store: Store::Lutris,
        };
        let games = scan(&root, &title_finder());
        assert_eq!(
            hashmap! {
                "windows-game".to_string() => LauncherGame {
                    install_dir: StrictPath::new("/home/deck/Games/service/windows-game/drive_c/game".to_string()),
                    prefix: Some(StrictPath::new("/home/deck/Games/service/windows-game".to_string())),
                    platform: Some(Os::Windows),
                },
            },
            games,
        );
    }
}
