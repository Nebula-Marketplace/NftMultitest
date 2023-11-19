# NFTMultiTest
NFTMultiTest is a cw721 testing package (does not yet support NBLA-721) for seamlessly adding Cw721 support to your cw-multi-test tests. 

## Usage
```rs
use nft_multi_test::{cw721_contract, instantiate, execute, query, ExecuteMsg}
use crate::contract::{execute, instantiate, query}

#[test]
fn init() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let nft_id = app.store_code(cw721_contract());
    
    let nft_contract = app.instantiate_contract(
        nft_id,
        Addr::unchecked("owner"),
        &cw721_base::msg::InstantiateMsg {
            name: "Nebula NFT".to_string(),
            symbol: "NFT".to_string(),
            minter: "owner".to_string()
        },
        &vec![],
        "Nebula NFT",
        None
    ).expect("couldn't instantiate nft contract");

    app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        &InstantiateMsg {
            collection: "collection".to_string(),
            contract: "contract".to_string(),
            description: "description".to_string(),
            symbol: "symbol".to_string(),
            logo_uri: "logo_uri".to_string(),
            banner_uri: "banner_uri".to_string(),
            supply: 100,
            creators: vec![Creator {
                address: "creator".to_string(),
                share: 100,
            }],
            basis_points: 100,
            phases: vec![],
            codeid: 2619,
        }, 
        &vec![], 
        "Test", 
        None
    ).expect("contract failed to instantiate");
}
```