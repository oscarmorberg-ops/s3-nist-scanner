mod guardduty;
use guardduty::check_guardduty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Startar GuardDuty NIST PR.AC-4 scan...");
    
    let test_bucket = "oscarmorberg-test"; 
    check_guardduty(test_bucket).await?;
    
    println!("✅ NIST PR.AC-4 compliant - CSIO L5 ready!");
    Ok(())
}
