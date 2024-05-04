use kernel::{
    TestContractHandler,
    ContractOne,
};
use nanoservices_utils::errors::NanoServiceError;


#[tokio::main]
async fn main() -> Result<(), NanoServiceError> {
    let contract_one = TestContractHandler::ContractOne(ContractOne {
        input_data: 5,
        result: None,
        error: None,
    });

    let result = contract_one.send_over_tcp("127.0.0.1:8080").await?.ContractOne()?;
    println!("{:?}", result);

    let contract_one = TestContractHandler::ContractOne(ContractOne {
        input_data: -5,
        result: None,
        error: None,
    });
    let result = contract_one.send_over_tcp("127.0.0.1:8080").await?.ContractOne()?;
    println!("{:?}", result);
    Ok(())
}
