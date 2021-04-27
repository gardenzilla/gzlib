// Helper to load service address from env
fn service_address(service_name: &'static str) -> Result<String, String> {
  match std::env::var(service_name) {
    Ok(addr) => Ok(format!("http://{}", addr)),
    Err(_) => Err(format!(
      "Requested service addr ENV var not found: {}",
      service_name
    )),
  }
}
