use crate::prelude::*;

struct Tos;
impl Component for Tos {
    fn render(&self) -> String {
        r#"
        <div class="prose bg-slate-300 rounded p-2 md:p-4">
            <h1>Terms of Service</h1>
            <p>
                Put your terms of service here!
            </p>
        </div>
        "#.into()
    }
}

pub async fn get_tos() -> impl IntoResponse {
    Page {
        title: "Terms of Service",
        children: &PageContainer { children: &Tos {} },
    }
    .render()
}
