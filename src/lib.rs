use std::fmt::Debug;
use cw_multi_test::{ContractWrapper, Contract, Executor, App, AppResponse};
use cosmwasm_std::{Empty, Addr, QuerierWrapper};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
pub use cw721_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg}; // re-export cw721_base messages for convenience
use cw721_base::{helpers::Cw721Contract};
use cw721::{
    OwnerOfResponse, 
    TokensResponse, 
    NumTokensResponse, 
    NftInfoResponse, 
    ApprovalResponse, 
    ApprovalsResponse, 
    OperatorsResponse, 
    AllNftInfoResponse, 
    ContractInfoResponse
};

pub fn cw721_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    );
    Box::new(contract) // create new cw721 contract code storage. We'll store and execute on it later
}

pub fn store(app: &mut App) -> Result<u64, Box<dyn std::error::Error>> {
    let contract = cw721_contract();
    let id: u64 = app.store_code(contract);
    Ok(id)
}

pub fn instantiate(
    app: &mut App,
    code_id: u64,
    msg: &cw721_base::msg::InstantiateMsg,
) -> Result<Addr, Box<dyn std::error::Error>> {
    let res = app.instantiate_contract(
        code_id, 
        Addr::unchecked("owner"), 
        msg,
        &vec![],
        "Instantiate cw721 contract",
        None
    )?;
    Ok(res)
}

pub fn execute<T: Serialize + Debug + Deserialize<'static> + JsonSchema>(
    app: &mut App,
    contract_addr: Addr,
    msg: &cw721_base::msg::ExecuteMsg<T>,
) -> Result<AppResponse, Box<dyn std::error::Error>> {
    let res = app.execute_contract(
        contract_addr, 
        Addr::unchecked("owner"),
        msg,
        &vec![]
    )?;
    Ok(res)
}

pub fn query<T: Serialize + Debug + JsonSchema + for<'de> Deserialize<'de>>(
    contract: Cw721Contract,
    query: QueryMsg,
    querier: &QuerierWrapper,
) -> Result<QueryResponse<T>, Box<dyn std::error::Error>> {
    match query {
        QueryMsg::OwnerOf { token_id, include_expired } => Ok(QueryResponse::OwnerOf(contract.owner_of(querier, token_id, include_expired.unwrap_or(false))?)),
        QueryMsg::Tokens { owner, start_after, limit } => Ok(QueryResponse::Tokens(contract.tokens(querier, owner, start_after, limit)?)),
        QueryMsg::NumTokens { } => Ok(QueryResponse::NumTokens(NumTokensResponse { count: contract.num_tokens(querier)?})),
        QueryMsg::NftInfo { token_id } => Ok(QueryResponse::NftInfo(contract.nft_info(querier, token_id)?)),
        QueryMsg::Approval { token_id, include_expired, spender } => Ok(QueryResponse::Approval(contract.approval(querier, token_id, spender, include_expired)?)),
        QueryMsg::Approvals { token_id, include_expired } => Ok(QueryResponse::Approvals(contract.approvals(querier, token_id, include_expired)?)),
        QueryMsg::AllOperators { owner, start_after, limit, include_expired  } => Ok(QueryResponse::Operators(OperatorsResponse { operators: contract.all_operators(querier, owner, include_expired.unwrap_or(false), start_after, limit)?})),
        QueryMsg::AllNftInfo { token_id, include_expired } => Ok(QueryResponse::AllNftInfo(contract.all_nft_info(querier, token_id, include_expired.unwrap_or(false))?)),
        QueryMsg::ContractInfo { } => Ok(QueryResponse::ContractInfo(contract.contract_info(querier)?)),
        QueryMsg::AllTokens { start_after, limit } => Ok(QueryResponse::Tokens(contract.all_tokens(querier, start_after, limit)?)),
        QueryMsg::Minter {  } => Ok(QueryResponse::DeprecatedQueryResponse { method: "'minter' query is deprecated.".to_string() }),
    }
}

pub enum QueryResponse<T> {
    OwnerOf(OwnerOfResponse),
    Tokens(TokensResponse),
    NumTokens(NumTokensResponse),
    NftInfo(NftInfoResponse<T>),
    Approval(ApprovalResponse),
    Approvals(ApprovalsResponse),
    Operators(OperatorsResponse),
    AllNftInfo(AllNftInfoResponse<T>),
    ContractInfo(ContractInfoResponse),
    DeprecatedQueryResponse {
        method: String
    }
}