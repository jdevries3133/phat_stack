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

variable "openai_api_key" {
  type      = string
  sensitive = true
}

variable "smtp_email_username" {
  type      = string
  sensitive = true
}

variable "smtp_email_password" {
  type      = string
  sensitive = true
}

resource "random_password" "secret_key" {
  length  = 48
  special = false
}

data "external" "git_sha" {
  program = [
    "sh",
    "-c",
    "echo '{\"output\": \"'\"$(if [[ ! -z $GITLAB_SHA ]]; then echo $GITLAB_SHA; else git rev-parse HEAD; fi)\"'\"}'"
  ]
}

module "basic-deployment" {
  source  = "jdevries3133/basic-deployment/kubernetes"
  version = "3.2.0"

  app_name  = "phat-stack"
  container = "jdevries3133/phat_stack:${data.external.git_sha.result.output}"
  domain    = "phat-stack.jackdevries.com"

  extra_env = {
    SESSION_SECRET                = random_password.secret_key.result
    SMTP_EMAIL_USERNAME           = "jdevries3133@gmail.com"
    SMTP_EMAIL_PASSWORD           = var.smtp_email_password
  }
}
