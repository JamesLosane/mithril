use crate::entities::{Beacon, CertificateMetadata, ProtocolMessage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Certificate represents a Mithril certificate embedding a Mithril STM multisignature
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Certificate {
    /// Hash of the current certificate
    /// Computed from the other fields of the certificate
    /// aka H(Cp,n))
    #[serde(rename = "hash")]
    pub hash: String,

    /// Hash of the previous certificate in the chain
    /// This is either the hash of the first certificate of the epoch in the chain
    /// Or the first certificate of the previous epoch in the chain (if the certificate is the first of its epoch)
    /// aka H(FC(n))
    #[serde(rename = "previous_hash")]
    pub previous_hash: String,

    /// Mithril beacon on the Cardano chain
    /// aka BEACON(p,n)
    #[serde(rename = "beacon")]
    pub beacon: Beacon,

    /// Certificate metadata
    /// aka METADATA(p,n)
    #[serde(rename = "metadata")]
    pub metadata: CertificateMetadata,

    /// Structured message that is used to created the signed message
    /// aka MSG(p,n) U AVK(n-1)
    #[serde(rename = "protocol_message")]
    pub protocol_message: ProtocolMessage,

    /// Message that is signed by the signers
    /// aka H(MSG(p,n) || AVK(n-1))
    #[serde(rename = "signed_message")]
    pub signed_message: String,

    /// Aggregate verification key
    /// The AVK used to sign during the current epoch
    /// aka AVK(n-2)
    #[serde(rename = "aggregate_verification_key")]
    pub aggregate_verification_key: String,

    /// STM multi signature created from a quorum of single signatures from the signers
    /// aka MULTI_SIG(H(MSG(p,n) || AVK(n-1)))
    #[serde(rename = "multi_signature")]
    pub multi_signature: String,

    /// Genesis signature created from the original stake distribution
    /// aka GENESIS_SIG(AVK(-1))
    #[serde(rename = "genesis_signature")]
    pub genesis_signature: String,
}

impl Certificate {
    /// Certificate factory
    pub fn new(
        previous_hash: String,
        beacon: Beacon,
        metadata: CertificateMetadata,
        protocol_message: ProtocolMessage,
        aggregate_verification_key: String,
        multi_signature: String,
        genesis_signature: String,
    ) -> Certificate {
        let signed_message = (&protocol_message.compute_hash()).to_owned();
        let mut certificate = Certificate {
            hash: "".to_string(),
            previous_hash,
            beacon,
            metadata,
            protocol_message,
            signed_message,
            aggregate_verification_key,
            multi_signature,
            genesis_signature,
        };
        certificate.hash = certificate.compute_hash();
        certificate
    }

    /// Computes the hash of a Certificate
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.beacon.compute_hash().as_bytes());
        hasher.update(self.metadata.compute_hash().as_bytes());
        hasher.update(self.protocol_message.compute_hash().as_bytes());
        hasher.update(self.signed_message.as_bytes());
        hasher.update(self.aggregate_verification_key.as_bytes());
        hasher.update(self.multi_signature.as_bytes());
        hasher.update(self.genesis_signature.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ProtocolMessagePartKey, ProtocolParameters, SignerWithStake};

    #[test]
    fn test_certificate_compute_hash() {
        let hash_expected = "7d714a2005ceb4778efe8805c9efe139a65aa5c607b38befeee97a4518928a0c";

        let mut protocol_message = ProtocolMessage::new();
        protocol_message.set_message_part(
            ProtocolMessagePartKey::SnapshotDigest,
            "snapshot-digest-123".to_string(),
        );
        protocol_message.set_message_part(
            ProtocolMessagePartKey::NextAggregateVerificationKey,
            "next-avk-123".to_string(),
        );
        assert_eq!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash-modified".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet-modified".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0-modified".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        let mut protocol_message_modified = protocol_message.clone();
        protocol_message_modified.set_message_part(
            ProtocolMessagePartKey::NextAggregateVerificationKey,
            "next-avk-456".to_string(),
        );
        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message_modified.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key-modified".to_string(),
                "multi_signature".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature-modified".to_string(),
                "genesis_signature".to_string(),
            )
            .compute_hash()
        );

        assert_ne!(
            hash_expected,
            Certificate::new(
                "previous_hash".to_string(),
                Beacon::new("testnet".to_string(), 10, 100),
                CertificateMetadata::new(
                    "0.1.0".to_string(),
                    ProtocolParameters::new(1000, 100, 0.123),
                    "initiated_at".to_string(),
                    "sealed_at".to_string(),
                    vec![
                        SignerWithStake::new(
                            "1".to_string(),
                            "verification-key-123".to_string(),
                            10
                        ),
                        SignerWithStake::new(
                            "2".to_string(),
                            "verification-key-456".to_string(),
                            20
                        )
                    ],
                ),
                protocol_message.clone(),
                "aggregate_verification_key".to_string(),
                "multi_signature".to_string(),
                "genesis_signature-modified".to_string(),
            )
            .compute_hash()
        );
    }
}
