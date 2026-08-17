//! Submit, approve, and cancel a governance proposal.
//!
//! Only a Super Representative account can run this example. Proposal parameters
//! alter chain behavior, so choose the parameter ID and value deliberately and use
//! a private network unless you understand the consequences.
//!
//! Required env:
//!   TRON_PRIVATE_KEY     — Super Representative private key
//!   TRON_PARAMETER_ID    — numeric chain-parameter ID
//!   TRON_PARAMETER_VALUE — proposed integer value
//!
//! Optional env:
//!   TRON_API_KEY         — TronGrid API key

use anyhow::Context as _;
use tronz::{LocalSigner, ProviderBuilder, TRONGRID_NILE, providers::ext::GovernanceApi as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let signer =
        LocalSigner::from_hex(&std::env::var("TRON_PRIVATE_KEY").context("TRON_PRIVATE_KEY")?)?;
    let proposer = signer.address();
    let parameter_id =
        std::env::var("TRON_PARAMETER_ID").context("TRON_PARAMETER_ID")?.parse::<i64>()?;
    let parameter_value =
        std::env::var("TRON_PARAMETER_VALUE").context("TRON_PARAMETER_VALUE")?.parse::<i64>()?;
    let api_key = std::env::var("TRON_API_KEY").ok();

    let provider = ProviderBuilder::new()
        .with_signer(signer)
        .maybe_api_key(api_key)
        .connect_grpc(TRONGRID_NILE)
        .await?;
    let existing_proposal_ids = provider
        .list_proposals()
        .await?
        .into_iter()
        .map(|proposal| proposal.proposal_id)
        .collect::<std::collections::HashSet<_>>();

    provider
        .submit_proposal()
        .parameter(parameter_id, parameter_value)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    let proposal = provider
        .list_proposals()
        .await?
        .into_iter()
        .filter(|proposal| {
            !existing_proposal_ids.contains(&proposal.proposal_id)
                && proposal.proposer_address == Some(proposer)
        })
        .max_by_key(|proposal| proposal.proposal_id)
        .context("created proposal was not found")?;
    println!("created proposal #{}", proposal.proposal_id);

    provider
        .approve_proposal()
        .proposal_id(proposal.proposal_id)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    provider
        .cancel_proposal()
        .proposal_id(proposal.proposal_id)
        .send()
        .await?
        .require_success()
        .get_receipt()
        .await?;

    let canceled = provider.get_proposal_by_id(proposal.proposal_id).await?;
    println!("proposal state: {:?}", canceled.state);

    Ok(())
}
