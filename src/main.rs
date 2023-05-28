/**
*   test instance in http://127.0.0.1:8080/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   api in http://127.0.0.1:8080/apis/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   optional with &format=json
**/
pub mod oj;

use actix_web::{get, web, Result};
use actix_json_response::JsonResponse;
use serde::Deserialize;
use serde::Serialize;

use crate::oj::main_battle;

#[derive(Deserialize)]
struct Info {
    hp : i32,
    atk : i32,
    def : i32,
    evd : i32,
    hpt : i32,
    atkt : i32,
    deft : i32,
    evdt : i32,
    format : Option<String>
}

#[derive(Serialize)]
pub struct BattleResult {
    kill_rate : f32,
    be_kill_rate : f32,
    you_alive_remain_hp : f32,
    opp_alive_remain_hp : f32,
    fb_decay: f32
}

#[get("/battle")]
async fn index(info: web::Query<Info>) -> Result<String> {
    let mut txt = String::new();
    let br = main_battle(info.hp, info.atk, info.def, info.evd, info.hpt, info.atkt, info.deft, info.evdt);
    txt += &format!("击杀率 : {:.2}\n", br.kill_rate);
    txt += &format!("反杀率 : {:.2}\n", br.be_kill_rate);
    txt += &format!("残余血量（双方均幸存时） : {:.1} / {:.1}\n", br.you_alive_remain_hp, br.opp_alive_remain_hp);
    txt += &format!("开战有利度 : {}\n", (br.fb_decay*100.0).round()/100.0);
    Ok(txt)
}

// web api with JsonResponse
#[get("/apis/battle")]
async fn api(info: web::Query<Info>) -> Result<JsonResponse<BattleResult>> {
    if info.format.is_none()||info.format.clone().unwrap().eq("json") {
        let txt = main_battle(info.hp, info.atk, info.def, info.evd, info.hpt, info.atkt, info.deft, info.evdt);
        Ok(txt.into())
    } else {
        panic!("unknown format");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| App::new().service(index).service(api))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}