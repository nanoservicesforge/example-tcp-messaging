use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::{
    create_contract_handler,
    errors::{NanoServiceError, NanoServiceErrorStatus},
    networking::codec::BincodeCodec
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContractOne {
    pub input_data: i32,
    pub result: Option<i32>,
    pub error: Option<NanoServiceError>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContractTwo {
    pub input_data: String,
    pub result: Option<String>,
    pub error: Option<NanoServiceError>,
}


create_contract_handler!(
    TestContractHandler, // this handler struct is created by the macro
    ContractOne,
    ContractTwo
);
