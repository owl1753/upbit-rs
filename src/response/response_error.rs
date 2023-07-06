use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ResponseErrorState {
    InternalNeitherParameterSpecified, //either parameter uuid or identifier must be specified.
    InternalTooManyParameterSpecified, //only one parameter of uuid and identifier must be specified.
    InternalReqwestError,
    InternalHmacError,
    InternalTokenEncodeError,
    InternalJsonParseError,
    InvalidQueryPayload,
    InvalidAccessKey,
    InvalideVolumeBid, //"주문수량 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    InvalidPriceBid, //"주문가격 단위를 잘못 입력하셨습니다. 확인 후 시도해주세요."
    InvalidParameter, // "잘못된 파라미터"
    OrderNotFound, // "주문을 찾지 못했습니다."
    UnderMinTotalAsk, //"최소주문금액 이상으로 주문해주세요"
    UnderMinTotalBid, //"최소주문금액 이상으로 주문해주세요"
    JwtVerificationError, //"Failed to verify Jwt token."
    CreateAskError,
    CreateBidError,
    InsufficientFundsAsk,
    InsufficientFundsBid,
    ExpiredAccessKey,
    NonceUsed,
    NoAuthorizationIp,
    OutOfScope,
    WithdrawAddressNotRegisterd,
    NotSupportedOrdType, //"현재 해당 마켓에서 지원하지 않는 주문입니다. 주문 조건을 다시 확인해주시기 바랍니다."
    UnexpectedError
}

impl From<&str> for ResponseErrorState {
    fn from(value: &str) -> Self {
        match value {
            "internal_neither_parameter_specified" => ResponseErrorState::InternalNeitherParameterSpecified,
            "internal_more_parameter_specified" => ResponseErrorState::InternalTooManyParameterSpecified,
            "internal_reqwest_error" => ResponseErrorState::InternalReqwestError,
            "internal_hmac_error" => ResponseErrorState::InternalHmacError,
            "internal_token_encode_error" => ResponseErrorState::InternalTokenEncodeError,
            "jwt_verification" => ResponseErrorState::JwtVerificationError,
            "expired_access_key" => ResponseErrorState::ExpiredAccessKey,
            "invalid_query_payload" => ResponseErrorState::InvalidQueryPayload,
            "invalid_access_key" => ResponseErrorState::InvalidAccessKey,
            "invalid_volume_bid" => ResponseErrorState::InvalideVolumeBid,
            "invalid_price_bid" => ResponseErrorState::InvalidPriceBid,
            "invalid_parameter" => ResponseErrorState::InvalidParameter,
            "under_min_total_ask" => ResponseErrorState::UnderMinTotalAsk,
            "under_min_total_bid" => ResponseErrorState::UnderMinTotalBid,
            "insufficient_funds_ask" => ResponseErrorState::InsufficientFundsAsk,
            "insufficient_funds_bid" => ResponseErrorState::InsufficientFundsBid,
            "create_ask_error" => ResponseErrorState::CreateAskError,
            "create_bid_error" => ResponseErrorState::CreateBidError,
            "nonce_used" => ResponseErrorState::NonceUsed,
            "no_authorization_i_p" => ResponseErrorState::NoAuthorizationIp,
            "out_of_scope" => ResponseErrorState::OutOfScope,
            "withdraw_address_not_registerd" => ResponseErrorState::WithdrawAddressNotRegisterd,
            "order_not_found" => ResponseErrorState::OrderNotFound,
            "not_supported_ord_type" => ResponseErrorState::NotSupportedOrdType, 
            "server_error" => ResponseErrorState::UnexpectedError,
            _ => ResponseErrorState::UnexpectedError
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub state: ResponseErrorState,
    pub error: ResponseErrorBody
}

#[derive(Deserialize, Debug)]
pub struct ResponseErrorSource {
    pub error: ResponseErrorBody
}

#[derive(Deserialize, Debug)]
pub struct ResponseErrorBody {
    pub name: String,
    pub message: String,
}