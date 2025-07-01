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

# DISPERSER TARGETS
target "execution-probe" {
  context    = "."
  dockerfile = "./Dockerfile"
  target     = "execution-probe"
  tags       = ["${REGISTRY}/${REPO}/execution-probe:${GIT_SHA}"]
}

target "beacon-probe" {
  context    = "."
  dockerfile = "./Dockerfile"
  target     = "beacon-probe"
  tags       = ["${REGISTRY}/${REPO}/beacon-probe:${GIT_SHA}"]
}

target "blockspeed" {
    context = "."
    dockerfile = "./Dockerfile"
    target = "blockspeed"
    tags       = ["${REGISTRY}/${REPO}/blockspeed:${GIT_SHA}"]
}