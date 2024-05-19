resource "google_artifact_registry_repository" "rust_sample_http_server" {
  repository_id = var.repository_id
  location      = "asia-northeast1"
  format        = "DOCKER"

  labels = {
    env = var.env
  }
}
