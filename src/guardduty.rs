use aws_sdk_guardduty::Client;
use aws_config::meta::region::RegionProviderChain;

pub async fn check_guardduty(bucket: &str) -> Result<String, Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-north-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let response = client.list_findings()
        .detector_id("7cce33799064eaa5d7bbbaecb6ddab3b")  // Ersätt med din Detector ID
        .max_results(10)
        .send()
        .await?;

    println!("🔍 GuardDuty för {}:", bucket);
    println!("   Hittade {} finding IDs: {:?}", 
             response.finding_ids.as_ref().map_or(0, |v| v.len()), 
             response.finding_ids);
    
    Ok("GuardDuty scan klar".to_string())
}
