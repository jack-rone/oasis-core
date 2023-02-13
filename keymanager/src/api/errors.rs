use thiserror::Error;

/// Key manager error.
#[derive(Error, Debug)]
pub enum KeyManagerError {
    #[error("client session is not authenticated")]
    NotAuthenticated,
    #[error("client is not authorized")]
    NotAuthorized,
    #[error("invalid epoch: expected {0}, got {1}")]
    InvalidEpoch(u64, u64),
    #[error("invalid generation: expected {0}, got {1}")]
    InvalidGeneration(u64, u64),
    #[error("generation is in the future: expected max {0}, got {1}")]
    GenerationFromFuture(u64, u64),
    #[error("height is not fresh")]
    HeightNotFresh,
    #[error("key manager is not initialized")]
    NotInitialized,
    #[error("key manager state corrupted")]
    StateCorrupted,
    #[error("key manager replication required")]
    ReplicationRequired,
    #[error("policy rollback")]
    PolicyRollback,
    #[error("policy alteration, without serial increment")]
    PolicyChanged,
    #[error("policy has invalid runtime")]
    PolicyInvalidRuntime,
    #[error("policy is malformed or invalid: {0}")]
    PolicyInvalid(#[from] anyhow::Error),
    #[error("policy has insufficient signatures")]
    PolicyInsufficientSignatures,
    #[error("runtime signing key missing")]
    RSKMissing,
    #[error("runtime encryption key not published")]
    REKNotPublished,
    #[error("signature verification failed: {0}")]
    InvalidSignature(anyhow::Error),
    #[error("master secret generation {0} not found")]
    MasterSecretNotFound(u64),
    #[error("master secret generation {0} not replicated")]
    MasterSecretNotReplicated(u64),
    #[error("ephemeral secret for epoch {0} not found")]
    EphemeralSecretNotFound(u64),
    #[error("ephemeral secret for epoch {0} not replicated")]
    EphemeralSecretNotReplicated(u64),
    #[error("ephemeral secret not published")]
    EphemeralSecretNotPublished,
    #[error("ephemeral secret checksum mismatch")]
    EphemeralSecretChecksumMismatch,
    #[error("invalid ciphertext")]
    InvalidCiphertext,
    #[error("status not found")]
    StatusNotFound,
    #[error("runtime mismatch")]
    RuntimeMismatch,
    #[error("active deployment not found")]
    ActiveDeploymentNotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}
