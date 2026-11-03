use super::{
    FAMILY_ID_BITCOIN, FAMILY_ID_BITCOIN_CASH, FAMILY_ID_COSMOS, FAMILY_ID_DOGECOIN,
    FAMILY_ID_LITECOIN, FAMILY_ID_NEAR, FAMILY_ID_OSMOSIS, FAMILY_ID_SOLANA, FAMILY_ID_TRON,
};
use crate::dex::blockchain::Blockchain;

define_network! {
    bitcoin,
    "bitcoin",
    FAMILY_ID_BITCOIN,
    8,
    r"^([13][1-9A-HJ-NP-Za-km-z]{25,34}|bc1[ac-hj-np-z02-9]{39,59}|bc1p[ac-hj-np-z02-9]{58,59})$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
        Signet => NetworkId::SIGNET,
        Regtest => NetworkId::REGTEST,
    }
}

define_network! {
    bitcoin_cash,
    "bitcoin-cash",
    FAMILY_ID_BITCOIN_CASH,
    8,
    "^(bitcoincash:)?(q|p)[a-zA-HJ-NP-Z0-9]{41,42}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    cosmos,
    "cosmos",
    FAMILY_ID_COSMOS,
    6,
    "^cosmos1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Hub => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    dogecoin,
    "dogecoin",
    FAMILY_ID_DOGECOIN,
    8,
    "^D[1-9A-HJ-NP-Za-km-z]{25,34}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    litecoin,
    "litecoin",
    FAMILY_ID_LITECOIN,
    8,
    "^(L|M|ltc1|ltc1p)[a-zA-HJ-NP-Z0-9]{26,60}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    near,
    "near",
    FAMILY_ID_NEAR,
    24,
    "^(([a-z0-9]+[\\-_])?[a-z0-9]+\\.)?([a-z0-9]+[\\-_])?[a-z0-9]+$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    osmosis,
    "osmosis",
    FAMILY_ID_OSMOSIS,
    6,
    "^osmo1[a-zA-HJ-NP-Z0-9]{38}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
    }
}

define_network! {
    solana,
    "solana",
    FAMILY_ID_SOLANA,
    9,
    "^[1-9A-HJ-NP-Za-km-z]{32,44}$",
    {
        Mainnet => NetworkId::MAINNET,
        Testnet => NetworkId::TESTNET,
        Devnet => NetworkId::DEVNET,
    }
}

define_network! {
    tron,
    "tron",
    FAMILY_ID_TRON,
    6,
    "^T[a-zA-HJ-NP-Z0-9]{33}$",
    {
        Mainnet => NetworkId::MAINNET,
        Shasta => NetworkId::SHASTA,
        Nile => NetworkId::NILE,
    }
}
