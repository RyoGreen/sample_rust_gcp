module "artifact_registry" {
  source        = "../modules/artifact_registry"
  env           = local.env
  repository_id = local.artifact_registry.repository_id
}

module "cloud_run" {
  source         = "../modules/cloud_run"
  location       = local.region
  cloud_run_name = local.cloud_run.name
  repository_id  = local.cloud_run.repository_id
  project_name   = local.project
  tag            = local.cloud_run.tag
  app_name       = local.cloud_run.app_name
}
