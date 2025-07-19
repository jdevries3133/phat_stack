# Phat Stack

[PostgreSQL](https://www.postgresql.org/), [HTMX](https://htmx.org/),
[Axum Web](https://github.com/tokio-rs/axum), and
[Tailwind CSS](https://tailwindcss.com/)

# Project Status

This is a hobby-project, but I'd be happy if others find it useful, and
contributions are welcome.

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
| `enum Route`                   |            | ✅      | ✅       | ✅    | ✅         |
| `trait Component`              |            | ✅      | ✅       | ✅    | ✅         |
| opinionated error-handling     |            | ✅      | ✅       | ✅    | ✅         |
| GitHub workflows for CI/CD     |            |         | ✅       | ✅    | ✅         |
| todo app                       |            |         | ✅       | ✅    | ✅         |
| passkey auth                   |            |         |          | ✅    | ✅         |
| user registration & login      |            |         |          | ✅    | ✅         |
| stripe integration             |            |         |          |       | ✅         |
| transactional email (via SMTP) |            |         |          |       | ✅         |
| oauth2 client & server         |            |         |          |       | ✅         |

The "moderate," base-template is essentially the
[`original-template`](https://github.com/jdevries3133/phat_stack/tree/original-template).
At least, the parts of the original template I intend to keep. Minimal and
bare-bones templates will be derived from the moderate template, simply by
deleting stuff.

Features of the heavy and superheavy templates will each be available as
first-party "contrib-style," supporting libraries. This includes
Containerization, GitHub workflows, transactional email, Stripe integration, and
Oauth2. There won't be any points of coupling between each of these. The
`create-phat-app` CLI will support granular mixing and matching of these
features/libraries. They will also be distributed as standalone Rust crates, so
they can always be added to a project later or even used standalone.
