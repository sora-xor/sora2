(function() {var implementors = {
"common":[["impl&lt;AmountType&gt; MaxEncodedLen for <a class=\"enum\" href=\"common/prelude/enum.SwapAmount.html\" title=\"enum common::prelude::SwapAmount\">SwapAmount</a>&lt;AmountType&gt;<span class=\"where fmt-newline\">where\n    AmountType: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"common/prelude/enum.DEXId.html\" title=\"enum common::prelude::DEXId\">DEXId</a>"],["impl&lt;AssetId&gt; MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.TradingPair.html\" title=\"struct common::prelude::TradingPair\">TradingPair</a>&lt;AssetId&gt;<span class=\"where fmt-newline\">where\n    AssetId: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"common/prelude/enum.PredefinedAssetId.html\" title=\"enum common::prelude::PredefinedAssetId\">PredefinedAssetId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"common/prelude/enum.LiquiditySourceType.html\" title=\"enum common::prelude::LiquiditySourceType\">LiquiditySourceType</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.Description.html\" title=\"struct common::prelude::Description\">Description</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.BalanceUnit.html\" title=\"struct common::prelude::BalanceUnit\">BalanceUnit</a>"],["impl&lt;AssetId&gt; MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.AssetId32.html\" title=\"struct common::prelude::AssetId32\">AssetId32</a>&lt;AssetId&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html\" title=\"struct core::marker::PhantomData\">PhantomData</a>&lt;AssetId&gt;: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.ContentSource.html\" title=\"struct common::prelude::ContentSource\">ContentSource</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"common/prelude/enum.PriceVariant.html\" title=\"enum common::prelude::PriceVariant\">PriceVariant</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.AssetName.html\" title=\"struct common::prelude::AssetName\">AssetName</a>"],["impl&lt;N: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"struct\" href=\"common/prelude/struct.BoundedString.html\" title=\"struct common::prelude::BoundedString\">BoundedString</a>&lt;N&gt;<span class=\"where fmt-newline\">where\n    BoundedVec&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>, N&gt;: MaxEncodedLen,</span>"]],
"eth_bridge":[["impl MaxEncodedLen for <a class=\"struct\" href=\"eth_bridge/offchain/crypto/struct.Public.html\" title=\"struct eth_bridge::offchain::crypto::Public\">Public</a>"]],
"extended_assets":[["impl&lt;Moment, AssetId, MaxRegulatedAssetsPerSBT: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt; MaxEncodedLen for <a class=\"struct\" href=\"extended_assets/struct.SoulboundTokenMetadata.html\" title=\"struct extended_assets::SoulboundTokenMetadata\">SoulboundTokenMetadata</a>&lt;Moment, AssetId, MaxRegulatedAssetsPerSBT&gt;<span class=\"where fmt-newline\">where\n    Moment: MaxEncodedLen,\n    BoundedBTreeSet&lt;AssetId, MaxRegulatedAssetsPerSBT&gt;: MaxEncodedLen,</span>"]],
"framenode_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"framenode_runtime/enum.OriginCaller.html\" title=\"enum framenode_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"framenode_runtime/struct.NposCompactSolution24.html\" title=\"struct framenode_runtime::NposCompactSolution24\">NposCompactSolution24</a>"]],
"kensetsu":[["impl&lt;Moment&gt; MaxEncodedLen for <a class=\"struct\" href=\"kensetsu/struct.CollateralInfo.html\" title=\"struct kensetsu::CollateralInfo\">CollateralInfo</a>&lt;Moment&gt;<span class=\"where fmt-newline\">where\n    Moment: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"kensetsu/struct.CollateralRiskParameters.html\" title=\"struct kensetsu::CollateralRiskParameters\">CollateralRiskParameters</a>"],["impl&lt;AccountId, AssetId&gt; MaxEncodedLen for <a class=\"struct\" href=\"kensetsu/struct.CollateralizedDebtPosition.html\" title=\"struct kensetsu::CollateralizedDebtPosition\">CollateralizedDebtPosition</a>&lt;AccountId, AssetId&gt;<span class=\"where fmt-newline\">where\n    AccountId: MaxEncodedLen,\n    AssetId: MaxEncodedLen,</span>"],["impl&lt;AssetId&gt; MaxEncodedLen for <a class=\"struct\" href=\"kensetsu/struct.StablecoinCollateralIdentifier.html\" title=\"struct kensetsu::StablecoinCollateralIdentifier\">StablecoinCollateralIdentifier</a>&lt;AssetId&gt;<span class=\"where fmt-newline\">where\n    AssetId: MaxEncodedLen,</span>"]],
"order_book":[["impl MaxEncodedLen for <a class=\"enum\" href=\"order_book/types/enum.CancelReason.html\" title=\"enum order_book::types::CancelReason\">CancelReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"order_book/types/enum.OrderBookStatus.html\" title=\"enum order_book::types::OrderBookStatus\">OrderBookStatus</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"order_book/types/enum.OrderBookTechStatus.html\" title=\"enum order_book::types::OrderBookTechStatus\">OrderBookTechStatus</a>"],["impl&lt;T&gt; MaxEncodedLen for <a class=\"struct\" href=\"order_book/struct.MarketOrder.html\" title=\"struct order_book::MarketOrder\">MarketOrder</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"order_book/pallet/trait.Config.html\" title=\"trait order_book::pallet::Config\">Config</a>,\n    T::AccountId: MaxEncodedLen,\n    <a class=\"struct\" href=\"order_book/types/struct.OrderBookId.html\" title=\"struct order_book::types::OrderBookId\">OrderBookId</a>&lt;AssetIdOf&lt;T&gt;, T::DEXId&gt;: MaxEncodedLen,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T::AccountId&gt;: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"order_book/types/enum.OrderAmount.html\" title=\"enum order_book::types::OrderAmount\">OrderAmount</a>"],["impl&lt;T&gt; MaxEncodedLen for <a class=\"struct\" href=\"order_book/struct.LimitOrder.html\" title=\"struct order_book::LimitOrder\">LimitOrder</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"order_book/pallet/trait.Config.html\" title=\"trait order_book::pallet::Config\">Config</a>,\n    T::<a class=\"associatedtype\" href=\"order_book/pallet/trait.Config.html#associatedtype.OrderId\" title=\"type order_book::pallet::Config::OrderId\">OrderId</a>: MaxEncodedLen,\n    T::AccountId: MaxEncodedLen,\n    <a class=\"type\" href=\"order_book/type.MomentOf.html\" title=\"type order_book::MomentOf\">MomentOf</a>&lt;T&gt;: MaxEncodedLen,\n    BlockNumberFor&lt;T&gt;: MaxEncodedLen,</span>"],["impl&lt;T&gt; MaxEncodedLen for <a class=\"struct\" href=\"order_book/struct.OrderBook.html\" title=\"struct order_book::OrderBook\">OrderBook</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"order_book/pallet/trait.Config.html\" title=\"trait order_book::pallet::Config\">Config</a>,\n    <a class=\"struct\" href=\"order_book/types/struct.OrderBookId.html\" title=\"struct order_book::types::OrderBookId\">OrderBookId</a>&lt;AssetIdOf&lt;T&gt;, T::DEXId&gt;: MaxEncodedLen,\n    T::<a class=\"associatedtype\" href=\"order_book/pallet/trait.Config.html#associatedtype.OrderId\" title=\"type order_book::pallet::Config::OrderId\">OrderId</a>: MaxEncodedLen,</span>"],["impl&lt;AssetId, DEXId&gt; MaxEncodedLen for <a class=\"struct\" href=\"order_book/types/struct.OrderBookId.html\" title=\"struct order_book::types::OrderBookId\">OrderBookId</a>&lt;AssetId, DEXId&gt;<span class=\"where fmt-newline\">where\n    DEXId: MaxEncodedLen,\n    AssetId: MaxEncodedLen,</span>"]],
"vested_rewards":[["impl&lt;BlockNumber, AssetId&gt; MaxEncodedLen for <a class=\"struct\" href=\"vested_rewards/vesting_currencies/struct.LinearVestingSchedule.html\" title=\"struct vested_rewards::vesting_currencies::LinearVestingSchedule\">LinearVestingSchedule</a>&lt;BlockNumber, AssetId&gt;<span class=\"where fmt-newline\">where\n    AssetId: MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    BlockNumber: MaxEncodedLen,</span>"],["impl&lt;BlockNumber, AssetId: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>, AccountId&gt; MaxEncodedLen for <a class=\"enum\" href=\"vested_rewards/vesting_currencies/enum.VestingScheduleVariant.html\" title=\"enum vested_rewards::vesting_currencies::VestingScheduleVariant\">VestingScheduleVariant</a>&lt;BlockNumber, AssetId, AccountId&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"vested_rewards/vesting_currencies/struct.LinearVestingSchedule.html\" title=\"struct vested_rewards::vesting_currencies::LinearVestingSchedule\">LinearVestingSchedule</a>&lt;BlockNumber, AssetId&gt;: MaxEncodedLen,\n    <a class=\"struct\" href=\"vested_rewards/vesting_currencies/struct.LinearPendingVestingSchedule.html\" title=\"struct vested_rewards::vesting_currencies::LinearPendingVestingSchedule\">LinearPendingVestingSchedule</a>&lt;BlockNumber, AssetId, AccountId&gt;: MaxEncodedLen,</span>"],["impl&lt;BlockNumber, AssetId, AccountId&gt; MaxEncodedLen for <a class=\"struct\" href=\"vested_rewards/vesting_currencies/struct.LinearPendingVestingSchedule.html\" title=\"struct vested_rewards::vesting_currencies::LinearPendingVestingSchedule\">LinearPendingVestingSchedule</a>&lt;BlockNumber, AssetId, AccountId&gt;<span class=\"where fmt-newline\">where\n    AssetId: MaxEncodedLen + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;AccountId&gt;: MaxEncodedLen,\n    <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;BlockNumber&gt;: MaxEncodedLen,\n    BlockNumber: MaxEncodedLen,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()