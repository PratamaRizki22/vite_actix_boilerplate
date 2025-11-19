use sqlx::PgPool;
use crate::services::web3_challenge_service::Web3ChallengeService;

/// Start background scheduled tasks
pub fn start_scheduled_tasks(pool: PgPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(600)); // Every 10 minutes
        
        loop {
            interval.tick().await;
            
            if let Err(e) = Web3ChallengeService::cleanup_expired(&pool).await {
                eprintln!("Error cleaning up expired Web3 challenges: {}", e);
            } else {
                println!("Successfully cleaned up expired Web3 challenges");
            }
        }
    });
}
