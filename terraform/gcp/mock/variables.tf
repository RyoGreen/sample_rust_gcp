locals {
  // project
  project = ""
  region  = ""
  zone    = ""
  env     = ""

  // artifact_registry
  artifact_registry = {
    repository_id = ""
  }

  // cloud_run
  cloud_run = {
    name          = ""
    tag           = ""
    repository_id = ""
    app_name      = ""
  }
}
