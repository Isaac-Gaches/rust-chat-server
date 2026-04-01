use axum::{routing::get, Router, response::Html};
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(home))
}

async fn home() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<body>
    <h1>Chat</h1>

    <input id="msg" placeholder="type message..." />
    <button onclick="send()">Send</button>

    <ul id="messages"></ul>

    <script>
        const ws = new WebSocket(`wss://${location.host}/ws`);

        ws.onopen = () => {
            console.log("connected");
        };

        ws.onmessage = (event) => {
            const li = document.createElement("li");
            li.textContent = event.data;
            document.getElementById("messages").appendChild(li);
        };

        function send() {
            const input = document.getElementById("msg");
            ws.send(input.value);
            input.value = "";
        }
    </script>
</body>
</html>
"#)
}