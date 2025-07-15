//! UI Components. [Component] trait is object-safe, allowing very nice
//! component composition patterns via Rust's dynamic dispatch features.

// In many cases, we need to do a let binding to satisfy the borrow checker
// and for some reason, clippy identifies those as unnecessary. Maybe there
// are and clippy knows more than me, maybe not.
#![allow(clippy::let_and_return)]

use super::{auth::is_anon, prelude::*};

#[cfg(feature = "live_reload")]
const LIVE_RELOAD_SCRIPT: &str = r#"<script>
    (async () => {
        while (true) {
            try {
                await fetch('/ping?poll_interval_secs=60');
            } catch (e) {
                console.log("hup from ping; let's live-reload");
                const el = document.createElement('p');
                el.innerText = "Reloading...";
                el.classList.add("bg-yellow-100");
                el.classList.add("p-2");
                el.classList.add("rounded");
                el.classList.add("w-full");
                el.classList.add("dark:text-black");
                document.body.insertBefore(el, document.body.firstChild);
                setInterval(async () => {
                    setTimeout(() => {
                        // At some point, a compiler error may be preventing
                        // the server from coming back
                        el.innerText = "Reload taking longer than usual; check for a compiler error";
                    }, 5000);
                    // Now the server is down, we'll fast-poll it (trying to
                    // get an immediate response), and reload the page when it
                    // comes back
                    try {
                        await fetch('/ping?poll_interval_secs=0');
                        window.location.reload()
                    } catch (e) {}
                }, 100);
                break;
            }
        }
    })();
</script>"#;

#[cfg(not(feature = "live_reload"))]
const LIVE_RELOAD_SCRIPT: &str = "";

pub trait Component: Send + Sync {
    /// Render the component to a HTML string. By convention, the
    /// implementation should sanitize all string properties at render-time
    fn render(&self) -> String;
}

impl std::fmt::Display for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

pub struct Page<'a> {
    pub title: &'a str,
    pub children: &'a dyn Component,
}

impl Component for Page<'_> {
    fn render(&self) -> String {
        // note: we'll get a compiler error here until the tailwind build
        // occurs. Make sure you use `make build` in the Makefile to get
        // both to happen together
        let tailwind = include_str!("./tailwind.generated.css");
        let htmx = Route::Htmx;
        let apple_icon = Route::StaticAppleIcon;
        let manifest = Route::StaticManifest;
        format!(
            r##"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
                    <meta name="theme-color" content="#BBF7D0"/>
                    <meta name="description" content="ChatGPT-powered calorie counter" />
                    <title>{title}</title>
                    <style>
                        {tailwind}
                    </style>
                    {LIVE_RELOAD_SCRIPT}
                    <link rel="manifest" href="{manifest}" />
                    <link rel="apple-touch-icon" href="{apple_icon}">
                    <script defer src="{htmx}"></script>
                </head>
                <body hx-boost="true">
                    {body_html}
                </body>
            </html>
            "##,
            tailwind = tailwind,
            title = clean(self.title),
            body_html = self.children.render()
        )
    }
}

struct Footer;
impl Component for Footer {
    fn render(&self) -> String {
        let privacy = Route::PrivacyPolicy;
        let tos = Route::TermsOfService;
        let home = Route::UserHome;
        let about = Route::About;
        format!(
            r#"
            <footer class="flex flex-wrap items-center justify-center gap-2 p-4">
                <a class="link" href="{privacy}">Privacy Policy</a>
                <a class="link" href="{tos}">Terms of Service</a>
                <a class="link" href="{home}">Dashboard</a>
                <a class="link" href="/">Home</a>
                <a class="link" href="{about}">About</a>
            </footer>
            "#
        )
    }
}

pub struct PageContainer<'a> {
    pub children: &'a dyn Component,
}
impl Component for PageContainer<'_> {
    fn render(&self) -> String {
        let children = self.children.render();
        let footer = Footer {}.render();
        format!(
            r#"
            <div
                class="p-2 sm:p-4 md:p-8 bg-teal-50 dark:bg-indigo-1000
                dark:text-slate-200 min-h-[100vh]"
            >
                {children}
                {footer}
            </div>
            "#
        )
    }
}

pub struct Home {}
impl Component for Home {
    fn render(&self) -> String {
        let login_route = Route::Login;
        let init_anon = Route::InitAnon;
        let footer = Footer {}.render();
        format!(
            r#"
            <main
                class="p-2 sm:p-4 md:p-8 bg-teal-50 dark:bg-indigo-1000
                dark:text-slate-200 min-h-[100vh]"
            >
                <h1 class="mt-2 md:mt-8 text-3xl font-extrabold">
                    &#127793; PHAT Stack &#129752;
                </h1>
                <div class="h-[90vh] flex justify-center flex-col">
                <h2
                    class="bg-gradient-to-br from-blue-600 via-green-500
                    to-indigo-400 inline-block text-transparent bg-clip-text
                    text-6xl"
                >
                    Web App Very Nice
                </h2>
                <h2
                    class="text-4xl"
                >
                    Make a very nice web app:
                    <span
                        class="font-extrabold dark:text-indigo-200 text-indigo-500"
                    >
                        PostgreSQL, HTMX, Axum, & Tailwind CSS
                    </span>
                </h2>
                <form method="POST" action="{init_anon}">
                    <input type="hidden" value="" name="timezone" id="timezone" />
                    <button
                        class="
                            bg-gradient-to-tr
                            from-blue-700
                            to-indigo-700
                            from-blue-100
                            to-indigo-200
                            p-2
                            rounded
                            shadow-md
                            hover:shadow-sm
                            dark:shadow-purple-200
                            text-xl
                            font-extrabold
                            text-white
                            my-4
                        "
                    >Get Started</button>
                    </form>
                </div>
            </main>
            <div
                class="bg-gradient-to-tr dark:from-emerald-900
                dark:via-indigo-1000 dark:to-indigo-1000"
            >
                <div class="flex items-center justify-center">
                    <div
                        class="bg-emerald-700 max-w-md p-2 inline-block my-2
                        text-lg text-center text-teal-50 dark:text-slate-200
                        rounded"
                    >

                        <div class="flex gap-3">
                            <form method="POST" action="{init_anon}">
                                <input type="hidden" value="" name="timezone" id="timezone" />
                                <div
                                    class="bg-gradient-to-tr from-blue-300
                                    to-indigo-300 rounded-full p-3 text-black"
                                >
                                    <h3 class="text-lg font-semibold">Try it Out!</h3>
                                    <p class="text-sm">
                                        Click here to jump right in and start using
                                        this app. Zero-commitment sign-up & 30 days
                                        free, on us!
                                    </p>
                                    <button
                                        class="
                                            bg-gradient-to-tr
                                            from-blue-700
                                            to-indigo-700
                                            from-blue-100
                                            to-indigo-200
                                            p-2
                                            rounded
                                            shadow-md
                                            hover:shadow-sm
                                            dark:shadow-purple-200
                                            text-xl
                                            font-extrabold
                                            text-white
                                            my-3
                                        "
                                    >Get Started</button>
                                </div>
                            </form>
                            <div class="bg-teal-50 dark:bg-indigo-900 border-2
                                border-indigo-800 inline-flex p-3 rounded-full
                                items-center gap-3 mt-2 dark:text-slate-200
                                self-center text-black"
                            >
                                <p>Have an account?</p>
                                <a href="{login_route}">
                                    <button
                                        class="border-2 border-slate-800 rounded p-2
                                        text-nowrap hover:bg-emerald-100 transition"
                                    >Log In</button>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex items-center justify-center my-12">
                </div>
                <div class="flex items-center justify-center">
                </div>
            {footer}
            </div>
            <script>
                (() => {{
                    for (const el of document.querySelectorAll("[name='timezone'")) {{
                        el.value = Intl.DateTimeFormat().resolvedOptions().timeZone;
                    }}
                }})();
            </script>
            "#
        )
    }
}

pub struct UserHome<'a> {
    pub username: &'a str,
}
impl Component for UserHome<'_> {
    fn render(&self) -> String {
        let username = clean(self.username);
        let log_out = Route::Logout;
        let register_route = Route::Register;
        let register_ui = if is_anon(self.username) {
            format!(
                r#"
            <p class="text-sm">
                You are an anonymous user. It was quick and easy to jump into
                the app, but you should register for an account to create a
                username and password. Your user ID won't change, so you'll
                retain ownership of any data you create while anonymously
                trying the app if you convert into a user.
            </p>
            <p class="text-sm">
                Note that if you allow an anonymous user to sign out, they will
                lose everything, so try to avoid that! I normally hide the
                "log out" button from anonymous users, and only show it after
                registration.
            </p>
            <a href="{register_route}">
                <button class="bg-emerald-100 hover:bg-emerald-200 rounded p-1">
                    Register
                </button>
            </a>
            "#
            )
        } else {
            "".into()
        };
        format!(
            r#"
            <div class="flex flex-col prose">
                Hi, {username}!
                <a class="link" href="{log_out}">Log Out</a>
                {register_ui}
            </div>
            "#
        )
    }
}

pub struct AboutPage;
impl Component for AboutPage {
    fn render(&self) -> String {
        let home = Route::UserHome;
        format!(
            r#"
            <div class="prose dark:text-slate-200">
                <h1 class="dark:text-slate-200">About Your App</h1>
                <p><a class="link" href="{home}">Return Home</a></p>
                <p>
                    Tell the world about your app!
                </p>
            </div>
            "#
        )
    }
}

pub struct Span {
    pub content: String,
}
impl Component for Span {
    fn render(&self) -> String {
        format!("<span>{}</span>", clean(&self.content))
    }
}
