// v95.0 SINGULARITY: RETH EXEX + REVM + PETGRAPH
use alloy_primitives::{Address, U256};
use reth_exex::{ExExContext, ExExNotification};
use revm::{EVM, primitives::ExecutionResult};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::Dfs;
use std::sync::Arc;

// --- 2026 ELITE CONSTANTS ---
const WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

/// The Singularity Bot: An Execution Extension that lives inside the node's memory.
pub async fn singularity_exex<Node>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
    info!("SINGULARITY ONLINE: NATIVE MEMORY FORKING ACTIVE");

    // Infinite Payload Engine: Market Graph in RAM
    let mut market_graph = UnGraph::<Address, PoolEdge>::new_undirected();
    
    // Process notifications directly from the Reth execution engine
    while let Some(notification) = ctx.notifications.recv().await {
        let start = std::time::Instant::now();

        match notification {
            ExExNotification::PendingTransaction { tx } => {
                // 1. NANO-SECOND MARKET ANALYSIS
                // Simulate victim trade in local REVM fork (sub-1µs)
                if let Some(profit_path) = find_infinite_arb(&market_graph, &tx).await {
                    
                    // 2. SATURATION STRIKE (Flashbots + Jito + Fiber)
                    execute_singularity_strike(&ctx, tx, profit_path).await?;
                    
                    let elapsed = start.elapsed().as_nanos();
                    info!("🚀 SINGULARITY STRIKE | Latency: {}ns", elapsed);
                }
            }
            ExExNotification::ChainCommitted { new } => {
                // Keep the Market DataGraph perfectly synced without RPC calls
                update_graph_from_block(&mut market_graph, &new);
            }
            _ => {}
        }
    }
    Ok(())
}
