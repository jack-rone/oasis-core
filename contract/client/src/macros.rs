// This is re-exported here only so it can be used in macros under a common name.
pub use ekiden_common::bytes::B256;
pub use ekiden_common::environment::Environment;
pub use ekiden_common::futures::{BoxFuture, Future};
pub use ekiden_common::signature::Signer;
pub use ekiden_consensus_base::backend::ConsensusBackend;
pub use ekiden_enclave_common::quote;
pub use ekiden_registry_base::EntityRegistryBackend;
pub use ekiden_rpc_client::backend::RpcClientBackend;
pub use ekiden_scheduler_base::Scheduler;
pub use ekiden_storage_base::backend::StorageBackend;

/// Create a contract client for a given API.
///
/// # Examples
///
/// This macro should be invoked using a concrete API generated by `contract_api` as
/// follows:
/// ```
/// with_api! {
///     create_contract_client!(foo, foo_api, api);
/// }
/// ```
///
/// In this example, the generated client will be put into a module called `foo`
/// which will use API structures from module `foo_api`. The API definitions will
/// passed as the last argument as defined by the `api` token.
#[macro_export]
macro_rules! create_contract_client {
    (
        $output_module: ident,
        $api_module: path,

        $(
            pub fn $method_name: ident ( $request_type: ty ) -> $response_type: ty ;
        )*
    ) => {
        mod $output_module {
            use std::sync::Arc;
            use std::time::Duration;

            use $crate::manager::ContractClientManager;
            use $crate::macros::quote::MrEnclave;
            use $crate::macros::*;

            pub use $api_module::*;

            pub struct Client {
                manager: ContractClientManager,
            }

            #[allow(dead_code)]
            impl Client {
                /// Create new client instance.
                pub fn new(
                    contract_id: B256,
                    mr_enclave: MrEnclave,
                    timeout: Option<Duration>,
                    environment: Arc<Environment>,
                    scheduler: Arc<Scheduler>,
                    entity_registry: Arc<EntityRegistryBackend>,
                    signer: Arc<Signer>,
                    consensus: Arc<ConsensusBackend>,
                    storage: Arc<StorageBackend>,
                ) -> Self {
                    Client {
                        manager: ContractClientManager::new(
                            contract_id,
                            mr_enclave,
                            timeout,
                            environment,
                            scheduler,
                            entity_registry,
                            signer,
                            consensus,
                            storage,
                        ),
                    }
                }

                // Generate methods.
                $(
                    pub fn $method_name(
                        &self,
                        arguments: $request_type
                    ) -> BoxFuture<$response_type> {
                        self.manager.call(stringify!($method_name), arguments)
                    }
                )*
            }
        }
    };
}
