use kube::config::Kubeconfig;
use metal_rs::apis::{
    configuration::{ApiKey, Configuration},
    machine_api, size_api,
};

static CLOUDCTL_CONTEXT: &str = "cloudctl-prod";
static METAL_BASE_PROD: &str = "https://api.fits.cloud/metal";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = get_metal_config()?;

    let res = size_api::list_sizes(&cfg).await;
    match res {
        Ok(imgs) => println!("got images: {:#?}", imgs),
        Err(e) => println!("got error: {:#?}", e),
    }

    Ok(())
}

fn get_token_from_k8s() -> Result<String, Box<dyn std::error::Error>> {
    let k = Kubeconfig::read()?;
    let nc = k
        .contexts
        .iter()
        .find(|c| c.name == CLOUDCTL_CONTEXT.to_string());

    let ccp = match nc {
        Some(n) => n,
        None => return Err("no named context cloudctl-prod".into()),
    };

    let ai = match k.auth_infos.iter().find(|a| a.name == ccp.context.user) {
        Some(x) => x,
        None => return Err("no token found in k8s context".into()),
    };
    let token = ai
        .auth_info
        .auth_provider
        .as_ref()
        .unwrap()
        .config
        .get("id-token")
        .unwrap()
        .to_string();

    Ok(token)
}

fn get_metal_config() -> Result<Configuration, Box<dyn std::error::Error>> {
    let tok = get_token_from_k8s()?;
    let mut cfg = Configuration::new();

    cfg.base_path = METAL_BASE_PROD.to_string();

    cfg.api_key = Some(ApiKey {
        prefix: Some("Bearer".to_string()),
        key: tok,
    });
    Ok(cfg)
}
