use crate::tools::{
    ascii::ToolAscii, base::ToolBase, base64::ToolBase64, crc::ToolCrc, json::ToolJson, quaternion::ToolQuaternion,
    unixtime::ToolUnixtime, file_hash::ToolFileHash, html::ToolHtml, url::ToolUrl,
};
use log::info;
use web_sys::window;
use yew::prelude::*;

pub struct Page {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

pub enum Msg {
    Update(String),
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let title = _ctx.props().title.clone();

        let content = match title.as_str() {
            "unix-timestamp" => html! { <ToolUnixtime /> },
            "quaternion" => html! { <ToolQuaternion /> },
            "crc" => html! { <ToolCrc /> },
            "ascii" => html! { <ToolAscii /> },
            "json" => html! { <ToolJson /> },
            "base64" => html! { <ToolBase64 /> },
            "base" => html! { <ToolBase /> },
            "file-hash" => html! { <ToolFileHash /> },
            "html" => html! { <ToolHtml /> },
            "url" => html! { <ToolUrl /> },
            _ => html! { <p>{ "Content not found" }</p> },
        };
        self.add_item(title.as_str());

        html! {
                <>
                    <div class="home-wrapper">
                        { content }
                    </div>
                </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {}
}

impl Page {
    pub fn get_recent_items(&self) -> Vec<String> {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        if let Ok(Some(json)) = local_storage.get_item("recent-item") {
            serde_json::from_str(&json).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        }
    }

    pub fn add_item(&self, item: &str) {
        let mut items = self.get_recent_items();

        // 중복 제거 및 리스트 갱신
        if let Some(pos) = items.iter().position(|x| x == item) {
            items.remove(pos);
        }
        items.insert(0, item.to_string());

        // 최대 크기를 초과하는 경우 초과 항목 제거
        if items.len() > 4 {
            items.truncate(4);
        }

        // JSON으로 직렬화하여 저장
        let json = serde_json::to_string(&items).unwrap();

        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage.set_item("recent-item", &json).unwrap();
    }
}
