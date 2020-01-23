// Copyright 2019. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use futures::channel::mpsc::{Receiver, Sender};
use std::sync::Arc;
use tari_core::{
    base_node::{
        service::{BaseNodeServiceConfig, BaseNodeServiceInitializer},
        BaseNodeStateMachine,
        BaseNodeStateMachineConfig,
        LocalNodeCommsInterface,
        OutboundNodeCommsInterface,
    },
    blocks::Block,
    chain_storage::{
        create_lmdb_database,
        BlockchainBackend,
        BlockchainDatabase,
        LMDBDatabase,
        MemoryDatabase,
        Validators,
    },
    consensus::ConsensusManager,
    mempool::{Mempool, MempoolConfig, MempoolValidators},
    mining::Miner,
    proof_of_work::DiffAdjManager,
    transactions::{
        crypto::keys::SecretKey as SK,
        types::{CryptoFactories, HashDigest, PrivateKey, PublicKey},
    },
};
use tari_service_framework::handles::ServiceHandles;

pub fn build_miner<B: BlockchainBackend>(
    handles: Arc<ServiceHandles>,
    node: &BaseNodeStateMachine<B>,
    consensus_manager: ConsensusManager<B>,
) -> Miner<B>
{
    let stop_flag = node.get_interrupt_flag();
    let node_local_interface = handles.get_handle::<LocalNodeCommsInterface>().unwrap();
    let miner = Miner::new(stop_flag, consensus_manager, &node_local_interface);
    miner
}
