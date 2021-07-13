use ranking_handler::{fetch, ranking_type, seichi_api};
use std::{collections::HashMap, process};
use rand::seq::SliceRandom;

const RANKING_URL: &str = "https://ranking-gigantic.seichi.click/api/ranking";

fn queries(ranking_type: ranking_type::RankingType) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("type".to_string(), ranking_type.as_str());
    map.insert("offset".to_string(), "0".to_string());
    map.insert("lim".to_string(), ranking_type.get_targets().to_string());
    map.insert("duration".to_string(), "monthly".to_string());
    map
}

#[tokio::main]
async fn main() {
    let result = fetch::fetch(RANKING_URL, Some(queries(ranking_type::RankingType::Break))).await;
    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
    let result = seichi_api::deserialize(result.unwrap()).await;
    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
    let result = result.unwrap().ranks;
    let result: Vec<_> = result.iter().map(|rank| seichi_api::Lottery::convert(rank)).collect();
    let rng = &mut rand::thread_rng();
    let _samples: Vec<_> = result.choose_multiple(rng, 1).cloned().collect();
    println!("{:#?}", _samples);

    // println!("月別ランキング報酬の抽選を開始します。");
    // println!();

    // println!("今月の月別景品付与の対象になったプレイヤーの方々を一覧表示します。");
    // util::pause();

    // println!("整地量：20名");
    // println!();

    // println!("建築量：10名");
    // println!();

    // println!("景品が実際に付与される方はこちらです。");
    // util::pause();

    // println!("整地量：5名／20名");
    // println!();

    // println!("建築量：3名／10名");
    // println!();

    // println!("抽選を終了しました。");
}
