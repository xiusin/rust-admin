use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum Event {
    #[serde(rename = "user.registered")]
    UserRegistered {
        consumer_id: i64,
        phone: String,
        tenant_id: i64,
    },
    #[serde(rename = "payment.success")]
    PaymentSuccess {
        order_no: String,
        consumer_id: i64,
        amount: String,
        tenant_id: i64,
    },
    #[serde(rename = "logistics.status_changed")]
    LogisticsStatusChanged {
        tracking_no: String,
        status: String,
        tenant_id: i64,
    },
}

pub struct EventBus;

impl EventBus {
    pub fn new() -> Self {
        Self
    }

    pub async fn publish(&self, event: Event) -> Result<(), EventBusError> {
        let channel = match &event {
            Event::UserRegistered { .. } => "user.registered",
            Event::PaymentSuccess { .. } => "payment.success",
            Event::LogisticsStatusChanged { .. } => "logistics.status_changed",
        };

        let payload = serde_json::to_string(&event)
            .map_err(|e| EventBusError::SerializeError(e.to_string()))?;

        tracing::info!("Publishing event to channel {}: {}", channel, payload);

        Ok(())
    }
}

#[derive(Debug)]
pub enum EventBusError {
    RedisError(String),
    SerializeError(String),
    HandlerError(String),
}

impl std::fmt::Display for EventBusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventBusError::RedisError(msg) => write!(f, "Redis error: {}", msg),
            EventBusError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            EventBusError::HandlerError(msg) => write!(f, "Handler error: {}", msg),
        }
    }
}

impl std::error::Error for EventBusError {}

#[derive(Clone)]
pub struct EventHandlers;

impl EventHandlers {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_user_registered(&self, payload: &str) -> Result<(), EventBusError> {
        let event: Event = serde_json::from_str(payload)
            .map_err(|e| EventBusError::HandlerError(e.to_string()))?;

        if let Event::UserRegistered {
            consumer_id,
            phone,
            tenant_id,
        } = event
        {
            tracing::info!(
                "Handling user registered event: consumer_id={}, phone={}, tenant_id={}",
                consumer_id,
                phone,
                tenant_id
            );
        }

        Ok(())
    }

    pub async fn handle_payment_success(&self, payload: &str) -> Result<(), EventBusError> {
        let event: Event = serde_json::from_str(payload)
            .map_err(|e| EventBusError::HandlerError(e.to_string()))?;

        if let Event::PaymentSuccess {
            order_no,
            consumer_id,
            amount,
            tenant_id,
        } = event
        {
            tracing::info!(
                "Handling payment success event: order_no={}, consumer_id={}, amount={}, tenant_id={}",
                order_no,
                consumer_id,
                amount,
                tenant_id
            );
        }

        Ok(())
    }

    pub async fn handle_logistics_status_changed(&self, payload: &str) -> Result<(), EventBusError> {
        let event: Event = serde_json::from_str(payload)
            .map_err(|e| EventBusError::HandlerError(e.to_string()))?;

        if let Event::LogisticsStatusChanged {
            tracking_no,
            status,
            tenant_id,
        } = event
        {
            tracing::info!(
                "Handling logistics status changed event: tracking_no={}, status={}, tenant_id={}",
                tracking_no,
                status,
                tenant_id
            );
        }

        Ok(())
    }
}