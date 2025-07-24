# VARIABLES
variable "REGISTRY" {
  default = "ghcr.io"
}

variable "REPO" {
  default = "mysteryforge/eth-kit"
}

variable "GIT_SHA" {
  default = "undefined"
}

# GROUPS
group "default" {
  targets = ["all"]
}

group "all" {
  targets = [
    "execution-probe",
    "beacon-probe",
    "blockspeed",
  ]
}

target "common" {
  dockerfile = "Dockerfile"
  cache-from = [
    "type=registry,ref=${REGISTRY}/${REPO}-cache:base",
    "type=registry,ref=${REGISTRY}/${REPO}-cache:planner",
    "type=registry,ref=${REGISTRY}/${REPO}-cache:builder"
  ]
  cache-to = [
    "type=registry,ref=${REGISTRY}/${REPO}-cache:base,mode=max",
    "type=registry,ref=${REGISTRY}/${REPO}-cache:planner,mode=max",
    "type=registry,ref=${REGISTRY}/${REPO}-cache:builder,mode=max"
  ]
}

# DISPERSER TARGETS
target "execution-probe" {
    inherits = ["common"]
    context    = "."
    dockerfile = "./Dockerfile"
    target     = "execution-probe"
    tags       = ["${REGISTRY}/${REPO}/execution-probe:${GIT_SHA}"]
}

target "beacon-probe" {
    inherits = ["common"]
    context    = "."
    dockerfile = "./Dockerfile"
    target     = "beacon-probe"
    tags       = ["${REGISTRY}/${REPO}/beacon-probe:${GIT_SHA}"]
}

target "blockspeed" {
    inherit = ["common"]
    context = "."
    dockerfile = "./Dockerfile"
    target = "blockspeed"
    tags       = ["${REGISTRY}/${REPO}/blockspeed:${GIT_SHA}"]
}