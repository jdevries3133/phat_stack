# Phat Stack

[PostgreSQL](https://www.postgresql.org/), [HTMX](https://htmx.org/),
[Axum Web](https://github.com/tokio-rs/axum), and
[Tailwind CSS](https://tailwindcss.com/)

## Example Project

For a more in-depth project using this stack, see my [Notion
clone](https://github.com/jdevries3133/nc).

I also built [beancount.bot](https://beancount.bot) with this stack, and the
[repository is open source](https://github.com/jdevries3133/calcount) (though it
has a restrictive license).

## Benefits

- app compiles to a very small statically linked musl binary, allowing super
  fast time to deployment
- SQLx provides type-safe SQL queries at compile time when using the `query_as!`
  macro
- DX with live-reloading is actually quite nice
- there's a lot less complexity here compared to a typical React/Express stack
  or similar, and even less compared to a Laraval/Django setup where you might
  have an ORM and/or a templating language.
- PWA-ready
- Stripe integration hath been integrated
- support for anonymous users enables very low-friction conversion (see
  [beancount.bot](https://beancount.bot) for a live example)

## Bugs

- there is a home-grown auth framework in here which I certainly use, but should
  be audited by a security expert (see `mod session`, `mod auth`, `mod crypto`,
  `mod pw`)
- I'm not using a templating library; implementers of `Component::render` need
  to beware of XSS sanitization concerns, and call `ammonia::clean` for
  user-provided strings.
