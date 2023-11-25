# Phat Stack

[PostgreSQL](https://www.postgresql.org/), [HTMX](https://htmx.org/),
[Axum Web](https://github.com/tokio-rs/axum), and
[Tailwind CSS](https://tailwindcss.com/)

## Example Project

For a more in-depth project using this stack, see my [Notion
clone](https://github.com/jdevries3133/nc).

## Benefits

- app compiles to a very small statically linked musl binary, allowing super
  fast time to deployment
- SQLx provides type-safe SQL queries at compile time when using the `query_as!`
  macro
- DX with live-reloading is actually quite nice
- there's a lot less complexity here compared to a typical React/Express stack
  or similar, and even less compared to a Laraval/Django setup where you might
  have an ORM and/or a templating language.

## Bugs

- there is a homelab auth framework in here which is probably not
  production-grade (see `mod session`, `mod auth`, `mod crypto`, `mod pw`)
- internal abstractions like `trait Component` and `trait DbModel` feel maybe
  60% good to me; certainly, there's room for growth, especially towards DRY'ing
  out forms and having easy CRUD object management
- I'm not using a templating library; implementers of `Component::render` need
  to beware of XSS sanitization concerns, and call `ammonia::clean` for
  user-provided strings.
