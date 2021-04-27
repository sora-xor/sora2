// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cli::{Cli, Subcommand};
use crate::{chain_spec, service};
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

fn set_default_ss58_version() {
    sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::Custom(
        framenode_runtime::SS58Prefix::get() as u16,
    ));
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "SORA".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/sora-xor/sora2-network/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        #[cfg(feature = "private-net")]
        let chain_spec = match id {
            "" | "local" => Ok(chain_spec::local_testnet_config()),
            // dev doesn't use json chain spec to make development easier
            // "dev" => chain_spec::dev_net(),
            // "dev-coded" => Ok(chain_spec::dev_net_coded()),
            "dev" => Ok(chain_spec::dev_net_coded()),
            "test" => chain_spec::test_net(),
            "test-coded" => Ok(chain_spec::staging_net_coded(true)),
            "staging" => chain_spec::staging_net(),
            "staging-coded" => Ok(chain_spec::staging_net_coded(false)),
            path => chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path)),
        };

        #[cfg(not(feature = "private-net"))]
        let chain_spec = match id {
            "main" => chain_spec::main_net(),
            "" | "main-coded" => Ok(chain_spec::main_net_coded()),
            path => chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path)),
        };

        Ok(Box::new(chain_spec?))
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &framenode_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();
            runner.sync_run(|config| cmd.run::<framenode_runtime::Block, service::Executor>(config))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            set_default_ss58_version();
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    Role::Light => service::new_light(config),
                    _ => service::new_full(config),
                }
                .map_err(sc_cli::Error::Service)
            })
        }
    }
}
