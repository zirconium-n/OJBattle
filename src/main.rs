/**
*   test instance in http://127.0.0.1:8080/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   api in http://127.0.0.1:8080/apis/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   optional with &format=json
**/

//cargo run -- -p [port]
//http://127.0.0.1:[port]/battle/4/1/1/-1/4/0/0/1


pub mod oj;
use crate::oj::{main_battle};


use std::env;
use actix_web::{Responder, HttpResponse};
use actix_web::{get, web, Result};
use serde::Deserialize;
use serde::Serialize;


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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleResponse { 
    pub be_kill_rate : f32,
    pub you_alive_remain_hp : f32,
    pub opp_alive_remain_hp : f32,
    pub fb_10_win : f32,
    pub fb_10_draw : f32,
    pub fb_10_lose : f32,
    pub challenge_advantage : f32,
}


#[get("/battle")]
async fn index(info: web::Query<Info>) -> Result<String> {
    let mut txt = String::new();
    let br = main_battle(info.hp, info.atk, info.def, info.evd, info.hpt, info.atkt, info.deft, info.evdt);
    txt += &format!("击杀率 : {:.2}\n", br.kill_rate);
    txt += &format!("反杀率 : {:.2}\n", br.be_kill_rate);
    txt += &format!("残余血量（双方均幸存时） : {:.1} / {:.1}\n", br.you_alive_remain_hp, br.opp_alive_remain_hp);
    txt += &format!("最终决战（10回合，胜/平/负） : {:.2} / {:.2} / {:.2}\n", br.fb_10_win, br.fb_10_draw, br.fb_10_lose);
    txt += &format!("开战有利度 : {:.2}\n", br.challenge_advantage);
    Ok(txt)
}

struct BattleRequest {
    hp : i32,
    atk : i32,
    def : i32,
    evd : i32,
    hpt : i32,
    atkt : i32,
    deft : i32,
    evdt : i32,
}

async fn api(input: web::Json<BattleRequest>) -> impl Responder {
    let result = main_battle(input.hp, input.atk, input.def, input.evd, input.hpt, input.atkt, input.deft, input.evdt);
    let response = BattleResponse {
        be_kill_rate: result.be_kill_rate,
        you_alive_remain_hp: result.you_alive_remain_hp,
        opp_alive_remain_hp: result.opp_alive_remain_hp,
        fb_10_win: result.fb_10_win,
        fb_10_draw: result.fb_10_draw,
        fb_10_lose: result.fb_10_lose,
        challenge_advantage: result.challenge_advantage,
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut port_str = String::from("8080");
    for i in 1..args.len() {
        if args[i] == "-p" {
            port_str = String::from(&args[i + 1]);
        }
    }
    let port : u16 = port_str.parse().unwrap();
    use actix_web::{App, HttpServer};


    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", port))?
        .run()
        .await

}
