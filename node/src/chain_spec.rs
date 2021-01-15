use framenode_runtime::{
    bonding_curve_pool, eth_bridge, opaque::SessionKeys, AccountId, AssetSymbol, AssetsConfig,
    BabeConfig, BalancesConfig, BondingCurvePoolConfig, DEXAPIConfig, DEXManagerConfig,
    EthBridgeConfig, FarmingConfig, FaucetConfig, GenesisConfig, GetBaseAssetId, GrandpaConfig,
    LiquiditySourceType, MultisigConfig, PermissionsConfig, PswapDistributionConfig, PswapId,
    Runtime, SessionConfig, Signature, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    TechAccountId, TechnicalConfig, TokensConfig, UsdId, ValId, XorId, WASM_BINARY,
};

use common::{
    balance::Balance, fixed, hash, prelude::DEXInfo, DEXId, Fixed, TechPurpose, PSWAP, VAL, XOR,
};
use frame_support::sp_runtime::Percent;
use framenode_runtime::bonding_curve_pool::{DistributionAccountData, DistributionAccounts};
use framenode_runtime::eth_bridge::AssetKind;
use grandpa::AuthorityId as GrandpaId;
use hex_literal::hex;
use permissions::Scope;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
    sp_std::iter::once,
    traits::{IdentifyAccount, Verify},
    Perbill,
};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
type Technical = technical::Module<Runtime>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, BabeId, GrandpaId) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<AuraId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<GrandpaId>(seed),
    )
}

fn session_keys(grandpa: GrandpaId, babe: BabeId) -> SessionKeys {
    SessionKeys { babe, grandpa }
}

pub fn staging_test_net() -> ChainSpec {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    ChainSpec::from_genesis(
        "SORA-staging Testnet",
        "sora-substrate-staging",
        ChainType::Live,
        move || {
            testnet_genesis(
                hex!("92c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                    authority_keys_from_seed("Charlie"),
                    authority_keys_from_seed("Dave"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    hex!("92c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                ],
                vec![
                    hex!("92c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                    hex!("93c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                    hex!("94c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                    hex!("95c4ff71ae7492a1e6fef5d80546ea16307c560ac1063ffaa5e0e084df1e2b7e").into(),
                ],
                hex!("da723e9d76bd60da0ec846895c5e0ecf795b50ae652c012f27e56293277ef372").into(),
                hex!("16fec57d383a1875ab4e9786aea7a626e721a491c828f475ae63ef098f98f373").into(),
                hex!("da723e9d76bd60da0ec846895c5e0ecf795b50ae652c012f27e56293277ef372").into(),
            )
        },
        vec![],
        None,
        Some("sora-substrate-1"),
        Some(properties),
        None,
    )
}

fn bonding_curve_distribution_accounts(
) -> DistributionAccounts<DistributionAccountData<<Runtime as technical::Trait>::TechAccountId>> {
    let val_holders_coefficient: Fixed = fixed!(50%);
    let val_holders_xor_alloc_coeff = val_holders_coefficient * fixed!(90%);
    let val_holders_buy_back_coefficient = val_holders_coefficient * (fixed!(100%) - fixed!(90%));
    let projects_coefficient = fixed!(100%) - val_holders_coefficient;
    let projects_sora_citizens_coeff = projects_coefficient * fixed!(1%);
    let projects_stores_and_shops_coeff = projects_coefficient * fixed!(4%);
    let projects_parliament_and_development_coeff = projects_coefficient * fixed!(5%);
    let projects_other_coeff = projects_coefficient * fixed!(90%);

    debug_assert_eq!(
        fixed!(100%),
        val_holders_xor_alloc_coeff
            + projects_sora_citizens_coeff
            + projects_stores_and_shops_coeff
            + projects_parliament_and_development_coeff
            + projects_other_coeff
            + val_holders_buy_back_coefficient
    );

    let xor_allocation = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"xor_allocation".to_vec()),
        ),
        val_holders_xor_alloc_coeff,
    );
    let sora_citizens = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"sora_citizens".to_vec()),
        ),
        projects_sora_citizens_coeff,
    );
    let stores_and_shops = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"stores_and_shops".to_vec()),
        ),
        projects_stores_and_shops_coeff,
    );
    let parliament_and_development = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"parliament_and_development".to_vec()),
        ),
        projects_parliament_and_development_coeff,
    );
    let projects = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"projects".to_vec()),
        ),
        projects_other_coeff,
    );
    let val_holders = DistributionAccountData::new(
        TechAccountId::Pure(
            DEXId::Polkaswap.into(),
            TechPurpose::Identifier(b"val_holders".to_vec()),
        ),
        val_holders_buy_back_coefficient,
    );
    DistributionAccounts::<_> {
        xor_allocation,
        sora_citizens,
        stores_and_shops,
        parliament_and_development,
        projects,
        val_holders,
    }
}

pub fn local_testnet_config() -> ChainSpec {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    ChainSpec::from_genesis(
        "SORA-local Testnet",
        "sora-substrate-local",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                    authority_keys_from_seed("Charlie"),
                    authority_keys_from_seed("Dave"),
                    /*
                    authority_keys_from_seed("Eve"),
                    authority_keys_from_seed("Ferdie"),
                    */
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        vec![],
        None,
        None,
        Some(properties),
        None,
    )
}

fn testnet_genesis(
    root_key: AccountId,
    initial_authorities: Vec<(AccountId, AccountId, AuraId, BabeId, GrandpaId)>,
    endowed_accounts: Vec<AccountId>,
    initial_bridge_peers: Vec<AccountId>,
    dex_root: AccountId,
    tech_permissions_owner: AccountId,
    initial_assets_owner: AccountId,
) -> GenesisConfig {
    let initial_balance = 1u128 << 60;
    let initial_staking: Balance = (initial_balance / 2).into();
    let xor_fee_tech_account_id = TechAccountId::Generic(
        xor_fee::TECH_ACCOUNT_PREFIX.to_vec(),
        xor_fee::TECH_ACCOUNT_MAIN.to_vec(),
    );
    let xor_fee_account_id: AccountId =
        technical::Module::<Runtime>::tech_account_id_to_account_id(&xor_fee_tech_account_id)
            .expect("Failed to decode account Id");
    let faucet_tech_account_id = TechAccountId::Generic(
        faucet::TECH_ACCOUNT_PREFIX.to_vec(),
        faucet::TECH_ACCOUNT_MAIN.to_vec(),
    );
    let faucet_account_id: AccountId =
        technical::Module::<Runtime>::tech_account_id_to_account_id(&faucet_tech_account_id)
            .expect("Failed to decode account id");
    let initial_eth_bridge_xor_amount = 350_000_u32;
    let initial_eth_bridge_val_amount = 33_900_000_u32;
    let eth_bridge_tech_account_id = TechAccountId::Generic(
        eth_bridge::TECH_ACCOUNT_PREFIX.to_vec(),
        eth_bridge::TECH_ACCOUNT_MAIN.to_vec(),
    );
    let eth_bridge_account_id =
        technical::Module::<Runtime>::tech_account_id_to_account_id(&eth_bridge_tech_account_id)
            .unwrap();
    let eth_bridge_authority_tech_account_id = TechAccountId::Generic(
        eth_bridge::TECH_ACCOUNT_PREFIX.to_vec(),
        eth_bridge::TECH_ACCOUNT_AUTHORITY.to_vec(),
    );
    let eth_bridge_authority_account_id =
        technical::Module::<Runtime>::tech_account_id_to_account_id(
            &eth_bridge_authority_tech_account_id,
        )
        .unwrap();

    let bonding_curve_reserves_tech_account_id = TechAccountId::Generic(
        bonding_curve_pool::TECH_ACCOUNT_PREFIX.to_vec(),
        bonding_curve_pool::TECH_ACCOUNT_RESERVES.to_vec(),
    );

    let pswap_distribution_tech_account_id =
        framenode_runtime::GetPswapDistributionTechAccountId::get();
    let pswap_distribution_account_id = framenode_runtime::GetPswapDistributionAccountId::get();

    let mut tech_accounts = vec![
        (xor_fee_account_id.clone(), xor_fee_tech_account_id),
        (faucet_account_id.clone(), faucet_tech_account_id.clone()),
        (
            eth_bridge_account_id.clone(),
            eth_bridge_tech_account_id.clone(),
        ),
        (
            eth_bridge_authority_account_id.clone(),
            eth_bridge_authority_tech_account_id.clone(),
        ),
        (
            pswap_distribution_account_id.clone(),
            pswap_distribution_tech_account_id.clone(),
        ),
    ];
    let accounts = bonding_curve_distribution_accounts();
    tech_accounts.push((
        Technical::tech_account_id_to_account_id(&accounts.val_holders.account_id).unwrap(),
        accounts.val_holders.account_id.clone(),
    ));
    for tech_account in &accounts.xor_distribution_accounts_as_array() {
        tech_accounts.push((
            Technical::tech_account_id_to_account_id(&tech_account).unwrap(),
            (*tech_account).to_owned(),
        ));
    }

    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: WASM_BINARY.unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_sudo: Some(SudoConfig { key: root_key }),
        technical: Some(TechnicalConfig {
            account_ids_to_tech_account_ids: tech_accounts,
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|(account, _, _, babe_id, grandpa_id)| {
                    (
                        account.clone(),
                        account.clone(),
                        session_keys(grandpa_id.clone(), babe_id.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|(stash_account, account, _, _, _)| {
                    (
                        stash_account.clone(),
                        account.clone(),
                        initial_staking,
                        StakerStatus::Validator,
                    )
                })
                .collect(),
            invulnerables: initial_authorities
                .iter()
                .map(|(stash_account, _, _, _, _)| stash_account.clone())
                .collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        assets: Some(AssetsConfig {
            endowed_assets: vec![
                (
                    XorId::get(),
                    initial_assets_owner.clone(),
                    AssetSymbol(b"XOR".to_vec()),
                    18,
                ),
                (
                    UsdId::get(),
                    initial_assets_owner.clone(),
                    AssetSymbol(b"USDT".to_vec()),
                    18,
                ),
                (
                    ValId::get(),
                    initial_assets_owner.clone(),
                    AssetSymbol(b"VAL".to_vec()),
                    18,
                ),
                (
                    PswapId::get(),
                    initial_assets_owner.clone(),
                    AssetSymbol(b"PSWAP".to_vec()),
                    18,
                ),
            ],
        }),
        permissions: Some(PermissionsConfig {
            initial_permission_owners: vec![
                (
                    permissions::TRANSFER,
                    Scope::Unlimited,
                    vec![tech_permissions_owner.clone()],
                ),
                (
                    permissions::INIT_DEX,
                    Scope::Unlimited,
                    vec![tech_permissions_owner.clone()],
                ),
                (
                    permissions::MANAGE_DEX,
                    Scope::Limited(hash(&0u32)),
                    vec![tech_permissions_owner.clone()],
                ),
                (
                    permissions::MINT,
                    Scope::Unlimited,
                    vec![tech_permissions_owner.clone()],
                ),
                (
                    permissions::BURN,
                    Scope::Unlimited,
                    vec![tech_permissions_owner.clone()],
                ),
            ],
            initial_permissions: vec![
                (
                    dex_root.clone(),
                    Scope::Unlimited,
                    vec![permissions::INIT_DEX],
                ),
                (
                    dex_root.clone(),
                    Scope::Limited(hash(&0u32)),
                    vec![permissions::MANAGE_DEX],
                ),
                (
                    xor_fee_account_id,
                    Scope::Unlimited,
                    vec![permissions::MINT, permissions::BURN],
                ),
                (
                    initial_assets_owner,
                    Scope::Unlimited,
                    vec![
                        permissions::MINT,
                        permissions::BURN,
                        permissions::CREATE_FARM,
                        permissions::LOCK_TO_FARM,
                        permissions::UNLOCK_FROM_FARM,
                        permissions::CLAIM_FROM_FARM,
                    ],
                ),
                (
                    endowed_accounts[1].clone(),
                    Scope::Unlimited,
                    vec![
                        permissions::MINT,
                        permissions::BURN,
                        permissions::CREATE_FARM,
                        permissions::LOCK_TO_FARM,
                        permissions::UNLOCK_FROM_FARM,
                        permissions::CLAIM_FROM_FARM,
                    ],
                ),
                (
                    endowed_accounts[2].clone(),
                    Scope::Unlimited,
                    vec![
                        permissions::MINT,
                        permissions::BURN,
                        permissions::CREATE_FARM,
                        permissions::LOCK_TO_FARM,
                        permissions::UNLOCK_FROM_FARM,
                        permissions::CLAIM_FROM_FARM,
                    ],
                ),
                (
                    pswap_distribution_account_id,
                    Scope::Unlimited,
                    vec![permissions::MINT, permissions::BURN],
                ),
            ],
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .chain(once(faucet_account_id.clone()))
                .map(|k| (k, initial_balance.into()))
                .chain(once((
                    eth_bridge_account_id.clone(),
                    initial_eth_bridge_xor_amount.into(),
                )))
                .collect(),
        }),
        dex_manager: Some(DEXManagerConfig {
            dex_list: vec![(
                0,
                DEXInfo {
                    base_asset_id: GetBaseAssetId::get(),
                    default_fee: 30,
                    default_protocol_fee: 0,
                },
            )],
        }),
        mock_liquidity_source_Instance1: None,
        mock_liquidity_source_Instance2: None,
        mock_liquidity_source_Instance3: None,
        mock_liquidity_source_Instance4: None,
        faucet: Some(FaucetConfig {
            reserves_account_id: faucet_tech_account_id,
        }),
        tokens: Some(TokensConfig {
            endowed_accounts: vec![
                (
                    faucet_account_id.clone(),
                    ValId::get(),
                    initial_balance.into(),
                ),
                (faucet_account_id, PswapId::get(), initial_balance.into()),
                (
                    eth_bridge_account_id.clone(),
                    VAL,
                    initial_eth_bridge_val_amount.into(),
                ),
            ],
        }),
        dex_api: Some(DEXAPIConfig {
            source_types: [LiquiditySourceType::XYKPool].into(),
        }),
        eth_bridge: Some(EthBridgeConfig {
            peers: initial_bridge_peers.iter().cloned().collect(),
            bridge_account: eth_bridge_account_id.clone(),
            authority_account: eth_bridge_authority_account_id.clone(),
            tokens: vec![
                (
                    XOR.into(),
                    Some(sp_core::H160::from(hex!(
                        "40fd72257597aa14c7231a7b1aaa29fce868f677"
                    ))),
                    AssetKind::SidechainOwned,
                ),
                (
                    VAL.into(),
                    Some(sp_core::H160::from(hex!(
                        "3f9feac97e5feb15d8bf98042a9a01b515da3dfb"
                    ))),
                    AssetKind::SidechainOwned,
                ),
            ],
            pswap_owners: vec![],
        }),
        multisig: Some(MultisigConfig {
            accounts: once((
                eth_bridge_account_id.clone(),
                multisig::MultisigAccount::new(initial_bridge_peers, Percent::from_parts(67)),
            ))
            .collect(),
        }),
        bonding_curve_pool: Some(BondingCurvePoolConfig {
            distribution_accounts: accounts,
            reserves_account_id: bonding_curve_reserves_tech_account_id,
        }),
        farming: Some(FarmingConfig {
            initial_farm: (dex_root, XOR, PSWAP),
        }),
        pswap_distribution: Some(PswapDistributionConfig {
            subscribed_accounts: Vec::new(),
            burn_info: (
                fixed!(0),
                fixed!(0, 000369738339021615),
                fixed!(0, 65),
                14400,
            ),
        }),
    }
}
