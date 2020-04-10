use tokio::sync::Mutex;
use std::sync::Arc;
use super::{Context, FutureOutcome};
use crate::model::{raw::RawUpdate, ChosenInlineResult, InlineQuery, Message, Update};

pub(crate) type MessageHandlerFunc = fn(Context, Message) -> FutureOutcome;
pub(crate) type InlineQueryHandlerFunc = fn(Context, InlineQuery) -> FutureOutcome;
pub(crate) type InlineResultHandlerFunc = fn(Context, ChosenInlineResult) -> FutureOutcome;

pub(crate) type EventHandlerFunc = fn(Context, Update) -> FutureOutcome;
pub(crate) type RawEventHandlerFunc =
    fn(Context, RawUpdate) -> FutureOutcome;

#[derive(Clone)]
pub struct EventHandler {
    inner: Arc<Mutex<EventHandlerFunc>>,
}

impl EventHandler {
    pub fn new(handler: EventHandlerFunc) -> Self
    {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: Update) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

#[derive(Clone)]
pub struct RawEventHandler {
    inner: Arc<Mutex<RawEventHandlerFunc>>,
}

impl RawEventHandler {
    pub fn new(handler: RawEventHandlerFunc) -> Self
    {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: RawUpdate) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

#[derive(Clone)]
pub struct MessageHandler {
    inner: Arc<Mutex<MessageHandlerFunc>>,
}

impl MessageHandler {
    pub fn new(handler: MessageHandlerFunc) -> Self
    {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: Message) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<MessageHandlerFunc> for MessageHandler {
    fn from(func: MessageHandlerFunc) -> MessageHandler {
        Self::new(func)
    }
}

#[derive(Clone)]
pub struct InlineQueryHandler {
    inner: Arc<Mutex<InlineQueryHandlerFunc>>,
}

impl InlineQueryHandler {
    pub fn new(handler: InlineQueryHandlerFunc) -> Self
    {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: InlineQuery) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<InlineQueryHandlerFunc> for InlineQueryHandler {
    fn from(func: InlineQueryHandlerFunc) -> InlineQueryHandler {
        Self::new(func)
    }
}

#[derive(Clone)]
pub struct InlineResultHandler {
    inner: Arc<Mutex<InlineResultHandlerFunc>>,
}

impl InlineResultHandler {
    pub fn new(handler: InlineResultHandlerFunc) -> Self
    {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: ChosenInlineResult) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<InlineResultHandlerFunc> for InlineResultHandler {
    fn from(func: InlineResultHandlerFunc) -> InlineResultHandler {
        Self::new(func)
    }
}
