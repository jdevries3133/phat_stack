use crate::prelude::*;

struct PrivacyPolicy;
impl Component for PrivacyPolicy {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Privacy Policy</h1>
            <p>
                Put your privacy policy here!
            </p>
        </div>
        "#
        .into()
    }
}

pub async fn get_privacy_policy() -> impl IntoResponse {
    Page {
        title: "Privacy Policy",
        children: &PageContainer {
            children: &PrivacyPolicy {},
        },
    }
    .render()
}
