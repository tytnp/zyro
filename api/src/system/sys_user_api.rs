use axum::response::Html;

pub async fn add() -> Html<&'static str> {
    Html("<h1>user_add</h1>")
}

pub async fn del() -> Html<&'static str> {
    Html("<h1>user_del</h1>")
}

pub async fn edit() -> Html<&'static str> {
    Html("<h1>user_edit</h1>")
}

pub async fn list() -> Html<&'static str> {
    Html("<h1>user_list</h1>")
}

pub async fn one() -> Html<&'static str> {
    Html("<h1>user_one</h1>")
}
