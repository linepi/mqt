use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use reqwest::Client;

/// 表示ntfy通知的操作按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyAction {
    /// 操作类型，例如 "view"
    pub action: String,
    /// 按钮显示的标签
    pub label: String,
    /// 点击按钮时打开的URL
    pub url: String,
    /// 请求方法
    pub method: Option<String>,
    /// 请求头
    pub headers: Option<HashMap<String, String>>,
    /// 请求体
    pub body: Option<String>,
    /// 是否清除
    pub clear: Option<bool>
}

impl NtfyAction {
    pub fn new() -> Self {
        NtfyAction {
            action: "".to_string(),
            label: "".to_string(),
            url: "".to_string(),
            method: None,
            headers: None,
            body: None,
            clear: None
        }
    }

    pub fn view_action(mut self, label: String, url: String) -> Self {
        self.action = "view".to_string();
        self.label = label;
        self.url = url;
        self.method = None;
        self.headers = None;
        self.body = None;
        self.clear = None;
        self
    }

    pub fn http_action(mut self, label: String, url: String, method: String, headers: HashMap<String, String>, body: String) -> Self {
        self.action = "http".to_string();
        self.label = label;
        self.url = url;
        self.method = Some(method);
        self.headers = Some(headers);
        self.body = Some(body);
        self.clear = None;
        self
    }
        
}

/// ntfy通知的JSON结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyMessage {
    /// 必填: 目标主题名称
    pub topic: String,
    /// 消息正文; 如果为空或未传递则设置为"triggered"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 消息标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 标签列表，可能映射到表情符号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 消息优先级，1=最低，3=默认，5=最高
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 自定义操作按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<NtfyAction>>,
    /// 点击通知时打开的网站URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<String>,
    /// 附件的URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    /// 是否使用Markdown格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<bool>,
    /// 用作通知图标的URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 附件的文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// 延迟发送的时间戳或持续时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// 用于电子邮件通知的电子邮件地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用于语音呼叫的电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<String>,
}

impl NtfyMessage {

    pub fn new(topic: String) -> Self {
        NtfyMessage {
            topic,
            message: None,
            title: None,
            tags: None,
            priority: None,
            actions: None,
            click: None,
            attach: None,
            markdown: None,
            icon: None,
            filename: None,
            delay: None,
            email: None,
            call: None,
        }
    }

    /// 设置消息内容
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// 设置标题
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: String) -> Self {
        if let Some(ref mut tags) = self.tags {
            tags.push(tag);
        } else {
            self.tags = Some(vec![tag]);
        }
        self
    }

    /// 设置标签列表
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// 设置优先级 (1-5)
    pub fn priority(mut self, priority: i32) -> Self {
        if priority >= 1 && priority <= 5 {
            self.priority = Some(priority);
        }
        self
    }

    /// 添加操作按钮
    pub fn add_action(mut self, action: NtfyAction) -> Self {
        if let Some(ref mut actions) = self.actions {
            actions.push(action);
        } else {
            self.actions = Some(vec![action]);
        }
        self
    }

    /// 设置点击URL
    pub fn click(mut self, url: String) -> Self {
        self.click = Some(url);
        self
    }

    /// 设置附件URL
    pub fn attach(mut self, url: String) -> Self {
        self.attach = Some(url);
        self
    }

    /// 启用Markdown格式
    pub fn with_markdown(mut self) -> Self {
        self.markdown = Some(true);
        self
    }

    /// 设置通知图标
    pub fn icon(mut self, url: String) -> Self {
        self.icon = Some(url);
        self
    }

    /// 设置附件文件名
    pub fn filename(mut self, name: String) -> Self {
        self.filename = Some(name);
        self
    }

    /// 设置延迟发送
    pub fn delay(mut self, delay: String) -> Self {
        self.delay = Some(delay);
        self
    }

    /// 设置电子邮件地址
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    /// 设置电话号码
    pub fn call(mut self, number: String) -> Self {
        self.call = Some(number);
        self
    }

    /// 将消息发送到ntfy服务器
    /// 返回一个Result，表示发送操作是否成功
    pub async fn send(&self) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();
        client.post(crate::constants::NTFY_URL)
            .json(self)
            .send()
            .await
    }
}

pub fn warning_message(topic: String, title: String, message: String) -> NtfyMessage {
    NtfyMessage::new(topic)
        .title(title)
        .message(message)
        .add_tag("warning".to_string())
        .priority(4)
}

pub fn success_message(topic: String, title: String, message: String) -> NtfyMessage {
    NtfyMessage::new(topic)
        .title(title)
        .message(message)
        .add_tag("white_check_mark".to_string())
        .priority(3)
}

pub fn error_message(topic: String, title: String, message: String) -> NtfyMessage {
    NtfyMessage::new(topic)
        .title(title)
        .message(message)
        .add_tag("rotating_light".to_string())
        .priority(5)
} 

pub fn info_message(topic: String, title: String, message: String) -> NtfyMessage {
    NtfyMessage::new(topic)
        .title(title)
        .message(message)
        .add_tag("information".to_string())
        .priority(3)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_warning_message() {
        let message = warning_message("afg".to_string(), "test".to_string(), "test".to_string());
        message.send().await.unwrap();
        let message = success_message("afg".to_string(), "test".to_string(), "test".to_string());
        message.send().await.unwrap();
        let message = error_message("afg".to_string(), "test".to_string(), "test".to_string());
        message.send().await.unwrap();
        let message = info_message("afg".to_string(), "test".to_string(), "test".to_string());
        message.send().await.unwrap();
    }

    #[tokio::test]
    async fn test_http_action() {
        let action_msg = NtfyMessage::new("afg".to_string())
            .title("test".to_string())
            .message("test".to_string())
            .add_action(NtfyAction::new().http_action("go to baidu".to_string(), "https://www.baidu.com".to_string(), "GET".to_string(), HashMap::new(), "".to_string()));
        action_msg.send().await.unwrap();
    }

    #[tokio::test]
    async fn test_view_action() {
        let action_msg = NtfyMessage::new("afg".to_string())
            .title("test".to_string())
            .message("test".to_string())
            .add_action(NtfyAction::new().view_action("go to baidu".to_string(), "https://www.baidu.com".to_string()));
        action_msg.send().await.unwrap();
    }
}

