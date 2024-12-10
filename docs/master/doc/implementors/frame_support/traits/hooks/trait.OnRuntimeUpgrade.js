(function() {var implementors = {
"apollo_platform":[["impl&lt;T: <a class=\"trait\" href=\"apollo_platform/pallet/trait.Config.html\" title=\"trait apollo_platform::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"apollo_platform/pallet/struct.Pallet.html\" title=\"struct apollo_platform::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"assets":[["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"assets/migration/asset_infos_v2/struct.AssetInfosUpdate.html\" title=\"struct assets::migration::asset_infos_v2::AssetInfosUpdate\">AssetInfosUpdate</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"assets/pallet/trait.Config.html\" title=\"trait assets::pallet::Config\">Config</a>,</span>"],["impl&lt;T, AssetId, AssetName, AssetSymbol, AssetOwner&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"assets/migration/register_asset/struct.RegisterAsset.html\" title=\"struct assets::migration::register_asset::RegisterAsset\">RegisterAsset</a>&lt;T, AssetId, AssetName, AssetSymbol, AssetOwner&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"assets/pallet/trait.Config.html\" title=\"trait assets::pallet::Config\">Config</a>,\n    AssetId: Get&lt;T::<a class=\"associatedtype\" href=\"assets/pallet/trait.Config.html#associatedtype.AssetId\" title=\"type assets::pallet::Config::AssetId\">AssetId</a>&gt;,\n    AssetName: Get&lt;AssetName&gt;,\n    AssetSymbol: Get&lt;AssetSymbol&gt;,\n    AssetOwner: Get&lt;T::AccountId&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"assets/pallet/trait.Config.html\" title=\"trait assets::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"assets/pallet/struct.Pallet.html\" title=\"struct assets::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"band":[["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"band/migrations/v2/struct.BandUpdateV2.html\" title=\"struct band::migrations::v2::BandUpdateV2\">BandUpdateV2</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"band/pallet/trait.Config.html\" title=\"trait band::pallet::Config\">Config</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"band/pallet/trait.Config.html\" title=\"trait band::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"band/pallet/struct.Pallet.html\" title=\"struct band::pallet::Pallet\">Pallet</a>&lt;T, I&gt;"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"band/migrations/v1/struct.BandUpdateV1.html\" title=\"struct band::migrations::v1::BandUpdateV1\">BandUpdateV1</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"band/pallet/trait.Config.html\" title=\"trait band::pallet::Config\">Config</a>,</span>"]],
"bridge_proxy":[["impl&lt;T: <a class=\"trait\" href=\"bridge_proxy/pallet/trait.Config.html\" title=\"trait bridge_proxy::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"bridge_proxy/migrations/generic_account_v2/struct.LiberlandGenericAccount.html\" title=\"struct bridge_proxy::migrations::generic_account_v2::LiberlandGenericAccount\">LiberlandGenericAccount</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"bridge_proxy/pallet/trait.Config.html\" title=\"trait bridge_proxy::pallet::Config\">Config</a>, ListAssets: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(AssetIdOf&lt;T&gt;, Balance)&gt;&gt;, NetworkId: Get&lt;GenericNetworkId&gt;&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"bridge_proxy/migrations/init/struct.InitLockedAssets.html\" title=\"struct bridge_proxy::migrations::init::InitLockedAssets\">InitLockedAssets</a>&lt;T, ListAssets, NetworkId&gt;"],["impl&lt;T: <a class=\"trait\" href=\"bridge_proxy/pallet/trait.Config.html\" title=\"trait bridge_proxy::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"bridge_proxy/pallet/struct.Pallet.html\" title=\"struct bridge_proxy::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"ceres_governance_platform":[["impl&lt;T: <a class=\"trait\" href=\"ceres_governance_platform/pallet/trait.Config.html\" title=\"trait ceres_governance_platform::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"ceres_governance_platform/pallet/struct.Pallet.html\" title=\"struct ceres_governance_platform::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"ceres_launchpad":[["impl&lt;T: <a class=\"trait\" href=\"ceres_launchpad/pallet/trait.Config.html\" title=\"trait ceres_launchpad::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"ceres_launchpad/pallet/struct.Pallet.html\" title=\"struct ceres_launchpad::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"ceres_liquidity_locker":[["impl&lt;T: <a class=\"trait\" href=\"ceres_liquidity_locker/pallet/trait.Config.html\" title=\"trait ceres_liquidity_locker::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"ceres_liquidity_locker/pallet/struct.Pallet.html\" title=\"struct ceres_liquidity_locker::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"ceres_staking":[["impl&lt;T: <a class=\"trait\" href=\"ceres_staking/pallet/trait.Config.html\" title=\"trait ceres_staking::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"ceres_staking/pallet/struct.Pallet.html\" title=\"struct ceres_staking::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"ceres_token_locker":[["impl&lt;T: <a class=\"trait\" href=\"ceres_token_locker/pallet/trait.Config.html\" title=\"trait ceres_token_locker::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"ceres_token_locker/pallet/struct.Pallet.html\" title=\"struct ceres_token_locker::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"demeter_farming_platform":[["impl&lt;T: <a class=\"trait\" href=\"demeter_farming_platform/pallet/trait.Config.html\" title=\"trait demeter_farming_platform::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"demeter_farming_platform/pallet/struct.Pallet.html\" title=\"struct demeter_farming_platform::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"dex_api":[["impl&lt;T: <a class=\"trait\" href=\"dex_api/pallet/trait.Config.html\" title=\"trait dex_api::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"dex_api/pallet/struct.Pallet.html\" title=\"struct dex_api::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"dex_manager":[["impl&lt;T: <a class=\"trait\" href=\"dex_manager/pallet/trait.Config.html\" title=\"trait dex_manager::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"dex_manager/migrations/kusd_dex/struct.AddKusdBasedDex.html\" title=\"struct dex_manager::migrations::kusd_dex::AddKusdBasedDex\">AddKusdBasedDex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"dex_manager/pallet/trait.Config.html\" title=\"trait dex_manager::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"dex_manager/migrations/vxor_dex/struct.AddVxorBasedDex.html\" title=\"struct dex_manager::migrations::vxor_dex::AddVxorBasedDex\">AddVxorBasedDex</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"dex_manager/pallet/trait.Config.html\" title=\"trait dex_manager::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"dex_manager/pallet/struct.Pallet.html\" title=\"struct dex_manager::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"eth_bridge":[["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"eth_bridge/pallet/struct.Pallet.html\" title=\"struct eth_bridge::pallet::Pallet\">Pallet</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: CreateSignedTransaction&lt;&lt;T as <a class=\"trait\" href=\"eth_bridge/pallet/trait.Config.html\" title=\"trait eth_bridge::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"eth_bridge/pallet/trait.Config.html#associatedtype.RuntimeCall\" title=\"type eth_bridge::pallet::Config::RuntimeCall\">RuntimeCall</a>&gt; + <a class=\"trait\" href=\"eth_bridge/pallet/trait.Config.html\" title=\"trait eth_bridge::pallet::Config\">Config</a>,</span>"]],
"extended_assets":[["impl&lt;T: <a class=\"trait\" href=\"extended_assets/pallet/trait.Config.html\" title=\"trait extended_assets::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"extended_assets/pallet/struct.Pallet.html\" title=\"struct extended_assets::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"farming":[["impl&lt;T: <a class=\"trait\" href=\"farming/pallet/trait.Config.html\" title=\"trait farming::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"farming/pallet/struct.Pallet.html\" title=\"struct farming::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T, P, B&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"farming/migrations/v3/struct.Migrate.html\" title=\"struct farming::migrations::v3::Migrate\">Migrate</a>&lt;T, P, B&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"farming/pallet/trait.Config.html\" title=\"trait farming::pallet::Config\">Config</a>,\n    P: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(T::AccountId, T::AccountId)&gt;&gt;,\n    B: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T::BlockNumber&gt;&gt;,</span>"],["impl&lt;T, G&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"farming/migrations/v2/struct.Migrate.html\" title=\"struct farming::migrations::v2::Migrate\">Migrate</a>&lt;T, G&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"farming/pallet/trait.Config.html\" title=\"trait farming::pallet::Config\">Config</a>,\n    G: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(T::AccountId, T::BlockNumber)&gt;&gt;,</span>"]],
"faucet":[["impl&lt;T: <a class=\"trait\" href=\"faucet/pallet/trait.Config.html\" title=\"trait faucet::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"faucet/pallet/struct.Pallet.html\" title=\"struct faucet::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"hermes_governance_platform":[["impl&lt;T: <a class=\"trait\" href=\"hermes_governance_platform/pallet/trait.Config.html\" title=\"trait hermes_governance_platform::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"hermes_governance_platform/pallet/struct.Pallet.html\" title=\"struct hermes_governance_platform::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"iroha_migration":[["impl&lt;T: <a class=\"trait\" href=\"iroha_migration/pallet/trait.Config.html\" title=\"trait iroha_migration::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"iroha_migration/pallet/struct.Pallet.html\" title=\"struct iroha_migration::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"kensetsu":[["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/pallet/struct.Pallet.html\" title=\"struct kensetsu::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a> + Config&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/migrations/v4_to_v5/struct.UpgradeToV5.html\" title=\"struct kensetsu::migrations::v4_to_v5::UpgradeToV5\">UpgradeToV5</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a> + Config + Config + Config&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/migrations/v3_to_v4/struct.UpgradeToV4.html\" title=\"struct kensetsu::migrations::v3_to_v4::UpgradeToV4\">UpgradeToV4</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a> + Config + Config + Config&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/migrations/v2_to_v3/struct.UpgradeToV3.html\" title=\"struct kensetsu::migrations::v2_to_v3::UpgradeToV3\">UpgradeToV3</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a> + Config + Config&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/migrations/init/struct.RegisterTreasuryTechAccount.html\" title=\"struct kensetsu::migrations::init::RegisterTreasuryTechAccount\">RegisterTreasuryTechAccount</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"kensetsu/pallet/trait.Config.html\" title=\"trait kensetsu::pallet::Config\">Config</a> + Config + Config + Config&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"kensetsu/migrations/v1_to_v2/struct.UpgradeToV2.html\" title=\"struct kensetsu::migrations::v1_to_v2::UpgradeToV2\">UpgradeToV2</a>&lt;T&gt;"]],
"liquidity_proxy":[["impl&lt;T: <a class=\"trait\" href=\"liquidity_proxy/pallet/trait.Config.html\" title=\"trait liquidity_proxy::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"liquidity_proxy/pallet/struct.Pallet.html\" title=\"struct liquidity_proxy::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"mock_liquidity_source":[["impl&lt;T: <a class=\"trait\" href=\"mock_liquidity_source/pallet/trait.Config.html\" title=\"trait mock_liquidity_source::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"mock_liquidity_source/pallet/struct.Pallet.html\" title=\"struct mock_liquidity_source::pallet::Pallet\">Pallet</a>&lt;T, I&gt;"]],
"multicollateral_bonding_curve_pool":[["impl&lt;T: <a class=\"trait\" href=\"multicollateral_bonding_curve_pool/pallet/trait.Config.html\" title=\"trait multicollateral_bonding_curve_pool::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"multicollateral_bonding_curve_pool/pallet/struct.Pallet.html\" title=\"struct multicollateral_bonding_curve_pool::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"multicollateral_bonding_curve_pool/migrations/v2/struct.InitializeTBCD.html\" title=\"struct multicollateral_bonding_curve_pool::migrations::v2::InitializeTBCD\">InitializeTBCD</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"multicollateral_bonding_curve_pool/pallet/trait.Config.html\" title=\"trait multicollateral_bonding_curve_pool::pallet::Config\">Config</a>,\n    &lt;T as Config&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt;,</span>"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"multicollateral_bonding_curve_pool/migrations/v3/struct.MigrateToV3.html\" title=\"struct multicollateral_bonding_curve_pool::migrations::v3::MigrateToV3\">MigrateToV3</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"multicollateral_bonding_curve_pool/pallet/trait.Config.html\" title=\"trait multicollateral_bonding_curve_pool::pallet::Config\">Config</a>,</span>"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"multicollateral_bonding_curve_pool/migrations/v1/struct.InitializeXSTPool.html\" title=\"struct multicollateral_bonding_curve_pool::migrations::v1::InitializeXSTPool\">InitializeXSTPool</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"multicollateral_bonding_curve_pool/pallet/trait.Config.html\" title=\"trait multicollateral_bonding_curve_pool::pallet::Config\">Config</a>,</span>"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"multicollateral_bonding_curve_pool/migrations/v4/struct.MigrateToV4.html\" title=\"struct multicollateral_bonding_curve_pool::migrations::v4::MigrateToV4\">MigrateToV4</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"multicollateral_bonding_curve_pool/pallet/trait.Config.html\" title=\"trait multicollateral_bonding_curve_pool::pallet::Config\">Config</a>,</span>"]],
"oracle_proxy":[["impl&lt;T: <a class=\"trait\" href=\"oracle_proxy/pallet/trait.Config.html\" title=\"trait oracle_proxy::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"oracle_proxy/pallet/struct.Pallet.html\" title=\"struct oracle_proxy::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"order_book":[["impl&lt;T: <a class=\"trait\" href=\"order_book/pallet/trait.Config.html\" title=\"trait order_book::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"order_book/pallet/struct.Pallet.html\" title=\"struct order_book::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"permissions":[["impl&lt;T: <a class=\"trait\" href=\"permissions/pallet/trait.Config.html\" title=\"trait permissions::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"permissions/pallet/struct.Pallet.html\" title=\"struct permissions::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"pool_xyk":[["impl&lt;T: <a class=\"trait\" href=\"pool_xyk/pallet/trait.Config.html\" title=\"trait pool_xyk::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pool_xyk/pallet/struct.Pallet.html\" title=\"struct pool_xyk::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T, L&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pool_xyk/migrations/v3/struct.XYKPoolUpgrade.html\" title=\"struct pool_xyk::migrations::v3::XYKPoolUpgrade\">XYKPoolUpgrade</a>&lt;T, L&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"pool_xyk/pallet/trait.Config.html\" title=\"trait pool_xyk::pallet::Config\">Config</a>,\n    L: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(AssetIdOf&lt;T&gt;, AssetIdOf&lt;T&gt;, T::DEXId)&gt;&gt;,</span>"]],
"presto":[["impl&lt;T: <a class=\"trait\" href=\"presto/pallet/trait.Config.html\" title=\"trait presto::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"presto/pallet/struct.Pallet.html\" title=\"struct presto::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"price_tools":[["impl&lt;T: <a class=\"trait\" href=\"price_tools/pallet/trait.Config.html\" title=\"trait price_tools::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"price_tools/migrations/v3/struct.AddFastPriceInfos.html\" title=\"struct price_tools::migrations::v3::AddFastPriceInfos\">AddFastPriceInfos</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"price_tools/pallet/trait.Config.html\" title=\"trait price_tools::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"price_tools/pallet/struct.Pallet.html\" title=\"struct price_tools::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"pswap_distribution":[["impl&lt;T, G&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pswap_distribution/migrations/v2/struct.Migrate.html\" title=\"struct pswap_distribution::migrations::v2::Migrate\">Migrate</a>&lt;T, G&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"pswap_distribution/pallet/trait.Config.html\" title=\"trait pswap_distribution::pallet::Config\">Config</a>,\n    G: Get&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;(T::AccountId, T::AccountId)&gt;&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"pswap_distribution/pallet/trait.Config.html\" title=\"trait pswap_distribution::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"pswap_distribution/pallet/struct.Pallet.html\" title=\"struct pswap_distribution::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"qa_tools":[["impl&lt;T: <a class=\"trait\" href=\"qa_tools/pallet/trait.Config.html\" title=\"trait qa_tools::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"qa_tools/pallet/struct.Pallet.html\" title=\"struct qa_tools::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"referrals":[["impl&lt;T: <a class=\"trait\" href=\"referrals/pallet/trait.Config.html\" title=\"trait referrals::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"referrals/pallet/struct.Pallet.html\" title=\"struct referrals::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"rewards":[["impl&lt;T: <a class=\"trait\" href=\"rewards/pallet/trait.Config.html\" title=\"trait rewards::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"rewards/pallet/struct.Pallet.html\" title=\"struct rewards::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"soratopia":[["impl&lt;T: <a class=\"trait\" href=\"soratopia/pallet/trait.Config.html\" title=\"trait soratopia::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"soratopia/pallet/struct.Pallet.html\" title=\"struct soratopia::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"technical":[["impl&lt;T: <a class=\"trait\" href=\"technical/pallet/trait.Config.html\" title=\"trait technical::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"technical/pallet/struct.Pallet.html\" title=\"struct technical::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"trading_pair":[["impl&lt;T: <a class=\"trait\" href=\"trading_pair/pallet/trait.Config.html\" title=\"trait trading_pair::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"trading_pair/pallet/struct.Pallet.html\" title=\"struct trading_pair::pallet::Pallet\">Pallet</a>&lt;T&gt;"]],
"vested_rewards":[["impl&lt;T: <a class=\"trait\" href=\"vested_rewards/pallet/trait.Config.html\" title=\"trait vested_rewards::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"vested_rewards/pallet/struct.Pallet.html\" title=\"struct vested_rewards::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T: <a class=\"trait\" href=\"vested_rewards/pallet/trait.Config.html\" title=\"trait vested_rewards::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"vested_rewards/migrations/v4/struct.Migration.html\" title=\"struct vested_rewards::migrations::v4::Migration\">Migration</a>&lt;T&gt;"]],
"xor_fee":[["impl&lt;T: <a class=\"trait\" href=\"xor_fee/pallet/trait.Config.html\" title=\"trait xor_fee::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"xor_fee/pallet/struct.Pallet.html\" title=\"struct xor_fee::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"xor_fee/migrations/remove_vxor_remint/struct.Migrate.html\" title=\"struct xor_fee::migrations::remove_vxor_remint::Migrate\">Migrate</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"xor_fee/pallet/trait.Config.html\" title=\"trait xor_fee::pallet::Config\">Config</a>,</span>"]],
"xst":[["impl&lt;T: <a class=\"trait\" href=\"xst/pallet/trait.Config.html\" title=\"trait xst::pallet::Config\">Config</a>&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"xst/pallet/struct.Pallet.html\" title=\"struct xst::pallet::Pallet\">Pallet</a>&lt;T&gt;"],["impl&lt;T&gt; OnRuntimeUpgrade for <a class=\"struct\" href=\"xst/migrations/struct.CustomSyntheticsUpgrade.html\" title=\"struct xst::migrations::CustomSyntheticsUpgrade\">CustomSyntheticsUpgrade</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"xst/pallet/trait.Config.html\" title=\"trait xst::pallet::Config\">Config</a>,\n    &lt;T as Config&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">32</a>]&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()