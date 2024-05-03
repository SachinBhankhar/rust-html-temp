use axum::{extract::Path, response::Html};
use minijinja::render;
use serde::Serialize;

#[path = "../templates/profile.rs"]
mod templates;

#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Profile {
    full_name: String,
    items: Vec<Items>,
}

pub async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    let items = vec![
        Items {
            id: 1,
            name: "sachin".into(),
        },
        Items {
            id: 2,
            name: String::from("sachin bhankhar"),
        },
    ];
    let profile = Profile {
        full_name: profile_name,
        items,
    };
    let r = render!(templates::profile(), profile => profile );
    Html(r)
}
