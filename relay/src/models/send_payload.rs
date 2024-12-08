use crate::models::relay_error::RelayError;
use crate::models::socket_writer::SocketPublisher;
use webterm_core::serialisers::talk_v1::r2a_builder::R2aRootBlob;
use webterm_core::serialisers::talk_v1::r2f_builder::R2fRootBlob;
use webterm_core::types::FrontendId;

pub struct SendPayload {
    to_frontend: Option<(FrontendId, R2fRootBlob)>,
    to_agent: Option<R2aRootBlob>,
}

impl SendPayload {
    pub fn new() -> Self {
        SendPayload {
            to_frontend: None,
            to_agent: None,
        }
    }

    pub async fn dispatch(
        self,
        frontend_pub: &SocketPublisher,
        agent_pub: &SocketPublisher,
    ) -> Result<(), RelayError> {
        if let Some((_to_frontend_id, data)) = self.to_frontend {
            frontend_pub.send(data.0).await?;
        }

        if let Some(data) = self.to_agent {
            agent_pub.send(data.0).await?;
        }

        Ok(())
    }

    pub fn to_frontend_id(&self) -> Option<FrontendId> {
        self.to_frontend.as_ref().map(|(id, _)| *id)
    }

    pub fn prepare_for_frontend(&mut self, to_frontend_id: FrontendId, data: R2fRootBlob) {
        self.to_frontend = Some((to_frontend_id, data));
    }

    pub fn prepare_for_agent(&mut self, data: R2aRootBlob) {
        self.to_agent = Some(data);
    }
}
