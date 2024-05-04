use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::{
    register_contract_routes,
    errors::{NanoServiceError, NanoServiceErrorStatus},
    networking::codec::BincodeCodec
};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

use kernel::{
    TestContractHandler,
    ContractOne,
    ContractTwo,
};


async fn handle_contract_one(mut contract: ContractOne) -> Result<ContractOne, NanoServiceError> {
    let data = contract.input_data;
    if data < 0 {
        contract.error = Some(NanoServiceError::new(
            "Input data must be a positive integer".to_string(),
            NanoServiceErrorStatus::BadRequest)
        );
    } else {
        contract.result = Some(data * 2);
    }
    Ok(contract)
}

async fn handle_contract_two(mut contract: ContractTwo) -> Result<ContractTwo, NanoServiceError> {
    let data = contract.input_data.clone();
    if data.is_empty() {
        contract.error = Some(NanoServiceError::new(
            "Input data must not be empty".to_string(),
            NanoServiceErrorStatus::BadRequest)
        );
    } else {
        contract.result = Some(data.to_uppercase());
    }
    Ok(contract)
}


register_contract_routes!(
    TestContractHandler,                  // Struct handling contract serialization
    handle_contract_routes,               // Generate an overall contract handler function of this name
    ContractOne => handle_contract_one,   // Map a contract struct to existing handler function
    ContractTwo => handle_contract_two    // Map a contract struct to existing handler function
);


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    while let Ok((socket, _)) = listener.accept().await {
        let mut framed = Framed::new(socket, BincodeCodec::<TestContractHandler>::new());

        while let Some(result) = framed.next().await {
            match result {
                Ok(data) => {
                    println!("Received: {:?}", data);
                    let response = handle_contract_routes(data).await.unwrap();
                    framed.send(response).await.unwrap();
                    break;
                },
                Err(e) => {
                    eprintln!("Error processing data: {}", e);
                    break;
                }
            }
        }
    }
}

