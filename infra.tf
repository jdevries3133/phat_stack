terraform {

  backend "s3" {
    bucket = "my-sites-terraform-remote-state"
    key    = "phat-stack-state"
    region = "us-east-2"
  }

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.7.1"
    }
    helm = {
      source  = "hashicorp/helm"
      version = ">= 2.4.1"
    }
  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

provider "helm" {
  kubernetes {
    config_path = "~/.kube/config"
  }
}

resource "random_password" "secret_key" {
  length  = 48
  special = false
}


data "external" "git_describe" {
  program = [
    "sh",
    "-c",
    "echo '{\"output\": \"'\"$(if [[ ! -z $GITLAB_SHA ]]; then echo $GITLAB_SHA; else git rev-parse HEAD; fi)\"'\"}'"
  ]
}

// This infrastructure module is fairly tightly coupled to my homelab
// Kubernetes cluster. YMMV
module "basic-deployment" {
  source  = "jdevries3133/basic-deployment/kubernetes"
  version = "3.0.2"

  app_name  = "phat-stack"
  container = "jdevries3133/phat_stack:${data.external.git_describe.result.output}"
  domain    = "phat-stack.jackdevries.com"

  extra_env = {
    SESSION_SECRET = random_password.secret_key.result
  }
}
