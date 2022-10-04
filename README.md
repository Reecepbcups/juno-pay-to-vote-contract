# Juno Paid To Vote

(WIP) The actual query does not work yet since it requires some Protobuf.

```bash
# cargo install cargo-generate --features vendored-openssl
# cargo install cargo-run-script
cargo generate --git https://github.com/Reecepbcups/cw-template.git --branch main --name PROJECT_NAME
```

Idea:
- Smart contract which holds X funds from the community pool
- If a user votes on a proposal, we can send them X juno per votes
- needs an admin (multisig / dao) to controll it so no one can just walk away with funds in it