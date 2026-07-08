use arc_swap::ArcSwap;
use jsonwebtoken::jwk::JwkSet;
use std::sync::{Arc, LazyLock};

pub static SUPABASE_JWKS: LazyLock<ArcSwap<JwkSet>> = LazyLock::new(|| {
    ArcSwap::from_pointee(JwkSet {
        keys: Vec::with_capacity(0),
    })
});

pub async fn fetch_from_supabase(project_ref: &str) -> Result<(), anyhow::Error> {
    let url = format!("https://{project_ref}.supabase.co/auth/v1/.well-known/jwks.json");

    let jwks = reqwest::get(&url).await?.json::<JwkSet>().await?;

    SUPABASE_JWKS.store(Arc::new(jwks));
    tracing::info!("Populated Supabase JWKs");

    Ok(())
}
