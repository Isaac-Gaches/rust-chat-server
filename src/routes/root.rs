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
    const username = prompt("Enter your name:");

    const ws = new WebSocket(`wss://${location.host}/ws`);

    ws.onopen = () => {
        ws.send(username); // first message = username
    };

    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);

        const li = document.createElement("li");

        if (data.type === "Join") {
            li.textContent = `${data.user} joined`;
        } else if (data.type === "Leave") {
            li.textContent = `${data.user} left`;
        } else if (data.type === "Chat") {
            li.textContent = `${data.user}: ${data.content}`;
        } else if (data.type === "Query") {
            data.logs.forEach(log => {
                const li = document.createElement("li");
                li.textContent = log;
                document.getElementById("messages").appendChild(li);
            });
        }

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