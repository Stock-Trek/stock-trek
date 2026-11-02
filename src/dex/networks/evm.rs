use crate::dex::asset_on_chain::AssetOnChain;

define_evm_network! {
    arbitrum,
    "arbitrum",
    {
        Mainnet => (42161, NetworkId::MAINNET),
        Sepolia => (421614, NetworkId::new("sepolia")?),
        Nova => (42170, NetworkId::new("nova")?),
    }
}

define_evm_network! {
    avalanche,
    "avalanche",
    {
        Mainnet => (43114, NetworkId::MAINNET),
        Fuji => (43113, NetworkId::new("fuji")?),
    }
}

define_evm_network! {
    base,
    "base",
    {
        Mainnet => (8453, NetworkId::MAINNET),
        Sepolia => (84532, NetworkId::new("sepolia")?),
    }
}

define_evm_network! {
    bsc,
    "bsc",
    {
        Mainnet => (56, NetworkId::MAINNET),
        Testnet => (97, NetworkId::TESTNET),
    }
}

define_evm_network! {
    celo,
    "celo",
    {
        Mainnet => (42220, NetworkId::MAINNET),
        Alfajores => (44787, NetworkId::new("alfajores")?),
    }
}

define_evm_network! {
    cronos,
    "cronos",
    {
        Mainnet => (25, NetworkId::MAINNET),
        Testnet => (338, NetworkId::TESTNET),
    }
}

define_evm_network! {
    ethereum,
    "ethereum",
    {
        Mainnet => (1, NetworkId::MAINNET),
        Sepolia => (11155111, NetworkId::new("sepolia")?),
        Holesky => (17000, NetworkId::new("holesky")?),
    }
}

define_evm_network! {
    fantom,
    "fantom",
    {
        Mainnet => (250, NetworkId::MAINNET),
        Testnet => (4002, NetworkId::TESTNET),
    }
}

define_evm_network! {
    gnosis,
    "gnosis",
    {
        Mainnet => (100, NetworkId::MAINNET),
        Chiado => (10200, NetworkId::new("chiado")?),
    }
}

define_evm_network! {
    moonbeam,
    "moonbeam",
    {
        Mainnet => (1284, NetworkId::MAINNET),
        Moonbase => (1287, NetworkId::new("moonbase")?),
    }
}

define_evm_network! {
    optimism,
    "optimism",
    {
        Mainnet => (10, NetworkId::MAINNET),
        Sepolia => (11155420, NetworkId::new("sepolia")?),
    }
}

define_evm_network! {
    polygon,
    "polygon",
    {
        Mainnet => (137, NetworkId::MAINNET),
        Amoy => (80002, NetworkId::new("amoy")?),
        Mumbai => (80001, NetworkId::new("mumbai")?),
    }
}
