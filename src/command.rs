
trait MethodId {
    fn method_id(&self) -> u16;
}

pub enum ConnectionMethod {
    Start,
    StartOk,
    Secure,
    SecureOk,
    Tune,
    TuneOk,
    Open,
    OpenOk,
    Close,
    CloseOk
}

impl MethodId for ConnectionMethod {
    fn method_id(&self) -> u16 {
        match self {
            ConnectionMethod::Start => 10,
            ConnectionMethod::StartOk => 11,
            ConnectionMethod::Secure => 20,
            ConnectionMethod::SecureOk => 21,
            ConnectionMethod::Tune => 30,
            ConnectionMethod::TuneOk => 31,
            ConnectionMethod::Open => 40,
            ConnectionMethod::OpenOk => 41,
            ConnectionMethod::Close => 50,
            ConnectionMethod::CloseOk => 51
        }
    }
}

pub enum ChannelMethod {
    Open,
    OpenOk,
    Flow,
    FlowOk,
    Close,
    CloseOk
}

impl MethodId for ChannelMethod {
    fn method_id(&self) -> u16 {
        match self {
            ChannelMethod::Open => 10,
            ChannelMethod::OpenOk => 11,
            ChannelMethod::Flow => 20,
            ChannelMethod::FlowOk => 21,
            ChannelMethod::Close => 40,
            ChannelMethod::CloseOk => 41
        }
    }
}

pub enum ExchangeMethod {
    Declare,
    DeclareOk,
    Delete,
    DeleteOk
}

impl MethodId for ExchangeMethod {
    fn method_id(&self) -> u16 {
        match self {
            ExchangeMethod::Declare => 10,
            ExchangeMethod::DeclareOk => 11,
            ExchangeMethod::Delete => 20,
            ExchangeMethod::DeleteOk => 21
        }
    }
}

pub enum QueueMethod {
    Declare,
    DeclareOk,
    Bind,
    BindOk,
    Unbind,
    UnbindOk,
    Purge,
    PurgeOk,
    Delete,
    DeleteOk
}

impl MethodId for QueueMethod {
    fn method_id(&self) -> u16 {
        match self {
            QueueMethod::Declare => 10,
            QueueMethod::DeclareOk => 11,
            QueueMethod::Bind => 20,
            QueueMethod::BindOk => 21,
            QueueMethod::Unbind => 50,
            QueueMethod::UnbindOk => 51,
            QueueMethod::Purge => 30,
            QueueMethod::PurgeOk => 31,
            QueueMethod::Delete => 40,
            QueueMethod::DeleteOk => 41
        }
    }
}

pub enum BasicMethod {
    Qos,
    QosOk,
    Consume,
    ConsumeOk,
    Cancel,
    CancelOk,
    Publish,
    Return,
    Deliver,
    Get,
    GetOk,
    GetEmpty,
    Ack,
    Reject,
    RecoverAsync,
    Recover,
    RecoverOk
}

impl MethodId for BasicMethod {
    fn method_id(&self) -> u16 {
        match self {
            BasicMethod::Qos => 10,
            BasicMethod::QosOk => 11,
            BasicMethod::Consume => 20,
            BasicMethod::ConsumeOk => 21,
            BasicMethod::Cancel => 30,
            BasicMethod::CancelOk => 31,
            BasicMethod::Publish => 40,
            BasicMethod::Return => 50,
            BasicMethod::Deliver => 60,
            BasicMethod::Get => 70,
            BasicMethod::GetOk => 71,
            BasicMethod::GetEmpty => 72,
            BasicMethod::Ack => 80,
            BasicMethod::Reject => 90,
            BasicMethod::RecoverAsync => 100,
            BasicMethod::Recover => 110,
            BasicMethod::RecoverOk => 111
        }
    }
}

pub enum TxMethod {
    Select,
    SelectOk,
    Commit,
    CommitOk,
    Rollback,
    RollbackOk
}

impl MethodId for TxMethod {
    fn method_id(&self) -> u16 {
        match self {
            TxMethod::Select => 10,
            TxMethod::SelectOk => 11,
            TxMethod::Commit => 20,
            TxMethod::CommitOk => 21,
            TxMethod::Rollback => 30,
            TxMethod::RollbackOk => 31
        }
    }
}

pub enum Class {
    Connection(ConnectionMethod),
    Channel(ChannelMethod),
    Exchange(ExchangeMethod),
    Queue(QueueMethod),
    Basic(BasicMethod),
    Tx(TxMethod),
}

impl Class {
    pub fn class_id(&self) -> u16 {
        match self {
            Class::Connection(_) => 10,
            Class::Channel(_) => 20,
            Class::Exchange(_) => 40,
            Class::Queue(_) => 50,
            Class::Basic(_) => 60,
            Class::Tx(_) => 90
        }
    }
}