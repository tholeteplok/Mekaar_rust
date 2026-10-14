//! # mekaar-cli: Automated E2E Integration Tests

use mekaar_cli::simulation::run_simulation;

#[tokio::test]
async fn test_full_pair_simulation_in_process() {
    let result = run_simulation(None).await;
    assert!(result.is_ok(), "Simulasi Alice <-> Bob gagal: {:?}", result.err());
}
