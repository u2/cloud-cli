# 命令介绍

`cldi`是对区块链定义的对外的gRPC接口的封装。

这里列出这些接口以供参考。

## controller的接口

```protobuf
service RPCService {
    // flag means latest or pending.
    // true means pending, false means latest.
    rpc GetBlockNumber(Flag) returns (BlockNumber);

    rpc SendRawTransaction(blockchain.RawTransaction) returns (common.Hash);

    rpc SendRawTransactions(blockchain.RawTransactions) returns (common.Hashes);

    rpc GetBlockByHash(common.Hash) returns (blockchain.CompactBlock);

    rpc GetBlockByNumber(BlockNumber) returns (blockchain.CompactBlock);

    rpc GetTransaction(common.Hash) returns (blockchain.RawTransaction);

    rpc GetSystemConfig(common.Empty) returns (SystemConfig);

    rpc GetVersion(common.Empty) returns (SoftwareVersion);

    rpc GetBlockHash(BlockNumber) returns (common.Hash);

    rpc GetTransactionBlockNumber(common.Hash) returns (BlockNumber);

    rpc GetTransactionIndex(common.Hash) returns (TransactionIndex);

    rpc GetPeerCount(common.Empty) returns (PeerCount);

    // add new node
    rpc AddNode(common.NodeNetInfo) returns (common.StatusCode);

    // get peers info
    rpc GetPeersInfo(common.Empty) returns (common.TotalNodeInfo);
}
```

## executor的接口

其中call的数据格式由具体的executor微服务定义。
```protobuf
service ExecutorService {
    // exec a block return executed_block_hash
    rpc Exec(blockchain.Block) returns (common.HashResponse);

    rpc Call(CallRequest) returns (CallResponse);
}
```

## executor_evm的接口

```protobuf
service RPCService {
  rpc GetTransactionReceipt(common.Hash) returns (Receipt);

  rpc GetCode(common.Address) returns (ByteCode);

  rpc GetBalance(common.Address) returns (Balance);

  rpc GetTransactionCount(common.Address) returns (Nonce);

  rpc GetAbi(common.Address) returns (ByteAbi);
}
```

## proto文件参考

controller.proto文件

```protobuf
syntax = "proto3";

import "common.proto";
import "blockchain.proto";

package controller;

message Flag {
    bool flag = 1;
}

message BlockNumber {
    uint64 block_number = 1;
}

message SystemConfig {
    uint32 version = 1;
    bytes chain_id = 2;
    bytes admin = 3;
    uint32 block_interval = 4;
    repeated bytes validators = 5;
    bool emergency_brake = 6;
    bytes version_pre_hash = 7;
    bytes chain_id_pre_hash = 8;
    bytes admin_pre_hash = 9;
    bytes block_interval_pre_hash = 10;
    bytes validators_pre_hash = 11;
    bytes emergency_brake_pre_hash = 12;
}

message SoftwareVersion {
    string version = 1;
}

message PeerCount {
    uint64 peer_count = 1;
}

message TransactionIndex {
    uint64 tx_index = 1;
}

service RPCService {
    // flag means latest or pending.
    // true means pending, false means latest.
    rpc GetBlockNumber(Flag) returns (BlockNumber);

    rpc SendRawTransaction(blockchain.RawTransaction) returns (common.Hash);

    rpc SendRawTransactions(blockchain.RawTransactions) returns (common.Hashes);

    rpc GetBlockByHash(common.Hash) returns (blockchain.CompactBlock);

    rpc GetBlockByNumber(BlockNumber) returns (blockchain.CompactBlock);

    rpc GetTransaction(common.Hash) returns (blockchain.RawTransaction);

    rpc GetSystemConfig(common.Empty) returns (SystemConfig);

    rpc GetVersion(common.Empty) returns (SoftwareVersion);

    rpc GetBlockHash(BlockNumber) returns (common.Hash);

    rpc GetTransactionBlockNumber(common.Hash) returns (BlockNumber);

    rpc GetTransactionIndex(common.Hash) returns (TransactionIndex);

    rpc GetPeerCount(common.Empty) returns (PeerCount);

    // add new node
    rpc AddNode(common.NodeNetInfo) returns (common.StatusCode);

    // get peers info
    rpc GetPeersInfo(common.Empty) returns (common.TotalNodeInfo);
}
```

common.proto文件

```proto
syntax = "proto3";

import "blockchain.proto";

package common;

message Empty {}

message Hash {
    bytes hash = 1;
}

message Hashes {
    repeated Hash hashes = 1;
}

message Address {
    bytes address = 1;
}

message Proposal {
    uint64 height = 1;
    bytes data = 2;
}

message ProposalWithProof {
    Proposal proposal = 1;
    bytes proof = 2;
}

message BFTProposal {
    bytes pre_state_root = 1;
    bytes pre_proof = 2;
    blockchain.Block proposal = 3;
}

message ProposalEnum {
    oneof proposal {
        BFTProposal bft_proposal = 1;
    }
}

message ConsensusConfiguration {
    uint64 height = 1;
    uint32 block_interval = 2;
    repeated bytes validators = 3;
}

message StatusCode {
    uint32 code = 1;
}

message HashResponse {
    StatusCode status = 1;
    Hash hash = 2;
}

message ProposalResponse {
    StatusCode status = 1;
    Proposal proposal = 2;
}

message ConsensusConfigurationResponse {
    StatusCode status = 1;
    ConsensusConfiguration config = 2;
}

message NodeNetInfo {
    string multi_address = 1;
    uint64 origin = 2;
}

message TotalNodeNetInfo {
    repeated NodeNetInfo nodes = 1;
}

message NodeInfo {
    bytes address = 1;
    NodeNetInfo net_info = 2;
}

message TotalNodeInfo {
    repeated NodeInfo nodes = 1;
}
```

blockchain.proto文件

```proto
syntax = "proto3";

package blockchain;

message BlockHeader {
    // hash of previous BlockHeader
    bytes prevhash = 1;
    uint64 timestamp = 2;
    uint64 height = 3;
    bytes transactions_root = 4;
    bytes proposer = 5;
}

message Transaction {
    uint32 version = 1;
    // 1. length is 20 bytes for evm.
    // 2. if executor is multi-vm, it will be a path.
    bytes to = 2;
    // length is less than 128
    string nonce = 3;
    uint64 quota = 4;
    uint64 valid_until_block = 5;
    bytes data = 6;
    // length is 32 bytes.
    bytes value = 7;
    // length is 32 bytes.
    bytes chain_id = 8;
}

message Witness {
    bytes signature = 1;
    // add to support multi-address, or we don't know which address algorithm to use
    bytes sender = 2;
}

message UnverifiedTransaction {
    Transaction transaction = 1;
    // add to support multi-hash, or we don't know which hash algorithm to use
    bytes transaction_hash = 2;
    Witness witness = 3;
}

message UtxoTransaction {
    uint32 version = 1;
    bytes pre_tx_hash = 2;
    bytes output = 3;
    uint64 lock_id = 4;
}

message UnverifiedUtxoTransaction {
    UtxoTransaction transaction = 1;
    // add to support multi-hash, or we don't know which hash algorithm to use
    bytes transaction_hash = 2;
    repeated Witness witnesses = 3;
}

message RawTransactions {
    repeated RawTransaction body = 1;
}

message RawTransaction {
    oneof tx {
        UnverifiedTransaction normal_tx = 1;
        UnverifiedUtxoTransaction utxo_tx = 2;
    }
}

message CompactBlockBody {
    // transaction hash of UnverifiedTransaction or UnverifyedUtxoTransaction.
    repeated bytes tx_hashes = 1;
}

message CompactBlock {
    uint32 version = 1;
    BlockHeader header = 2;
    CompactBlockBody body = 3;
}

message Block {
    uint32 version = 1;
    BlockHeader header = 2;
    RawTransactions body = 3;
    bytes proof = 4;
}
```
