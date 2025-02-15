use std::io;

use aws_sdk_cognitoidentityprovider::operation::initiate_auth::InitiateAuthError;
use aws_sdk_cognitoidentityprovider::operation::respond_to_auth_challenge::RespondToAuthChallengeError;
use aws_smithy_client;
use cognito_srp::CognitoSrpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CognitoSrpAuthError {
    #[error("cognito srp error: {0}")]
    SrpError(#[from] CognitoSrpError),

    #[error("illegal argument: {0}")]
    IllegalArgument(String),

    #[error("io error: {0}")]
    IOError(#[from] io::Error),

    #[error("cognito idp initiate error: {0}")]
    CognitoInitiateError(#[from] aws_smithy_client::SdkError<InitiateAuthError>),

    #[error("cognito idp response to auth challenge error: {0}")]
    CognitoResponseToAuthChallengeError(
        #[from] aws_smithy_client::SdkError<RespondToAuthChallengeError>,
    ),
}
