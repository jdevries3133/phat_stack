# Your Phat App

This is a phat-stack app template. It's very bare-bones, and avoids making
assumptions. The assumptions we do make are;

- you're building a web app with PostgreSQL, HTMX, Axum, and Tailwind
- you'd like to have patterns to follow for CI/CD, routing, error-handling, and
  UI

Grep through this template for the pattern `YOUR .* HERE`, and begin replacing
the boilerplate with your app-specific content where appropriate.

This is the default "moderate boilerplate," template, which comes with a few
bells and whistles to get you on the right track;

- It is a todo app
- It has GitHub workflows to run clippy and unit-tests
- It has a `Dockerfile`
- ⚠️ It has a GitHub workflow to push container images to Docker Hub
- ⚠️ It does not have authentication

Other templates are available;

| feature                       | bare-bones | minimal | moderate | heavy-passkey | superheavy |
| ----------------------------- | ---------- | ------- | -------- | ------------- | ---------- |
| PHAT development environment  | ✅         | ✅      | ✅       | ✅            | ✅         |
| PHAT production build         | ✅         | ✅      | ✅       | ✅            | ✅         |
| `trait Component` abstraction |            | ✅      | ✅       | ✅            | ✅         |
| `enum Route` abstraction      |            | ✅      | ✅       | ✅            | ✅         |
| `Dockerfile`                  |            | ✅      | ✅       | ✅            | ✅         |
| todo app                      |            |         | ✅       | ✅            | ✅         |
| GitHub workflows for CI/CD    |            |         | ✅       | ✅            | ✅         |
| user login & registration     |            |         |          | ✅            | ✅         |
| passkey auth                  |            |         |          | ✅            | ✅         |
| email + password auth         |            |         |          |               | ✅         |
| email verification (via SMTP) |            |         |          |               | ✅         |
| TOTP MFA                      |            |         |          |               | ✅         |
| email MFA                     |            |         |          |               | ✅         |
| passkey as MFA                |            |         |          |               | ✅         |
