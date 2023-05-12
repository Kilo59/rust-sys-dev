use actix_web::{web, App, HttpResponse, HttpServer};
use std::process;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
            .route("/std", web::post().to(post_std))
    });

    println!("pid is {}", process::id());
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
            <br>
            <form action="/std" method="post">
            <input type="file", name="CSV File"/>
            <input type="text", name="header"/>
            <button type="submit">Compute Standard Deviation</button>
        "#,
    )
}

use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n
    }
    n
}

fn post_std() -> HttpResponse {
    let result = 2.5;
    let response = format!("The standard deviation is {}", result);

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}
