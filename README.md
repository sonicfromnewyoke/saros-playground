# Saros Playground

## Source Tree

```
saros-playground/
.
├── playground/     the main package for checking your assumptions
|   └── src/  
|       ├── amm.rs      Saros AMM related manipulations (swap, liq add/remove, etc)
|       ├── dlmm.rs     Saros DLMM related manipulations (swap, liq add/remove, etc)
|       ├── keys.rs     public keys of accounts
|       └── setup.rs    bunch of helpers that simplifies SVM preparation
├── programs/
|   └── src/     
|       ├── elf/            onchain bins of programs
|       ├── amm.rs          setup of Saros AMM in Mollusk SVM
|       ├── dlmm.rs         setup of Saros DLMM in Mollusk SVM
|       ├── update.sh       script for dumping ELFs from mainnet-beta
|       └── dlmm_idl.json   Saros DLMM Official IDL
└─── sdk/
    ├── amm/         state and ix configurations for AMM
    └── dlmm/        state and ix configurations for DLMM
```

## How To Use?

### Actualize ELFs

```sh
cd programs

./update.sh
```

### Run predefined Swaps

from the root

```sh
cargo run -p playground swap-amm
```
output:
```
Program SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr invoke [1]
Program log: Instruction: Swap
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: Transfer
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4736 of 1384466 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: MintTo
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4492 of 1355438 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: Transfer
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 1347988 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr consumed 57240 of 1400000 compute units
Program SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr success
```


```sh
cargo run -p playground swap-dlmm
```
output:
```
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [1]
Program log: Instruction: Swap
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: TransferChecked
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6238 of 1287985 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
Program log: Instruction: TransferChecked
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 6173 of 1278961 compute units
Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1270180 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1264158 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1258136 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
rogram 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1252114 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1246092 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1240070 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1234048 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1228026 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1222004 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1215982 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1209960 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1203938 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1197916 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1191894 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1185872 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE invoke [2]
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 3503 of 1179850 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE consumed 225706 of 1400000 compute units
Program 1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE success
```

### Your Own Journey

1. Modify state of SVM (pool, vaults, bin arrays, etc)
2. Verify your shady idea
3. Use received knowledge wisely
