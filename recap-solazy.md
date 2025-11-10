# Program `fanout`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/fanout_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| distribute_v0 | payer | fanout, payer, to_account, token_account, voucher | fanout(has_one); receipt_account(constraint,spl); to_account(spl); voucher(has_one) | voucher | — |
| initialize_fanout_v0 | payer | collection, collection_account, fanout, master_edition, metadata, payer, token_account | collection(spl); collection_account(spl); token_account(spl) | collection, fanout, master_edition, metadata | fanout(space) |
| stake_v0 | payer, staker | collection_metadata, fanout, from_account, master_edition, metadata, mint, payer, receipt_account, stake_account, voucher | fanout(has_one); from_account(spl); mint(constraint,spl); receipt_account(spl); stake_account(spl) | collection_master_edition, collection_metadata, master_edition, metadata, voucher | voucher(space) |
| unstake_v0 | payer, voucher_authority | fanout, mint, payer, receipt_account, sol_destination, stake_account, to_account, voucher | fanout(has_one); receipt_account(constraint,spl); to_account(spl); voucher(has_one) | voucher | — |

# Program `rewards_oracle`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/rewards-oracle_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| set_current_rewards_wrapper_v0 | oracle | oracle, recipient | key_to_asset(constraint); recipient(has_one) | oracle_signer | — |
| set_current_rewards_wrapper_v1 | oracle | oracle, recipient | key_to_asset(constraint); recipient(has_one) | oracle_signer | — |
| set_current_rewards_wrapper_v2 | payer | payer, recipient | key_to_asset(constraint); recipient(has_one); sysvar_instructions(address) | oracle_signer | — |

# Program `lazy_transactions`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/lazy-transactions_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| close_canopy_v0 | authority | canopy, lazy_transactions, refund | lazy_transactions(has_one) | — | — |
| close_marker_v0 | authority | block, executed_transactions, lazy_transactions, refund | lazy_transactions(has_one) | block | — |
| execute_transaction_v0 | payer | executed_transactions, lazy_signer, lazy_transactions, payer | block(constraint); lazy_transactions(has_one,constraint) | block, lazy_signer | — |
| initialize_lazy_transactions_v0 | payer | canopy, executed_transactions, lazy_transactions, payer | canopy(owner,constraint); executed_transactions(owner,constraint) | lazy_transactions | lazy_transactions(space) |
| set_canopy_v0 | authority | canopy, lazy_transactions | lazy_transactions(has_one) | — | — |
| update_lazy_transactions_v0 | authority | canopy, executed_transactions, lazy_transactions | canopy(owner,constraint); executed_transactions(owner,constraint); lazy_transactions(has_one) | — | — |

# Program `mobile_entity_manager`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/mobile-entity-manager_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| approve_carrier_v0 | authority | carrier | carrier(has_one); sub_dao(has_one) | — | — |
| initialize_carrier_v0 | payer | carrier, collection, escrow, master_edition, metadata, payer, source, token_account | collection(spl); dao(has_one); escrow(spl); source(spl); sub_dao(has_one); token_account(spl) | carrier, collection, master_edition, metadata | carrier(space) |
| initialize_incentive_program_v0 | issuing_authority, payer | collection_metadata, incentive_escrow_program, key_to_asset, merkle_tree, payer, tree_authority | carrier(has_one,constraint); sub_dao(has_one); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, incentive_escrow_program, key_to_asset, program_approval, tree_authority | incentive_escrow_program(space) |
| initialize_subscriber_v0 | issuing_authority, payer | collection_metadata, key_to_asset, merkle_tree, payer, tree_authority | carrier(has_one,constraint); sub_dao(has_one); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, key_to_asset, program_approval, tree_authority | — |
| issue_carrier_nft_v0 | issuing_authority, payer | collection_metadata, key_to_asset, merkle_tree, payer, tree_authority | carrier(has_one); sub_dao(has_one); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, key_to_asset, program_approval, tree_authority | — |
| issue_mapping_rewards_nft_v0 | issuing_authority, payer | collection_metadata, key_to_asset, merkle_tree, payer, tree_authority | carrier(has_one); sub_dao(has_one); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, key_to_asset, program_approval, tree_authority | — |
| revoke_carrier_v0 | authority | carrier | carrier(has_one); sub_dao(has_one) | — | — |
| swap_carrier_stake | payer, update_authority | new_escrow, new_stake_source, original_stake, original_stake_destination, payer | carrier(has_one); dao(has_one); new_escrow(spl); new_stake_source(spl); original_stake(spl); original_stake_destination(spl); sub_dao(has_one) | — | — |
| update_carrier_tree_v0 | payer | carrier, new_merkle_tree, new_tree_authority, payer, tree_config | — | new_tree_authority, tree_config | — |
| update_carrier_v0 | update_authority | carrier, update_authority | carrier(has_one) | — | — |
| update_incentive_program_v0 | issuing_authority | incentive_escrow_program | carrier(has_one); incentive_escrow_program(has_one) | — | — |

# Program `data_credits`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/data-credits_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| burn_delegated_data_credits_v0 | dc_burn_authority | account_payer, dc_mint, escrow_account, sub_dao, sub_dao_epoch_info | dao(has_one); data_credits(has_one); delegated_data_credits(has_one); sub_dao(has_one) | account_payer, data_credits | — |
| burn_without_tracking_v0 | owner | burner, dc_mint, owner | — | — | — |
| change_delegated_sub_dao_v0 | authority, payer | destination_delegated_data_credits, destination_escrow_account, escrow_account, payer | dao(has_one); destination_escrow_account(spl); destination_sub_dao(has_one); escrow_account(spl); sub_dao(has_one) | data_credits, delegated_data_credits, destination_delegated_data_credits, destination_escrow_account, escrow_account | destination_delegated_data_credits(space) |
| delegate_data_credits_v0 | owner, payer | delegated_data_credits, escrow_account, from_account, payer | dao(has_one); escrow_account(spl); from_account(spl); sub_dao(has_one) | data_credits, delegated_data_credits, escrow_account | delegated_data_credits(space) |
| genesis_issue_delegated_data_credits_v0 | lazy_signer | circuit_breaker, dc_mint, delegated_data_credits, escrow_account, lazy_signer | dao(has_one); data_credits(has_one); escrow_account(spl); sub_dao(has_one) | circuit_breaker, data_credits, delegated_data_credits, escrow_account, lazy_signer | delegated_data_credits(space) |
| initialize_data_credits_v0 | freeze_authority, mint_authority, payer | account_payer, circuit_breaker, data_credits, dc_mint, payer | — | account_payer, circuit_breaker, data_credits | data_credits(space) |
| issue_data_credits_v0 | from | dc_mint, from, from_account, to_account | data_credits(has_one); from_account(constraint,spl); to_account(spl) | data_credits | — |
| mint_data_credits_v0 | owner | burner, circuit_breaker, dc_mint, hnt_mint, owner, recipient_token_account | burner(has_one,spl); data_credits(has_one); hnt_price_oracle(constraint); recipient_token_account(spl) | circuit_breaker, data_credits | — |
| update_data_credits_v0 | authority | data_credits | data_credits(has_one) | data_credits | — |

# Program `circuit_breaker`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/circuit-breaker_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| burn_v0 | owner | circuit_breaker, from, mint | circuit_breaker(has_one) | circuit_breaker | — |
| initialize_account_windowed_breaker_v0 | owner, payer | circuit_breaker, payer, token_account | token_account(has_one) | circuit_breaker | circuit_breaker(space) |
| initialize_mint_windowed_breaker_v0 | mint_authority, payer | circuit_breaker, mint, payer | — | circuit_breaker | circuit_breaker(space) |
| mint_v0 | mint_authority | circuit_breaker, mint, to | circuit_breaker(has_one) | circuit_breaker | — |
| remove_mint_authority_v0 | authority | circuit_breaker, mint, rent_refund | circuit_breaker(has_one) | — | — |
| transfer_v0 | owner | circuit_breaker, from, to | circuit_breaker(has_one); to(constraint) | circuit_breaker | — |
| update_account_windowed_breaker_v0 | authority | circuit_breaker | circuit_breaker(has_one) | — | — |
| update_mint_windowed_breaker_v0 | authority | circuit_breaker | circuit_breaker(has_one) | — | — |

# Program `treasury_management`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/treasury-management_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| initialize_treasury_management_v0 | mint_authority, payer | circuit_breaker, payer, treasury, treasury_management | supply_mint(spl); treasury(spl) | circuit_breaker, treasury_management | treasury_management(space) |
| redeem_v0 | owner | circuit_breaker, from, supply_mint, to, treasury | from(has_one,spl); to(spl); treasury_management(has_one) | circuit_breaker, treasury_management | — |
| update_treasury_management_v0 | authority | treasury_management | treasury_management(has_one) | — | — |

# Program `helium_entity_manager`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/helium-entity-manager_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| approve_maker_v0 | authority, payer | escrow, maker_approval, payer | dao(has_one); escrow(constraint,spl); maker(has_one); rewardable_entity_config(has_one); sub_dao(has_one) | maker_approval | maker_approval(space) |
| approve_program_v0 | authority, payer | payer, program_approval | dao(has_one) | program_approval | program_approval(space) |
| initialize_data_only_v0 | authority | authority, collection, data_only_config, master_edition, merkle_tree, metadata, token_account, tree_authority | collection(spl); dao(has_one); token_account(spl) | collection, data_only_config, master_edition, metadata, tree_authority | data_only_config(space) |
| initialize_maker_v0 | payer | collection, maker, master_edition, metadata, payer, token_account | collection(spl); token_account(spl) | collection, maker, master_edition, metadata | maker(space) |
| initialize_rewardable_entity_config_v0 | authority, payer | payer, rewardable_entity_config | sub_dao(has_one) | rewardable_entity_config | rewardable_entity_config(space) |
| issue_data_only_entity_v0 | ecc_verifier, payer | collection_metadata, data_only_config, data_only_escrow, key_to_asset, merkle_tree, payer, tree_authority | data_only_config(has_one); ecc_verifier(address); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, data_only_config, data_only_escrow, entity_creator, key_to_asset, tree_authority | key_to_asset(space) |
| issue_entity_v0 | ecc_verifier, issuing_authority, payer | collection_metadata, key_to_asset, maker, merkle_tree, payer, tree_authority | ecc_verifier(address); maker(has_one); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, key_to_asset, tree_authority | key_to_asset(space) |
| issue_iot_operations_fund_v0 | authority, payer | key_to_asset, master_edition, metadata, mint, payer, recipient_account | dao(has_one); mint(constraint,spl); recipient_account(spl) | entity_creator, key_to_asset, master_edition, metadata | key_to_asset(space) |
| issue_not_emitted_entity_v0 | authority, payer | key_to_asset, master_edition, metadata, mint, payer, recipient, recipient_account | mint(constraint,spl); recipient_account(spl) | entity_creator, key_to_asset, master_edition, metadata, recipient | key_to_asset(space) |
| issue_program_entity_v0 | collection_authority, payer, program_approver | collection_metadata, key_to_asset, merkle_tree, payer, tree_authority | program_approval(has_one); program_approver(constraint); token_metadata_program(address) | bubblegum_signer, collection_master_edition, collection_metadata, entity_creator, key_to_asset, tree_authority | key_to_asset(space) |
| onboard_data_only_iot_hotspot_v0 | dc_fee_payer, hotspot_owner, payer | dc_burner, dc_fee_payer, dc_mint, hotspot_owner, iot_info, payer, sub_dao | dao(has_one); data_only_config(has_one); dc(has_one); dc_burner(spl); key_to_asset(has_one,constraint); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | data_only_config, dc, iot_info | iot_info(space) |
| onboard_data_only_mobile_hotspot_v0 | dc_fee_payer, hotspot_owner, payer | dc_burner, dc_fee_payer, dc_mint, dnt_burner, dnt_mint, hotspot_owner, mobile_info, payer, sub_dao | dao(has_one); data_only_config(has_one); dc(has_one); dnt_price(address,constraint); key_to_asset(has_one,constraint); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | data_only_config, dc, mobile_info | mobile_info(space) |
| onboard_iot_hotspot_v0 | dc_fee_payer, hotspot_owner, issuing_authority, payer | dc_burner, dc_fee_payer, dc_mint, hotspot_owner, iot_info, payer, sub_dao | dao(has_one); dc(has_one); key_to_asset(has_one,constraint); maker(has_one); maker_approval(has_one); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | dc, iot_info, maker_approval | iot_info(space) |
| onboard_mobile_hotspot_v0 | dc_fee_payer, hotspot_owner, issuing_authority, payer | dc_burner, dc_fee_payer, dc_mint, dnt_burner, dnt_mint, hotspot_owner, mobile_info, payer, sub_dao | dao(has_one); dc(has_one); dnt_price(address,constraint); key_to_asset(has_one,constraint); maker(has_one); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | dc, maker_approval, mobile_info | mobile_info(space) |
| revoke_maker_v0 | authority, refund | maker_approval, refund | rewardable_entity_config(has_one) | maker_approval | — |
| revoke_program_v0 | authority, refund | program_approval, refund | dao(has_one) | program_approval | — |
| set_entity_active_v0 | active_device_authority | info, sub_dao | rewardable_entity_config(has_one); sub_dao(has_one) | — | — |
| set_maker_tree_v0 | payer, update_authority | maker, merkle_tree, payer, tree_authority | maker(has_one) | tree_authority | — |
| swap_maker_stake | payer, update_authority | new_escrow, new_stake_source, original_stake, original_stake_destination, payer | dao(has_one); maker(has_one); new_escrow(spl); new_stake_source(spl); original_stake(spl); original_stake_destination(spl); rewardable_entity_config(has_one); sub_dao(has_one) | maker_approval | — |
| temp_backfill_mobile_info | payer | mobile_info, payer | payer(address) | — | — |
| temp_pay_mobile_onboarding_fee_v0 | dc_fee_payer | dc_burner, dc_fee_payer, dc_mint, mobile_info, sub_dao | dao(has_one); dc(has_one); dc_burner(spl); key_to_asset(has_one); mobile_info(constraint); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | dc, mobile_info | — |
| temp_standardize_entity | authority, payer | collection_metadata, maker, merkle_tree | authority(address); key_to_asset(constraint); token_metadata_program(address) | collection_metadata, tree_authority | — |
| update_data_only_tree_v0 | payer | data_only_config, data_only_escrow, new_merkle_tree, new_tree_authority, old_tree_authority, payer | — | data_only_escrow, new_tree_authority, old_tree_authority | — |
| update_iot_info_v0 | dc_fee_payer, hotspot_owner, payer | dc_burner, dc_fee_payer, dc_mint, hotspot_owner, iot_info, payer | dao(has_one); data_credits_program(address); dc(has_one); iot_info(constraint); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | dc, tree_authority | — |
| update_maker_tree_v0 | payer | maker, new_merkle_tree, new_tree_authority, payer, tree_authority | tree_authority(constraint) | new_tree_authority, tree_authority | — |
| update_maker_v0 | update_authority | maker | maker(has_one) | — | — |
| update_mobile_info_v0 | dc_fee_payer, hotspot_owner, payer | dc_burner, dc_mint, hotspot_owner, mobile_info | dao(has_one); data_credits_program(address); dc(has_one); mobile_info(constraint); rewardable_entity_config(has_one,constraint); sub_dao(has_one) | dc, tree_authority | — |
| update_rewardable_entity_config_v0 | authority, payer | payer, rewardable_entity_config | rewardable_entity_config(has_one) | — | — |

# Program `hpl_crons`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/hpl-crons_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| add_entity_to_cron_v0 | payer, user_authority | cron_job, cron_job_transaction, payer | cron_job(has_one) | authority, cron_job_transaction | — |
| add_wallet_to_entity_cron_v0 | payer, user_authority | cron_job, cron_job_transaction, payer | cron_job(has_one) | authority, cron_job_transaction | — |
| close_delegation_claim_bot_v0 | position_authority | delegation_claim_bot, rent_refund | delegated_position(has_one); delegation_claim_bot(has_one); position(has_one); position_token_account(constraint,spl) | delegation_claim_bot | — |
| close_entity_claim_cron_v0 | payer, user_authority | cron_job, cron_job_name_mapping, payer, rent_refund, task_return_account_1, task_return_account_2, user_cron_jobs | cron_job(has_one) | authority, task_return_account_1, task_return_account_2, user_cron_jobs | — |
| init_delegation_claim_bot_v0 | payer, position_authority | delegation_claim_bot, payer | delegated_position(has_one); position(has_one); position_token_account(constraint,spl) | delegation_claim_bot | delegation_claim_bot(space) |
| init_entity_claim_cron_v0 | payer, user_authority | cron_job, cron_job_name_mapping, payer, task, task_queue, task_return_account_1, task_return_account_2, user_cron_jobs | — | authority, queue_authority, task_queue_authority, task_return_account_1, task_return_account_2, user_cron_jobs | — |
| init_epoch_tracker | payer | epoch_tracker, payer | — | epoch_tracker | epoch_tracker(space) |
| queue_delegation_claim_v0 | payer, position_claim_payer | delegation_claim_bot, payer, position_claim_payer, rent_refund, task_queue, task_return_account | dao(has_one); delegated_position(has_one); delegation_claim_bot(has_one); delegator_ata(spl); position(has_one); position_token_account(constraint,spl); sub_dao(has_one) | payer, position_claim_payer, task_return_account | — |
| queue_end_epoch | payer | epoch_tracker, payer, task_return_account | — | epoch_tracker, payer, task_return_account | — |
| queue_proxy_vote_v0 | payer | payer, pda_wallet, queue_authority, task, task_queue | marker(has_one) | pda_wallet, queue_authority, task_queue_authority | — |
| queue_relinquish_expired_proxy_vote_marker_v0 | payer | payer, queue_authority, task, task_queue | — | queue_authority, task_queue_authority | — |
| queue_relinquish_expired_vote_marker_v0 | payer, voter | payer, queue_authority, task, task_queue | marker(has_one); position(constraint) | queue_authority, task_queue_authority | — |
| queue_resolve_proposal_v0 | payer | payer, queue_authority, task, task_queue | namespace(constraint) | — | — |
| queue_wallet_claim_v0 | payer | payer, pda_wallet, queue_authority, task, task_queue | — | pda_wallet, queue_authority, task_queue_authority | — |
| remove_entity_from_cron_v0 | user_authority | cron_job, cron_job_transaction, rent_refund | cron_job(has_one) | authority | — |
| requeue_entity_claim_cron_v0 | payer, user_authority | cron_job, cron_job_name_mapping, payer, task, task_queue, user_cron_jobs | cron_job(has_one,constraint) | authority, queue_authority, task_queue_authority, task_return_account_1, task_return_account_2, user_cron_jobs | — |
| requeue_entity_claim_v0 | — | — | — | — | — |
| requeue_proxy_vote_v0 | — | — | — | — | — |
| requeue_wallet_claim_v0 | — | — | — | — | — |
| start_delegation_claim_bot_v0 | payer, position_authority | delegation_claim_bot, payer, task, task_queue | dao(has_one); delegation_claim_bot(has_one,constraint); delegator_ata(spl); position_token_account(constraint,spl); sub_dao(has_one) | queue_authority, task_queue_authority | — |
| update_epoch_tracker | authority | epoch_tracker | epoch_tracker(has_one) | — | — |

# Program `voter_stake_registry`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/voter-stake-registry_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| clear_recent_proposals_v0 | dao | position, registrar | position(has_one) | dao | — |
| close_position_v0 | position_authority | mint, position, position_token_account, registrar, sol_destination | position(has_one,constraint); position_token_account(constraint,spl) | position | — |
| configure_voting_mint_v0 | payer, realm_authority | payer, registrar | registrar(has_one) | — | — |
| count_proxy_vote_v0 | payer | marker, payer, position, proposal, registrar, state_controller | position(has_one); proposal(owner,has_one); proposal_config(owner,has_one); proposal_program(constraint); proxy_assignment(has_one,constraint); proxy_marker(has_one) | marker, proxy_marker | marker(space) |
| deposit_v0 | deposit_authority | deposit_token, position, vault | deposit_token(has_one,spl); position(has_one); vault(spl) | — | — |
| initialize_position_v0 | payer | collection_metadata, master_edition, metadata, mint, payer, position, position_token_account, vault | mint(constraint,spl); position_token_account(spl); registrar(has_one); vault(spl) | collection_master_edition, collection_metadata, master_edition, metadata, position | position(space) |
| initialize_registrar_v0 | payer, realm_authority | collection, master_edition, metadata, payer, registrar, token_account | collection(spl); token_account(spl) | collection, master_edition, metadata, registrar | registrar(space) |
| ledger_transfer_position_v0 | approver, from, payer, to | from_token_account, mint, payer, to_token_account | approver(address); from_token_account(spl); mint(constraint,spl); position(has_one); to_token_account(spl) | position | — |
| proxied_relinquish_vote_v0 | voter | marker, position, proposal, registrar, rent_refund, state_controller | marker(has_one); position(has_one); proposal(owner,has_one); proposal_config(owner,has_one); proposal_program(constraint); proxy_assignment(has_one,constraint) | marker | — |
| proxied_relinquish_vote_v1 | voter | marker, proposal | — | marker | — |
| proxied_vote_v0 | payer, voter | marker, payer, position, proposal, registrar, state_controller | position(has_one); proposal(owner,has_one); proposal_config(owner,has_one); proposal_program(constraint); proxy_assignment(has_one,constraint) | marker | marker(space) |
| proxied_vote_v1 | payer, voter | marker, payer | — | marker | marker(space) |
| relinquish_expired_proxy_vote_v0 | — | marker, rent_refund | marker(has_one) | — | — |
| relinquish_expired_vote_v0 | — | marker, position, rent_refund | marker(has_one) | marker | — |
| relinquish_vote_v1 | voter | marker, position, proposal, registrar, rent_refund, state_controller | marker(has_one); position(has_one); proposal(owner,has_one); proposal_config(owner,has_one); proposal_program(constraint); token_account(constraint,spl) | marker | — |
| reset_lockup_v0 | position_authority, position_update_authority | position | position(has_one); position_token_account(constraint,spl); position_update_authority(constraint) | position | — |
| set_time_offset_v0 | realm_authority | registrar | registrar(has_one) | — | — |
| temp_release_position_v0 | authority | position | authority(address); position(constraint) | — | — |
| transfer_v0 | position_authority, position_update_authority | source_position, source_vault, target_position, target_vault | position_token_account(constraint,spl); position_update_authority(constraint); source_position(has_one,constraint); source_vault(spl); target_position(has_one,constraint); target_vault(spl) | source_position | — |
| update_registrar_authority_v0 | realm_authority | registrar | registrar(has_one) | — | — |
| update_registrar_v0 | realm_authority | registrar | registrar(has_one) | — | — |
| vote_v0 | payer, voter | marker, payer, position, proposal, registrar, state_controller | position(has_one); proposal(owner,has_one); proposal_config(owner,has_one); proposal_program(constraint); token_account(constraint,spl) | marker | marker(space) |
| withdraw_v0 | position_authority | destination, position, position_authority, position_token_account, vault | destination(spl); position(has_one,constraint); position_token_account(constraint,spl); vault(spl) | position | — |

# Program `hexboosting`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/hexboosting_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| boost_v0 | hexboost_authority, payer | boosted_hex, dc_mint, payer, payment_account | boost_config(has_one); boosted_hex(constraint); carrier(has_one,constraint); data_credits(has_one); payment_account(spl) | boosted_hex, data_credits | boosted_hex(space) |
| close_boost_v0 | rent_reclaim_authority | boosted_hex | boost_config(has_one); boosted_hex(has_one,constraint) | — | — |
| initialize_boost_config_v0 | authority, payer | boost_config, payer | dao(has_one); sub_dao(has_one) | boost_config | boost_config(space) |
| start_boost_v0 | start_authority | boosted_hex | boost_config(has_one); boosted_hex(has_one) | — | — |
| start_boost_v1 | start_authority | boosted_hex | boost_config(has_one); boosted_hex(has_one) | — | — |
| update_boost_config_v0 | authority | boost_config | boost_config(has_one); sub_dao(has_one) | — | — |

# Program `no_emit`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/no-emit_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| no_emit_v0 | payer | mint, not_emitted_counter, payer, token_account | mint(constraint); token_account(spl) | no_emit_wallet, not_emitted_counter | not_emitted_counter(space) |

# Program `helium_sub_daos`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/helium-sub-daos_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| add_recent_proposal_to_dao_v0 | — | dao | — | — | — |
| admin_set_dc_onboarding_fees_paid | authority | sub_dao | dao(has_one); sub_dao(has_one) | — | — |
| admin_set_dc_onboarding_fees_paid_epoch_info | authority | sub_dao_epoch_info | dao(has_one); sub_dao(has_one); sub_dao_epoch_info(has_one) | — | — |
| calculate_utility_score_v0 | payer | dao_epoch_info, payer, prev_sub_dao_epoch_info, sub_dao, sub_dao_epoch_info | dao(has_one); sub_dao(has_one) | prev_dao_epoch_info | — |
| change_delegation_v0 | payer, position_authority | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, old_closing_time_sub_dao_epoch_info, old_genesis_end_sub_dao_epoch_info, old_sub_dao, old_sub_dao_epoch_info, payer, position_authority, sub_dao, sub_dao_epoch_info | dao(has_one); old_sub_dao(has_one); position(has_one,constraint); position_token_account(constraint,spl); registrar(has_one); sub_dao(has_one,constraint); sub_dao_epoch_info(constraint) | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, old_closing_time_sub_dao_epoch_info, old_genesis_end_sub_dao_epoch_info, old_sub_dao_epoch_info, position, sub_dao_epoch_info | closing_time_sub_dao_epoch_info(space); old_sub_dao_epoch_info(space); sub_dao_epoch_info(space) |
| claim_rewards_v0 | position_authority | delegated_position, delegator_ata, delegator_pool, delegator_pool_circuit_breaker, position_authority, sub_dao | dao(has_one); delegated_position(has_one); delegator_ata(spl); position(has_one); position_token_account(constraint,spl); sub_dao(has_one); sub_dao_epoch_info(constraint) | delegated_position, delegator_pool_circuit_breaker, position, sub_dao_epoch_info | — |
| claim_rewards_v1 | payer | dao, delegated_position, delegator_ata, delegator_pool, delegator_pool_circuit_breaker, hnt_mint, mint, payer, position, registrar, sub_dao | dao(has_one); dao_epoch_info(constraint); delegated_position(has_one); delegator_ata(spl); position(has_one); position_authority(constraint); position_token_account(constraint,spl); sub_dao(has_one) | dao_epoch_info, delegated_position, delegator_pool_circuit_breaker, position | — |
| close_delegation_v0 | payer, position_authority | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, payer, position_authority, sub_dao, sub_dao_epoch_info | dao(has_one); delegated_position(has_one); position(has_one); position_token_account(constraint,spl); sub_dao(has_one) | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, position, sub_dao_epoch_info | sub_dao_epoch_info(space) |
| delegate_v0 | payer, position_authority | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, payer, position_authority, sub_dao, sub_dao_epoch_info | dao(has_one); position(has_one,constraint); position_token_account(constraint,spl); registrar(has_one); sub_dao(has_one); sub_dao_epoch_info(constraint) | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, position, sub_dao_epoch_info | closing_time_sub_dao_epoch_info(space); delegated_position(space); sub_dao_epoch_info(space) |
| extend_expiration_ts_v0 | authority, payer | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, old_closing_time_sub_dao_epoch_info, payer, position, sub_dao | authority(constraint); dao(has_one); delegated_position(has_one); position(has_one); position_token_account(constraint,spl); registrar(has_one); sub_dao(has_one) | closing_time_sub_dao_epoch_info, delegated_position, genesis_end_sub_dao_epoch_info, old_closing_time_sub_dao_epoch_info | closing_time_sub_dao_epoch_info(space) |
| initialize_dao_v0 | hnt_freeze_authority, hnt_mint_authority, payer | dao, delegator_pool, delegator_pool_circuit_breaker, hnt_circuit_breaker, hnt_mint, payer | delegator_pool(spl); hst_pool(spl); rewards_escrow(spl) | dao, delegator_pool_circuit_breaker, hnt_circuit_breaker | dao(space) |
| initialize_hnt_delegator_pool | authority, payer | dao, delegator_pool, delegator_pool_circuit_breaker, payer | dao(has_one); delegator_pool(spl) | delegator_pool_circuit_breaker | — |
| initialize_sub_dao_v0 | authority, dnt_mint_authority, payer, sub_dao_freeze_authority | dao, dnt_mint, payer, sub_dao, treasury, treasury_circuit_breaker, treasury_management | dao(has_one) | sub_dao, treasury_circuit_breaker, treasury_management | sub_dao(space) |
| issue_rewards_v0 | — | dao_epoch_info, delegator_pool, dnt_mint, hnt_circuit_breaker, hnt_mint, rewards_escrow, sub_dao, sub_dao_epoch_info, treasury | dao(has_one); dao_epoch_info(has_one,constraint); sub_dao(has_one); sub_dao_epoch_info(has_one,constraint) | dao_epoch_info, hnt_circuit_breaker, prev_sub_dao_epoch_info, sub_dao_epoch_info | — |
| reset_lockup_v0 | position_authority | position | dao(has_one); delegated_position(constraint); position(has_one); position_token_account(constraint,spl) | delegated_position, position | — |
| switch_mobile_ops_fund | authority, payer | hnt_circuit_breaker, hnt_mint, mobile_mint, ops_fund_hnt, ops_fund_mobile, payer | dao(has_one); mobile_mint(address); ops_fund_hnt(spl); ops_fund_mobile(spl) | hnt_circuit_breaker | — |
| temp_backfill_dao_recent_proposals | authority | dao_epoch_info | authority(address); dao_epoch_info(has_one) | — | — |
| temp_update_sub_dao_epoch_info | authority | authority, sub_dao_epoch_info | authority(address) | sub_dao_epoch_info | sub_dao_epoch_info(space) |
| track_dc_burn_v0 | account_payer | account_payer, sub_dao, sub_dao_epoch_info | dao(has_one); sub_dao(has_one) | account_payer, sub_dao_epoch_info | sub_dao_epoch_info(space) |
| track_dc_onboarding_fees_v0 | hem_auth | sub_dao | — | hem_auth | — |
| track_vote_v0 | payer | dao, dao_epoch_info, delegated_position, payer, position | delegated_position(has_one); position(has_one,constraint); proposal(constraint); sub_dao(has_one) | delegated_position, marker | — |
| transfer_v0 | position_authority | source_position, source_vault, target_position, target_vault | dao(has_one); position_token_account(constraint,spl); source_delegated_position(constraint); source_position(has_one); source_vault(spl); target_delegated_position(constraint); target_position(has_one); target_vault(spl) | dao, source_delegated_position, source_position, target_delegated_position | — |
| update_dao_v0 | authority, payer | dao, payer | dao(has_one) | dao | — |
| update_sub_dao_v0 | authority, payer | payer, sub_dao | sub_dao(has_one) | sub_dao | — |
| update_sub_dao_vehnt_v0 | authority | sub_dao | sub_dao(has_one) | sub_dao | — |

# Program `price_oracle`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/price-oracle_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| initialize_price_oracle_v0 | payer, price_oracle | payer, price_oracle | — | — | price_oracle(space) |
| submit_price_v0 | oracle | price_oracle | — | — | — |
| update_price_oracle_v0 | authority | authority, price_oracle | price_oracle(has_one) | — | — |
| update_price_v0 | — | price_oracle | — | — | — |

# Program `lazy_distributor`
_Crate: /home/ectario/Documents/solana/bb/helium-program-library/programs/lazy-distributor_

| Instruction | Signers | Writable | Constrained | Seeded | Memory |
|---|---|---|---|---|---|
| distribute_compression_rewards_v0 | payer | circuit_breaker, destination_account, owner, payer, recipient, rewards_escrow | — | — | — |
| distribute_custom_destination_v0 | payer | circuit_breaker, destination_account, owner, payer, recipient, rewards_escrow | — | — | — |
| distribute_rewards_v0 | payer | circuit_breaker, destination_account, owner, payer, recipient, rewards_escrow | recipient_mint_account(constraint,spl) | — | — |
| dummy_ix | — | dummy | — | — | — |
| initialize_compression_recipient_v0 | payer | payer, recipient | — | lazy_distributor, recipient | recipient(space) |
| initialize_lazy_distributor_v0 | payer | circuit_breaker, lazy_distributor, payer, rewards_escrow, rewards_mint | rewards_escrow(spl) | circuit_breaker, lazy_distributor | lazy_distributor(space) |
| initialize_recipient_v0 | payer | payer, recipient | mint(constraint); target_metadata(has_one) | lazy_distributor, recipient, target_metadata | recipient(space) |
| set_current_rewards_v0 | oracle, payer | payer, recipient | oracle(constraint); recipient(has_one) | — | — |
| set_current_rewards_v1 | payer | payer, recipient | lazy_distributor(constraint); recipient(has_one); sysvar_instructions(address) | — | — |
| temp_update_matching_destination | authority | recipient | authority(address); original_recipient(constraint); recipient(constraint) | — | — |
| update_compression_destination_v0 | owner | recipient | — | — | — |
| update_destination_v0 | owner | recipient | recipient_mint_account(constraint,spl) | — | — |
| update_lazy_distributor_v0 | authority | lazy_distributor | lazy_distributor(has_one) | lazy_distributor | — |

