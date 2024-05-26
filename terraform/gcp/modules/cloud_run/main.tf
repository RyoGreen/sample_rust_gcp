resource "google_cloud_run_service" "default" {
  name     = var.cloud_run_name
  location = var.location

  template {
    spec {
      containers {
        image = "asia-northeast1-docker.pkg.dev/${var.project_name}/${var.repository_id}/${var.app_name}:${var.tag}"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}
