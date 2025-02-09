//! Compatibility functions for rpc `Log` type.

/// Creates a new rpc Log from a primitive log type from DB
#[inline]
pub fn from_primitive_log(log: reth_primitives::Log) -> reth_rpc_types::Log {
    reth_rpc_types::Log {
        address: log.address,
        topics: log.topics,
        data: log.data,
        block_hash: None,
        block_number: None,
        transaction_hash: None,
        transaction_index: None,
        log_index: None,
        removed: false,
    }
}
/// Converts a primitive `AccessList` structure from the `reth_primitives` module into the
/// corresponding RPC type.
#[inline]
pub fn from_primitive_access_list(list: reth_primitives::AccessList) -> reth_rpc_types::AccessList {
    let converted_list: Vec<reth_rpc_types::AccessListItem> = list
        .0
        .into_iter()
        .map(|item| reth_rpc_types::AccessListItem {
            address: item.address,
            storage_keys: item.storage_keys,
        })
        .collect();

    reth_rpc_types::AccessList(converted_list)
}
