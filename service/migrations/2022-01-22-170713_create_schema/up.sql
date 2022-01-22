create table account(
    id integer primary key,
    account_number integer not null unique, -- Long 16 digit number to login to the account
    payout_info text, -- JSON with saved LNURL, Liquid or onchain address
    passhash text, -- Optional password hash
    created integer not null -- timestamp when they created
    -- TODO: reserve info for lnauth
);

create table utxos(
    id integer primary key,
    txid text not null,
    vout integer not null,
    satoshi integer not null,
    asset_id text not null,
    account integer not null,
    created integer not null, -- timestamp of block when tx included
    spent integer, -- timestamp of block where the utxo spent
    foreign key(account) references account(id),
    constraint unique_utxo unique (txid, vout)
);

create table dividends(
    id integer primary key,
    created integer not null, -- timestamp when they created
    amount integer not null, -- Amount of asset payed
    asset_id text not null, -- ID of asset, for Stellar "EURMTL:<issuer>"
    rate integer not null -- sats/asset exchange rate at moment of payment
);

create table payout(
    id integer primary key,
    performed integer not null, -- timestamp when the payout peformed
    account integer not null,
    amount integer not null, -- sats payed out
    payout_info text not null, -- JSON with additional info, ex txid
    foreign key(account) references account(id)
)