//! All possible routes with their params are defined in a big enum.

use super::{auth, controllers, legal, middleware, models, stripe};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post, Router},
};

/// This enum contains all of the route strings in the application. This
/// solves several problems.
///
/// 1. Maintaining a single source of truth for route paths, even if it has
///    multiple controllers for various HTTP methods
/// 2. Making it easier to refactor routing without needing to keep the axum
///    router and paths referenced in routers in sync.
/// 3. Making it easier to jump from a component to the handlers in a route it
///    references and visa versa.
/// 4. Further decoupling the app from the underlying HTTP.
/// 5. Allowing documentation on a route, which is super useful for quick
///    reference when authoring components.
///
/// For each route, the parameters are inside an Option<T>. If no parameters
/// are provided, we'll construct the route with the `:id` template in it
/// for the Axum router.
pub enum Route {
    About,
    Favicon,
    /// This is just a route which, when visited, will trigger the backend
    /// to hit the stripe API and create a customer portal session, then
    /// redirect the user to the customer portal URL. This allows us to
    /// avoid invoking the Stripe API every time we render the home page
    /// just to render a stripe portal link that the user typically won't
    /// click, anyway
    GotoStripePortal,
    Htmx,
    InitAnon,
    Login,
    Logout,
    PasswordReset,
    PasswordResetSecret(Option<String>),
    Ping,
    PrivacyPolicy,
    Register,
    RobotsTxt,
    Root,
    StaticAppleIcon,
    StaticLargeIcon,
    StaticManifest,
    StaticMaskableLargeIcon,
    StaticMaskableMediumIcon,
    StaticMaskableSmallIcon,
    StaticMediumIcon,
    StaticSmallIcon,
    StaticTinyIcon,
    StripeWehhook,
    /// The user is sent to this page when the stripe subscription status
    /// changes to anything non-active, including cancelled.
    SubscriptionInactive,
    /// This, on the other hand, is when our own free trial implementation
    /// has expired. In this case, the customer does not have a stripe
    /// subscription, so we're going to need to send them into a checkout
    /// session instead of sending them to the customer portal. If a customer
    /// who never had a subscription is sent to the customer portal, they
    /// won't have any subscription to manage -- they'll only be able to update
    /// billing info and payment method details.
    SubscriptionTrialEnded,
    TermsOfService,
    UserHome,
    /// Route which will return an empty string. This is mainly an HTMX utility
    /// to allow a component to easily be swapped with nothing.
    Void,
}

impl Route {
    pub fn as_string(&self) -> String {
        match self {
            Self::About => "/about".into(),
            Self::Favicon => "/favicon.ico".into(),
            Self::GotoStripePortal => "/stripe-portal".into(),
            Self::Htmx => "/vendor/htmx-1.9.12".into(),
            Self::InitAnon => "/authentication/init-anon".into(),
            Self::Login => "/authentication/login".into(),
            Self::Logout => "/authentication/logout".into(),
            Self::PasswordReset => "/authentication/reset-password".into(),
            Self::PasswordResetSecret(slug) => match slug {
                Some(slug) => format!("/authentication/reset-password/{slug}"),
                None => "/authentication/reset-password/:slug".into(),
            },
            Self::Ping => "/ping".into(),
            Self::PrivacyPolicy => "/privacy".into(),
            Self::Register => "/authentication/register".into(),
            Self::Root => "/".into(),
            Self::RobotsTxt => "/robots.txt".into(),
            Self::StaticAppleIcon => "/static/apple_icon".into(),
            Self::StaticLargeIcon => "/static/large-icon".into(),
            Self::StaticManifest => "/static/manifest".into(),
            Self::StaticMaskableLargeIcon => {
                "/static/maskable-large-icon".into()
            }
            Self::StaticMaskableMediumIcon => {
                "/static/maskable-medium-icon".into()
            }
            Self::StaticMaskableSmallIcon => {
                "/static/maskable-small-icon".into()
            }
            Self::StaticMediumIcon => "/static/icon".into(),
            Self::StaticSmallIcon => "/static/xs-icon".into(),
            Self::StaticTinyIcon => "/static/xxs-icon".into(),
            Self::StripeWehhook => "/stripe-webhook".into(),
            Self::SubscriptionInactive => "/subscription-inactive".into(),
            Self::SubscriptionTrialEnded => "/trial-ended".into(),
            Self::TermsOfService => "/terms".into(),
            Self::UserHome => "/home".into(),
            Self::Void => "/void".into(),
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// In [crate::main], protected routes are registered in a router with
/// [crate::middlware::auth] middleware. This causes any requesters who are not
/// authenticated to be redirected to the login page before the request handlers
/// are called.
fn get_authenticated_routes() -> Router<models::AppState> {
    Router::new()
        .route(&Route::UserHome.as_string(), get(controllers::user_home))
}

/// Routes where authentication is required, but we do not check subscription
/// status, so they can be visited by users who are not paying or do not
/// have an active free trial.
fn get_authenticated_free_routes() -> Router<models::AppState> {
    Router::new()
        .route(
            &Route::GotoStripePortal.as_string(),
            get(stripe::redirect_to_billing_portal),
        )
        .route(
            &Route::SubscriptionInactive.as_string(),
            get(stripe::subscription_ended),
        )
        .route(
            &Route::SubscriptionTrialEnded.as_string(),
            get(stripe::trial_expired),
        )
}

/// In [crate::main], these routes are not protected by any authentication, so
/// any requester can access these routes.
fn get_public_routes() -> Router<models::AppState> {
    Router::new()
        .route(&Route::About.as_string(), get(controllers::about))
        .route(&Route::Favicon.as_string(), get(controllers::get_favicon))
        .route(&Route::Htmx.as_string(), get(controllers::get_htmx_js))
        .route(
            &Route::StaticTinyIcon.as_string(),
            get(controllers::get_tiny_icon),
        )
        .route(&Route::InitAnon.as_string(), post(auth::init_anon))
        .route(&Route::Login.as_string(), get(auth::get_login_form))
        .route(&Route::Login.as_string(), post(auth::handle_login))
        .route(&Route::Logout.as_string(), get(auth::logout))
        .route(
            &Route::PasswordReset.as_string(),
            get(auth::get_password_reset_request),
        )
        .route(
            &Route::PasswordReset.as_string(),
            post(auth::handle_pw_reset_request),
        )
        .route(
            &Route::PasswordResetSecret(None).as_string(),
            get(auth::get_password_reset_form),
        )
        .route(
            &Route::PasswordResetSecret(None).as_string(),
            post(auth::handle_password_reset),
        )
        .route(&Route::Ping.as_string(), get(controllers::pong))
        .route(
            &Route::PrivacyPolicy.as_string(),
            get(legal::get_privacy_policy),
        )
        .route(
            &Route::Register.as_string(),
            get(auth::get_registration_form),
        )
        .route(
            &Route::Register.as_string(),
            post(auth::handle_registration),
        )
        .route(
            &Route::RobotsTxt.as_string(),
            get(controllers::get_robots_txt),
        )
        .route(&Route::Root.as_string(), get(controllers::root))
        .route(
            &Route::StaticAppleIcon.as_string(),
            get(controllers::get_apple_icon),
        )
        .route(
            &Route::StaticLargeIcon.as_string(),
            get(controllers::get_large_icon),
        )
        .route(
            &Route::StaticManifest.as_string(),
            get(controllers::get_manifest),
        )
        .route(
            &Route::StaticMaskableLargeIcon.as_string(),
            get(controllers::get_maskable_large_icon),
        )
        .route(
            &Route::StaticMaskableMediumIcon.as_string(),
            get(controllers::get_maskable_medium_icon),
        )
        .route(
            &Route::StaticMaskableSmallIcon.as_string(),
            get(controllers::get_maskable_small_icon),
        )
        .route(
            &Route::StaticMediumIcon.as_string(),
            get(controllers::get_medium_icon),
        )
        .route(
            &Route::StaticSmallIcon.as_string(),
            get(controllers::get_small_icon),
        )
        .route(
            &Route::StripeWehhook.as_string(),
            post(stripe::handle_stripe_webhook),
        )
        .route(&Route::TermsOfService.as_string(), get(legal::get_tos))
        .route(&Route::Void.as_string(), get(controllers::void))
}

pub fn get_routes(state: models::AppState) -> Router<models::AppState> {
    let protected_routes = get_authenticated_routes()
        .layer(from_fn(middleware::html_headers))
        .layer(from_fn(middleware::auth))
        .layer(from_fn_with_state(state, middleware::narc_on_subscriptions));

    let protected_free_routes = get_authenticated_free_routes()
        .layer(from_fn(middleware::html_headers))
        .layer(from_fn(middleware::auth));

    let public_routes = get_public_routes()
        .layer(from_fn(middleware::html_headers))
        .layer(from_fn(middleware::log));

    Router::new()
        .nest("/", protected_routes)
        .nest("/", public_routes)
        .nest("/", protected_free_routes)
}
