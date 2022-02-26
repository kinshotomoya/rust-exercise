use actix_web;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::to;
use serde::Deserialize;
use std::thread;
use num_cpus;

fn main() {
    // actix-webは、　HttpServer::newした時点で内部的にthread Pool（workerという定義がされているが）を作成しているので
    //マルチスレッドにするために何か特別な処理をする必要がない
    // actix-webのデフォルトのスレッドプールは、マシンの論理コア数とと同じで設定されている
    // このcrateでマシンのcpu core数を取得できる
    // https://crates.io/crates/num_cpus
    println!("num cpus is {}", num_cpus::get());
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    server.bind("127.0.0.1:3000").expect("").run().expect("")

}

fn get_index() -> HttpResponse {
    println!("thread id: {}", thread::current().name().unwrap());
    HttpResponse::Ok().content_type("text/html").body("<title>webですよ!</title>")
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64
}
