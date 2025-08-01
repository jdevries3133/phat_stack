SHELL := /bin/bash
ENV=source .env &&
PROJECT_NAME=phat_stack

# The registry is presumed to be docker.io, which is the implicit default
DOCKER_ACCOUNT=jdevries3133
ifdef GITHUB_SHA
	TAG=$(GITHUB_SHA)
else
	TAG=$(shell git rev-parse HEAD)
endif
CONTAINER_QUALNAME=$(DOCKER_ACCOUNT)/$(PROJECT_NAME)
CONTAINER_EXACT_REF=$(DOCKER_ACCOUNT)/$(PROJECT_NAME):$(TAG)

.PHONY: _start-db
.PHONY: _stop-db
.PHONY: backup-prod
.PHONY: build
.PHONY: build-container
.PHONY: check
.PHONY: debug-container
.PHONY: deploy
.PHONY: dev
.PHONY: proxy-prod-db
.PHONY: push-container
.PHONY: push-container
.PHONY: setup
.PHONY: shell-db
.PHONY: sqlx
.PHONY: watch-db

check: setup
ifdef CI
	pnpm run build
endif
ifndef CI
	@# Locally, we want to ensure that `cargo sqlx prepare` was run, otherwise
	@# the build will fail in CI. So, we'll run an offline build as part of
	@# our checks
	SQLX_OFFLINE=true cargo build --features enable_smtp_email
endif
ifndef SKIP_CARGO
	cargo clippy -- -D warnings
	cargo fmt --check
	cargo test
endif

sqlx:
	cargo sqlx prepare -- --features enable_smtp_email

build: setup
	pnpm run build
	cargo build --release

setup:
ifndef CI
	@# we only want the `.env` file locally in practice. We never run the app
	@# in CI (yet). The problem with having the `.env` file in CI is that
	@# sqlx will pickup on the `DATABASE_URL` environment variable and try
	@# to talk to a datbase that isn't there, causing compilation to fail.
	@# See also https://github.com/launchbadge/sqlx/blob/540baf7df55a372cb79d8636d02b1361a495b344/sqlx-cli/README.md#force-building-in-offline-mode
	[[ ! -f .env ]] && cp env-template .env || true
endif

deploy:
ifdef CI
	terraform init
endif
	terraform apply -auto-approve

_start-db:
	$(ENV) docker run \
        --name $(PROJECT_NAME) \
        -e POSTGRES_DB="$$POSTGRES_DB" \
        -e POSTGRES_USER="$$POSTGRES_USER" \
        -e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
        -p 5432:5432 \
        -d \
        postgres:15

_stop-db:
	docker kill $(PROJECT_NAME) || true
	docker rm $(PROJECT_NAME) || true

watch-db:
	docker logs -f $(PROJECT_NAME)

shell-db:
	$(ENV) PGPASSWORD=$$POSTGRES_PASSWORD \
		psql -U "$$POSTGRES_USER" -h 0.0.0.0 $$POSTGRES_DB

proxy-prod-db:
	kubectl -n $(PROJECT_NAME) port-forward service/db-postgresql 5433:5432

backup-prod:
	kubectl exec \
		-n $(PROJECT_NAME) \
		pod/db-postgresql-0 \
		-- /bin/sh -c "pg_dump postgresql://$(PROJECT_NAME):\$$POSTGRES_PASSWORD@127.0.0.1:5432/$(PROJECT_NAME)" \
		> ~/Desktop/$(PROJECT_NAME)_backups/backup-$(shell date '+%m-%d-%Y__%H:%M:%S').sql

build-container: setup
	pnpm run build
	rustup target add x86_64-unknown-linux-musl
	cargo build \
		--release \
		--target x86_64-unknown-linux-musl \
		--features enable_smtp_email
	docker buildx build --load --platform linux/amd64 -t $(CONTAINER_EXACT_REF) .


# Run the above container locally, such that it can talk to the local
# PostgreSQL database launched by `make _start-db`. We expect here that the
# local database is already running and the container has already been built.
#
# Note: on macOS, you need to change the database host to "host.docker.internal"
# to allow the container to talk to the local PostgreSQL instance running inside
# Docker. On Linux, you can add the --net=host flag to the invocation of
# `docker run` below, to make PostgreSQL at localhost:5432 visible to the
# container.
debug-container:
	$(ENV) docker run \
		-e RUST_BACKTRACE=1 \
		-e DATABASE_URL="$$DATABASE_URL" \
		-e SESSION_SECRET="$$SESSION_SECRET" \
		-e SMTP_EMAIL_USERNAME="$$SMTP_EMAIL_USERNAME" \
		-e SMTP_EMAIL_PASSWORD="$$SMTP_EMAIL_PASSWORD" \
		-p 8000:8000 \
		$(CONTAINER_EXACT_REF)

push-container: build-container
	docker push $(CONTAINER_EXACT_REF)
