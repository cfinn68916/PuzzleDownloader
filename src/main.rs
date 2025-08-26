use chrono::prelude::{DateTime, Utc};
use std::fs::File;
use std::io::Write;

fn current_time_8601() -> String {
    let st = &std::time::SystemTime::now();
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%Y%m%d%H%M%S"))
}

async fn fetch_puzzle(url: String, name: String) -> Option<String> {
    let response = reqwest::get(url).await;
    if response.is_err() {
        return Some(format!(
            "First unwrap: {}",
            response.unwrap_err().to_string()
        ));
    }
    let tex2 = response.unwrap().text().await;
    if tex2.is_err() {
        return Some(format!("Second unwrap: {}", tex2.unwrap_err().to_string()));
    }
    let tex = tex2.unwrap();
    let finded = tex.find("var task = '");
    if finded.is_none() {
        return Some("Task not found".to_string());
    }
    let s = finded.unwrap() + "var task = '".len();
    let tsm = &tex[s..];
    let also_found = tsm.split("';").nth(0);
    if also_found.is_none() {
        return Some("'; not found".to_string());
    }
    let task = also_found.unwrap();
    let fx = File::create_new(format!("data/{}_{}", current_time_8601(), name));
    if fx.is_err() {
        return Some(format!(
            "File creation unwrap: {}",
            fx.unwrap_err().to_string()
        ));
    }
    let mut f = fx.unwrap();
    let fw = f.write_all(task.as_bytes());
    if fw.is_err() {
        return Some(format!(
            "File write unwrap: {}",
            fw.unwrap_err().to_string()
        ));
    }
    fw.unwrap();

    let fs = f.sync_all();
    if fs.is_err() {
        return Some(format!("File sync unwrap: {}", fs.unwrap_err().to_string()));
    }
    None
}

#[tokio::main]
async fn main() {
    let puzzles = vec![
        (
            "https://www.puzzle-binairo.com/daily-binairo-plus",
            "daily-binairo-plus",
        ),
        (
            "https://www.puzzle-binairo.com/weekly-binairo-plus",
            "weekly-binairo-plus",
        ),
        (
            "https://www.puzzle-binairo.com/monthly-binairo-plus",
            "monthly-binairo-plus",
        ),
        (
            "https://www.puzzle-futoshiki.com/daily-futoshiki",
            "daily-futoshiki",
        ),
        (
            "https://www.puzzle-futoshiki.com/weekly-futoshiki",
            "weekly-futoshiki",
        ),
        (
            "https://www.puzzle-futoshiki.com/monthly-futoshiki",
            "monthly-futoshiki",
        ),
        (
            "https://www.puzzle-futoshiki.com/daily-renzoku",
            "daily-renzoku",
        ),
        (
            "https://www.puzzle-futoshiki.com/weekly-renzoku",
            "weekly-renzoku",
        ),
        (
            "https://www.puzzle-futoshiki.com/monthly-renzoku",
            "monthly-renzoku",
        ),
        (
            "https://www.puzzle-chess.com/chess-melee-daily",
            "chess-melee-daily",
        ),
        (
            "https://www.puzzle-chess.com/chess-melee-weekly",
            "chess-melee-weekly",
        ),
        (
            "https://www.puzzle-chess.com/chess-melee-monthly",
            "chess-melee-monthly",
        ),
        (
            "https://www.puzzle-chess.com/solo-chess-daily",
            "solo-chess-daily",
        ),
        (
            "https://www.puzzle-chess.com/solo-chess-weekly",
            "solo-chess-weekly",
        ),
        (
            "https://www.puzzle-chess.com/solo-chess-monthly",
            "solo-chess-monthly",
        ),
        (
            "https://www.puzzle-chess.com/chess-ranger-daily",
            "chess-ranger-daily",
        ),
        (
            "https://www.puzzle-chess.com/chess-ranger-weekly",
            "chess-ranger-weekly",
        ),
        (
            "https://www.puzzle-chess.com/chess-ranger-monthly",
            "chess-ranger-monthly",
        ),
        (
            "https://www.puzzle-minesweeper.com/daily-mosaic",
            "daily-mosaic",
        ),
        (
            "https://www.puzzle-minesweeper.com/weekly-mosaic",
            "weekly-mosaic",
        ),
        (
            "https://www.puzzle-minesweeper.com/monthly-mosaic",
            "monthly-mosaic",
        ),
        (
            "https://www.puzzle-minesweeper.com/daily-minesweeper",
            "daily-minesweeper",
        ),
        (
            "https://www.puzzle-minesweeper.com/weekly-minesweeper",
            "weekly-minesweeper",
        ),
        (
            "https://www.puzzle-minesweeper.com/monthly-minesweeper",
            "monthly-minesweeper",
        ),
        (
            "https://www.puzzle-binairo.com/daily-binairo",
            "daily-binairo",
        ),
        (
            "https://www.puzzle-binairo.com/weekly-binairo",
            "weekly-binairo",
        ),
        (
            "https://www.puzzle-binairo.com/monthly-binairo",
            "monthly-binairo",
        ),
        ("https://www.puzzle-yin-yang.com/?size=15", "daily-yin-yang"),
        (
            "https://www.puzzle-yin-yang.com/?size=16",
            "weekly-yin-yang",
        ),
        (
            "https://www.puzzle-yin-yang.com/?size=17",
            "monthly-yin-yang",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=8",
            "daily-thermometers",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=10",
            "weekly-thermometers",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=12",
            "monthly-thermometers",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=9 ",
            "daily-thermometers-curved",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=11",
            "weekly-thermometers-curved",
        ),
        (
            "https://www.puzzle-thermometers.com/?size=13",
            "monthly-thermometers-curved",
        ),
        ("https://www.puzzle-norinori.com/?size=10", "daily-norinori"),
        (
            "https://www.puzzle-norinori.com/?size=11",
            "weekly-norinori",
        ),
        (
            "https://www.puzzle-norinori.com/?size=12",
            "monthly-norinori",
        ),
        ("https://www.puzzle-slant.com/?size=10", "daily-slant"),
        ("https://www.puzzle-slant.com/?size=11", "weekly-slant"),
        ("https://www.puzzle-slant.com/?size=12", "monthly-slant"),
        ("https://www.puzzle-lits.com/?size=10", "daily-lits"),
        ("https://www.puzzle-lits.com/?size=11", "weekly-lits"),
        ("https://www.puzzle-lits.com/?size=12", "monthly-lits"),
        ("https://www.puzzle-galaxies.com/?size=8", "daily-galaxies"),
        ("https://www.puzzle-galaxies.com/?size=9", "weekly-galaxies"),
        (
            "https://www.puzzle-galaxies.com/?size=10",
            "monthly-galaxies",
        ),
        ("https://www.puzzle-tents.com/?size=8", "daily-tents"),
        ("https://www.puzzle-tents.com/?size=9", "weekly-tents"),
        ("https://www.puzzle-tents.com/?size=10", "monthly-tents"),
        (
            "https://www.puzzle-battleships.com/?size=8",
            "daily-battleships",
        ),
        (
            "https://www.puzzle-battleships.com/?size=9",
            "weekly-battleships",
        ),
        (
            "https://www.puzzle-battleships.com/?size=10",
            "monthly-battleships",
        ),
        ("https://www.puzzle-pipes.com/?size=7", "daily-pipes"),
        ("https://www.puzzle-pipes.com/?size=8", "weekly-pipes"),
        ("https://www.puzzle-pipes.com/?size=9", "monthly-pipes"),
        ("https://www.puzzle-pipes.com/?size=17", "daily-pipes-wrap"),
        ("https://www.puzzle-pipes.com/?size=18", "weekly-pipes-wrap"),
        (
            "https://www.puzzle-pipes.com/?size=19",
            "monthly-pipes-wrap",
        ),
        ("https://www.puzzle-hitori.com/?size=12", "daily-hitoti"),
        ("https://www.puzzle-hitori.com/?size=13", "weekly-hitoti"),
        ("https://www.puzzle-hitori.com/?size=14", "monthly-hitoti"),
        ("https://www.puzzle-heyawake.com/?size=15", "daily-heyawake"),
        (
            "https://www.puzzle-heyawake.com/?size=16",
            "weekly-heyawake",
        ),
        (
            "https://www.puzzle-heyawake.com/?size=17",
            "monthly-heyawake",
        ),
        ("https://www.puzzle-shingoki.com/?size=17", "daily-shingoki"),
        (
            "https://www.puzzle-shingoki.com/?size=18",
            "weekly-shingoki",
        ),
        (
            "https://www.puzzle-shingoki.com/?size=19",
            "monthly-shingoki",
        ),
        ("https://www.puzzle-masyu.com/?size=13", "daily-masyu"),
        ("https://www.puzzle-masyu.com/?size=14", "weekly-masyu"),
        ("https://www.puzzle-masyu.com/?size=15", "monthly-masyu"),
        ("https://www.puzzle-stitches.com/?size=16", "daily-stiches"),
        ("https://www.puzzle-stitches.com/?size=17", "weekly-stiches"),
        (
            "https://www.puzzle-stitches.com/?size=18",
            "monthly-stiches",
        ),
        ("https://www.puzzle-aquarium.com/?size=9", "daily-aquarium"),
        (
            "https://www.puzzle-aquarium.com/?size=10",
            "weekly-aquarium",
        ),
        (
            "https://www.puzzle-aquarium.com/?size=11",
            "monthly-aquarium",
        ),
        ("https://www.puzzle-tapa.com/?size=8", "daily-tapa"),
        ("https://www.puzzle-tapa.com/?size=9", "weekly-tapa"),
        ("https://www.puzzle-tapa.com/?size=10", "monthly-tapa"),
        (
            "https://www.puzzle-star-battle.com/?size=9",
            "daily-star-battle",
        ),
        (
            "https://www.puzzle-star-battle.com/?size=10",
            "weekly-star-battle",
        ),
        (
            "https://www.puzzle-star-battle.com/?size=11",
            "monthly-star-battle",
        ),
        ("https://www.puzzle-kakurasu.com/?size=12", "daily-kakurasu"),
        (
            "https://www.puzzle-kakurasu.com/?size=13",
            "weekly-kakurasu",
        ),
        (
            "https://www.puzzle-kakurasu.com/?size=14",
            "monthly-kakurasu",
        ),
        (
            "https://www.puzzle-skyscrapers.com/?size=9",
            "daily-skyscraper",
        ),
        (
            "https://www.puzzle-skyscrapers.com/?size=10",
            "weekly-skyscraper",
        ),
        (
            "https://www.puzzle-skyscrapers.com/?size=11",
            "monthly-skyscraper",
        ),
        (
            "https://www.puzzle-shakashaka.com/?size=5",
            "daily-shakashaka",
        ),
        (
            "https://www.puzzle-shakashaka.com/?size=6",
            "weekly-shakashaka",
        ),
        (
            "https://www.puzzle-shakashaka.com/?size=7",
            "monthly-shakashaka",
        ),
        ("https://www.puzzle-kakuro.com/?size=15", "daily-kakuro"),
        ("https://www.puzzle-kakuro.com/?size=16", "weekly-kakuro"),
        ("https://www.puzzle-kakuro.com/?size=17", "monthly-kakuro"),
        (
            "https://www.puzzle-jigsaw-sudoku.com/?size=9",
            "daily-jigsaw-sudoku",
        ),
        (
            "https://www.puzzle-jigsaw-sudoku.com/?size=10",
            "weekly-jigsaw-sudoku",
        ),
        (
            "https://www.puzzle-jigsaw-sudoku.com/?size=11",
            "monthly-jigsaw-sudoku",
        ),
        (
            "https://www.puzzle-jigsaw-sudoku.com/?size=12",
            "daily-sandwich-sudoku",
        ),
        (
            "https://www.puzzle-killer-sudoku.com/?size=9",
            "daily-killer-sudoku",
        ),
        (
            "https://www.puzzle-killer-sudoku.com/?size=10",
            "weekly-killer-sudoku",
        ),
        (
            "https://www.puzzle-killer-sudoku.com/?size=11",
            "monthly-killer-sudoku",
        ),
        (
            "https://www.puzzle-nonograms.com/?size=6",
            "daily-nonograms",
        ),
        (
            "https://www.puzzle-nonograms.com/?size=5",
            "weekly-nonograms",
        ),
        (
            "https://www.puzzle-nonograms.com/?size=7",
            "monthly-nonograms",
        ),
        ("https://www.puzzle-loop.com/?size=13", "daily-loop"),
        ("https://www.puzzle-loop.com/?size=12", "weekly-loop"),
        ("https://www.puzzle-loop.com/?size=14", "monthly-loop"),
        ("https://www.puzzle-sudoku.com/?size=9", "daily-sudoku"),
        ("https://www.puzzle-sudoku.com/?size=10", "weekly-sudoku"),
        ("https://www.puzzle-sudoku.com/?size=11", "monthly-sudoku"),
        (
            "https://www.puzzle-sudoku.com/?size=8",
            "daily-sandwich-sudoku",
        ),
        ("https://www.puzzle-light-up.com/?size=13", "daily-light-up"),
        (
            "https://www.puzzle-light-up.com/?size=12",
            "weekly-light-up",
        ),
        (
            "https://www.puzzle-light-up.com/?size=14",
            "monthly-light-up",
        ),
        ("https://www.puzzle-bridges.com/?size=13", "daily-bridges"),
        ("https://www.puzzle-bridges.com/?size=12", "weekly-bridges"),
        ("https://www.puzzle-bridges.com/?size=14", "monthly-bridges"),
        ("https://www.puzzle-shikaku.com/?size=7", "daily-shikaku"),
        ("https://www.puzzle-shikaku.com/?size=6", "weekly-shikaku"),
        ("https://www.puzzle-shikaku.com/?size=8", "monthly-shikaku"),
        ("https://www.puzzle-nurikabe.com/?size=11", "daily-nurikabe"),
        (
            "https://www.puzzle-nurikabe.com/?size=12",
            "weekly-nurikabe",
        ),
        (
            "https://www.puzzle-nurikabe.com/?size=13",
            "monthly-nurikabe",
        ),
        ("https://www.puzzle-dominosa.com/?size=10", "daily-dominosa"),
        ("https://www.puzzle-dominosa.com/?size=5", "weekly-dominosa"),
        (
            "https://www.puzzle-dominosa.com/?size=11",
            "monthly-dominosa",
        ),
    ];
    if !std::fs::exists("data").unwrap() {
        std::fs::create_dir("data").unwrap();
    }

    for puzzle in puzzles {
        fetch_puzzle(puzzle.0.to_string(), puzzle.1.to_string()).await;
        println!("Completed {}", puzzle.1.to_string());
    }
}
