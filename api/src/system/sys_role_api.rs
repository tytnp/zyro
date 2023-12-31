use axum::response::Html;

pub async fn add() -> Html<&'static str> {
    Html("<h1>role_add</h1>")
}

pub async fn del() -> Html<&'static str> {
    Html("<h1>role_del</h1>")
}

pub async fn edit() -> Html<&'static str> {
    Html("<h1>role_edit</h1>")
}

pub async fn list() -> Html<&'static str> {
    Html("<h1>role_list</h1>")
}

pub async fn one() -> Html<&'static str> {
    Html("<h1>role_one</h1>")
}
