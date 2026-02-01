use tokio::time::sleep;
use std::time::Duration;

async fn tache_a() {
    println!("A début");
    sleep(Duration::from_secs(1)).await;
    println!("A fin");
}

async fn tache_b() {
    println!("B début");
    sleep(Duration::from_secs(1)).await;
    println!("B fin");
}

#[tokio::main]
async fn main() {
    let debut = std::time::Instant::now();

    // SANS `join!`, si on faisait `tache_a().await; tache_b().await;`,
    // le temps total serait de 1s + 1s = 2s.

    // AVEC `tokio::join!`, les deux futures sont "polled" (avancés) simultanément
    // par le runtime. C'est de la vraie concurrence.
    // Le programme attend que TOUTES les tâches listées soient terminées.
    tokio::join!(tache_a(), tache_b());

    // Le temps total devrait être proche de 1s (le max des deux tâches), et non 2s.
    println!("Temps total : {:?}", debut.elapsed());
}