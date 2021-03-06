// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SendSSHPublicKeyRequest {
    /// <p>The availability zone the EC2 instance was launched in.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>The EC2 instance you wish to publish the SSH key to.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// <p>The OS user on the EC2 instance whom the key may be used to authenticate as.</p>
    #[serde(rename = "InstanceOSUser")]
    pub instance_os_user: String,
    /// <p>The public key to be published to the instance. To use it after publication you must have the matching private key.</p>
    #[serde(rename = "SSHPublicKey")]
    pub ssh_public_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendSSHPublicKeyResponse {
    /// <p>The request ID as logged by EC2 Connect. Please provide this when contacting AWS Support.</p>
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Indicates request success.</p>
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// Errors returned by SendSSHPublicKey
#[derive(Debug, PartialEq)]
pub enum SendSSHPublicKeyError {
    /// <p>Indicates that either your AWS credentials are invalid or you do not have access to the EC2 instance.</p>
    Auth(String),
    /// <p>Indicates that the instance requested was not found in the given zone. Check that you have provided a valid instance ID and the correct zone.</p>
    EC2InstanceNotFound(String),
    /// <p>Indicates that you provided bad input. Ensure you have a valid instance ID, the correct zone, and a valid SSH public key.</p>
    InvalidArgs(String),
    /// <p>Indicates that the service encountered an error. Follow the message's instructions and try again.</p>
    Service(String),
    /// <p>Indicates you have been making requests too frequently and have been throttled. Wait for a while and try again. If higher call volume is warranted contact AWS Support.</p>
    Throttling(String),
}

impl SendSSHPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendSSHPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Auth(err.msg))
                }
                "EC2InstanceNotFoundException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::EC2InstanceNotFound(
                        err.msg,
                    ))
                }
                "InvalidArgsException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::InvalidArgs(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Service(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SendSSHPublicKeyError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SendSSHPublicKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SendSSHPublicKeyError {
    fn description(&self) -> &str {
        match *self {
            SendSSHPublicKeyError::Auth(ref cause) => cause,
            SendSSHPublicKeyError::EC2InstanceNotFound(ref cause) => cause,
            SendSSHPublicKeyError::InvalidArgs(ref cause) => cause,
            SendSSHPublicKeyError::Service(ref cause) => cause,
            SendSSHPublicKeyError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the EC2 Instance Connect API. EC2 Instance Connect clients implement this trait.
pub trait Ec2InstanceConnect {
    /// <p>Pushes an SSH public key to a particular OS user on a given EC2 instance for 60 seconds.</p>
    fn send_ssh_public_key(
        &self,
        input: SendSSHPublicKeyRequest,
    ) -> RusotoFuture<SendSSHPublicKeyResponse, SendSSHPublicKeyError>;
}
/// A client for the EC2 Instance Connect API.
#[derive(Clone)]
pub struct Ec2InstanceConnectClient {
    client: Client,
    region: region::Region,
}

impl Ec2InstanceConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> Ec2InstanceConnectClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> Ec2InstanceConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> Ec2InstanceConnectClient {
        Ec2InstanceConnectClient { client, region }
    }
}

impl fmt::Debug for Ec2InstanceConnectClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ec2InstanceConnectClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Ec2InstanceConnect for Ec2InstanceConnectClient {
    /// <p>Pushes an SSH public key to a particular OS user on a given EC2 instance for 60 seconds.</p>
    fn send_ssh_public_key(
        &self,
        input: SendSSHPublicKeyRequest,
    ) -> RusotoFuture<SendSSHPublicKeyResponse, SendSSHPublicKeyError> {
        let mut request = SignedRequest::new("POST", "ec2-instance-connect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSEC2InstanceConnectService.SendSSHPublicKey",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SendSSHPublicKeyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SendSSHPublicKeyError::from_response(response))),
                )
            }
        })
    }
}
