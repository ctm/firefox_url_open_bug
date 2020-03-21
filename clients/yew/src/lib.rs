use {
    anyhow::Error,
    log::info,
    serde_derive::{Deserialize, Serialize},
    wasm_bindgen::prelude::*,
    yew::{
        format::Json,
        prelude::*,
        services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask},
    },
};

#[derive(Serialize, Debug, Deserialize)]
struct Request {
    message: String,
}

pub struct App {
    ws_service: WebSocketService,
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
}

pub enum Msg {
    Start,
    Connected,
    ReceivedData,
    Ignore,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Start);
        Self {
            ws_service: WebSocketService::new(),
            link,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            Start => {
                let callback = self.link.callback(|Json::<Result<Request, Error>>(data)| {
                    info!("data: {:?}", data);
                    ReceivedData
                });
                let notification = self.link.callback(|status| match status {
                    WebSocketStatus::Opened => Connected,
                    WebSocketStatus::Closed | WebSocketStatus::Error => Ignore,
                });
                self.ws = self
                    .ws_service
                    .connect("ws://localhost:9001", callback, notification)
                    .ok();
            }
            Connected => {
                let request = Request {
                    message: "ping".to_string(),
                };
                self.ws.as_mut().unwrap().send(Json(&request));
            }
            ReceivedData => {
                let window = yew::utils::window();
                info!("about to open_with_url");
                let w = window.open_with_url("https://duckduckgo.com");
                info!("w: {:?}", w);
            }
            Ignore => {}
        }
        false
    }

    fn view(&self) -> Html {
        html! {}
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
