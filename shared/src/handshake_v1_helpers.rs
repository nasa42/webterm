use crate::generated::flatbuffers_schema::handshake_v1::{
    F2rHandshake, R2fHandshake, R2fHandshakeArgs,
};
use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};

pub fn create_r2f_handshake(success: bool, relay_auth_nonce: Option<&str>) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = R2fHandshakeArgs {
        success,
        relay_auth_nonce: Some(builder.create_string(relay_auth_nonce.unwrap_or_default())),
    };

    let message = R2fHandshake::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn read_f2r_handshake(data: &[u8]) -> Result<F2rHandshake, InvalidFlatbuffer> {
    flatbuffers::root::<F2rHandshake>(data)
}
