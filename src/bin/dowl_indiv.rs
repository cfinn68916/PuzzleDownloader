use std::fs::File;
use std::io::{Read, Write};

async fn fetch_puzzle(url: String) -> Result<String, String> {
    let response = reqwest::get(url).await;
    if response.is_err() {
        return Err(format!(
            "First unwrap: {}",
            response.unwrap_err().to_string()
        ));
    }
    let tex2 = response.unwrap().text().await;
    if tex2.is_err() {
        return Err(format!("Second unwrap: {}", tex2.unwrap_err().to_string()));
    }
    let tex = tex2.unwrap();
    let finded = tex.find("var task = '");
    if finded.is_none() {
        return Err("Task not found".to_string());
    }
    let s = finded.unwrap() + "var task = '".len();
    let tsm = &tex[s..];
    let also_found = tsm.split("';").nth(0);
    if also_found.is_none() {
        return Err("'; not found".to_string());
    }
    let task = also_found.unwrap();
    Ok(task.to_string())
}

#[tokio::main]
async fn main() {
    let mut puzzles = vec![];
    let mut ident = 0;
    if std::fs::exists("kakurasu").ok().unwrap_or(false) {
        let mut f = File::open("kakurasu").unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        puzzles = contents
            .split("\n")
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect();
        std::fs::remove_file("kakurasu").unwrap();
    }
    let mut f = File::create_new("kakurasu").unwrap();
    puzzles.iter().for_each(|x|{
        f.write_all(x.as_bytes()).unwrap();
        f.write_all("\n".as_bytes()).unwrap();
    });
    while ident < 100 {
        let p = fetch_puzzle("https://www.puzzle-kakurasu.com/".to_string()).await;
        if p.is_ok() {
            let up = p.unwrap();
            if !puzzles.contains(&up) {
                puzzles.push(up.clone());
                f.write_all(up.as_bytes()).unwrap();
                f.write_all("\n".as_bytes()).unwrap();
                ident = 0;
            } else {
                ident = ident + 1;
            }
        } else {
            println!("{}", p.unwrap_err());
        }
        println!("{}", puzzles.len());
    }
    f.sync_all().unwrap();
}
