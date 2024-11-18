// Copyright 2017-2021 int08h LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std;

use crate::kms::KmsError;
use crate::tag::Tag;

/// Error types generated by this implementation
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The associated tag was added to an `RtMessage` in non-increasing order.
    TagNotStrictlyIncreasing(Tag),

    /// The associated byte sequence does not correspond to a valid Roughtime tag.
    InvalidTag(Box<[u8]>),

    /// Invalid number of tags specified
    InvalidNumTags(u32),

    /// Tag value length exceeds length of source bytes
    InvalidValueLength(Tag, u32),

    /// Encoding failed. The associated String should provide more information.
    EncodingFailure(String),

    /// Request was less than 1024 bytes
    RequestTooShort,

    /// Request was larger than 1500 bytes
    RequestTooLarge,

    /// Offset was not 32-bit aligned
    InvalidAlignment(u32),

    /// Offset is outside of valid message range
    InvalidOffsetValue(u32),

    /// Could not convert bytes to message because bytes were too short
    MessageTooShort,

    /// Otherwise invalid request
    InvalidRequest,

    /// Otherwise invalid response
    InvalidResponse,

    /// Runtime configuration is invalid for the reason provided
    InvalidConfiguration(String),

    /// The message length reported by the frame length != the actual message payload length
    LengthMismatch(u32, u32),

    /// Request did not provide versions compatible with this implementation
    NoCompatibleVersion,

    /// Sending response to a client request has failed
    SendingResponseFailed,

    /// The request's SRV value and this server's SRV value do not match
    SrvMismatch,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::EncodingFailure(err.to_string())
    }
}

impl From<KmsError> for Error {
    fn from(err: KmsError) -> Self {
        match err {
            KmsError::OperationFailed(m) => {
                Error::InvalidConfiguration(format!("KMS operation failed: {}", m))
            }
            KmsError::InvalidConfiguration(m) => {
                Error::InvalidConfiguration(format!("invalid KMS config: {}", m))
            }
            KmsError::InvalidData(m) => {
                Error::InvalidConfiguration(format!("invalid KMS data: {}", m))
            }
            KmsError::InvalidKey(m) => {
                Error::InvalidConfiguration(format!("invalid KMS key: {}", m))
            }
        }
    }
}
