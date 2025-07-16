# Phat Stack

[PostgreSQL](https://www.postgresql.org/), [HTMX](https://htmx.org/),
[Axum Web](https://github.com/tokio-rs/axum), and
[Tailwind CSS](https://tailwindcss.com/)

# Project Status

Originally, this project was a template app, with the idea that this repository
could be forked and modified into a specific project. Look at the
[`original-template`](https://github.com/jdevries3133/phat_stack/tree/original-template)
tag to see the old template.

Since
[`7fa061b590f8c298de187eddf7e33375e3175889`](https://github.com/jdevries3133/phat_stack/commit/7fa061b590f8c298de187eddf7e33375e3175889),
I'm moving towards a new approach where this project will become a
`create-phat-app` CLI and an ecosystem of first-party "contrib-style" supporting
libraries.

I'm planning for `create-phat-app` to be able to boilerplate-out the following
variants;

| feature                        | bare-bones | minimal | moderate | heavy | superheavy |
| ------------------------------ | ---------- | ------- | -------- | ----- | ---------- |
| PHAT development environment   | ✅         | ✅      | ✅       | ✅    | ✅         |
| PHAT production build          | ✅         | ✅      | ✅       | ✅    | ✅         |
| `trait Component` abstraction  |            | ✅      | ✅       | ✅    | ✅         |
| `enum Route` abstraction       |            | ✅      | ✅       | ✅    | ✅         |
| `Dockerfile`                   |            | ✅      | ✅       | ✅    | ✅         |
| todo app                       |            |         | ✅       | ✅    | ✅         |
| GitHub workflows for CI/CD     |            |         | ✅       | ✅    | ✅         |
| user registration & login      |            |         |          | ✅    | ✅         |
| passkey auth                   |            |         |          | ✅    | ✅         |
| transactional email (via SMTP) |            |         |          |       | ✅         |
| stripe integration             |            |         |          |       | ✅         |

Containerization, GitHub workflows, transactional email, and Stripe integration
will each be available as first-party "contrib-style" supporting libraries, and
there won't be any points of coupling between them. The `create-phat-app` will
support granular mixing and matching of these features/libraries. 
