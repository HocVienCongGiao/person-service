module "persons" {
  source                 = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name           = var.service_name
  query_api_parent_id    = module.person-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.person-service.mutation_api_gateway_resource_id

  function_name = "persons"
  file_name     = var.service_name

  depends_on = [
    module.person-service
  ]

  environment = var.environment
  db_host     = var.db_host
  db_user     = var.db_user
  db_password = var.db_password
  db_name     = var.db_name
}

module "person_id" {
  source                 = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name           = var.service_name
  query_api_parent_id    = module.persons.query_api_gateway_resource_id
  mutation_api_parent_id = module.persons.mutation_api_gateway_resource_id

  function_name = "person_id"
  file_name     = var.service_name
  path_part     = "{id}"
  depends_on = [
    module.persons
  ]

  environment = var.environment
  db_host     = var.db_host
  db_user     = var.db_user
  db_password = var.db_password
  db_name     = var.db_name
}
