use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

type Message = (String, String); // (message_type, message_value)

pub struct Messenger {
    message_sink: Sender<Message>,
}

impl Messenger {
    // 创建一个新的 Messenger 实例
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100); // 创建一个大小为 100 的广播通道
        Messenger { message_sink: tx }
    }

    // 获取 message_sink 的引用
    pub fn message_sink(&self) -> Sender<Message> {
        self.message_sink.clone()
    }

    // 模拟发送消息的方法
    pub async fn send_message(&self, message_type: String, message_value: String) {
        let message = (message_type, message_value);
        let _ = self.message_sink.send(message); // 向所有接收者广播消息
    }
}

// 接收消息的函数
async fn message_handler(mut rx: broadcast::Receiver<Message>) {
    loop {
        match rx.recv().await {
            Ok((message_type, message_value)) => {
                // 在这里处理消息
            }
            Err(_) => {
                break; // 通常表示发送者已经关闭了广播通道
            }
        }
    }
}
