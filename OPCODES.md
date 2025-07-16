# List of each opcode and it's leakage in revm and revm-oo

| **Opcode** | **Name** | **Minimum Gas** | **Stack Input** | **Stack Output** | **DescriptionExpand** | Leakage revm | Gas leakage revm | Leakage revm-oo |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 00 | STOP | 0 |  |  | Halts execution | - | - | - |
| 01 | ADD | 3 | ab | a + b | Addition operation | - | - | - |
| 02 | MUL | 5 | ab | a * b | Multiplication operation | - | - | - |
| 03 | SUB | 3 | ab | a - b | Subtraction operation | - | - | - |
| 04 | DIV | 5 | ab | a // b | Integer division operation | b.is_zero() | - | - |
| 05 | SDIV | 5 | ab | a // b | Signed integer division operation (truncated) | b.is_zero(), a==-1 && b=1, sign(a)==sign(b) | - | ? |
| 06 | MOD | 5 | ab | a % b | Modulo remainder operation | b.is_zero() | - | - |
| 07 | SMOD | 5 | ab | a % b | Signed modulo remainder operation | a.is_zero(), b.is_zero(), a<0 | - | ? |
| 08 | ADDMOD | 8 | abN | (a + b) % N | Modulo addition operation | a+b > N | - | ? |
| 09 | MULMOD | 8 | abN | (a * b) % N | Modulo multiplication operation | N == 0 | - | ? |
| 0a | EXP | 10 | aexponent | a ** exponent | Exponential operation | exp | exp = 0 | ? |
| 0b | SIGNEXTEND | 5 | bx | y | Extend length of two’s complement signed integer | ext<31, x & (1<<ext),  | - | ext < 31 |
| 10 | LT | 3 | ab | a < b | Less-than comparison | - | - | - |
| 11 | GT | 3 | ab | a > b | Greater-than comparison | - | - | - |
| 12 | SLT | 3 | ab | a < b | Signed less-than comparison | a.sign(), b.sign() | - | - |
| 13 | SGT | 3 | ab | a > b | Signed greater-than comparison | a.sign(), b.sign() | - | - |
| 14 | EQ | 3 | ab | a == b | Equality comparison | - | - | - |
| 15 | ISZERO | 3 | a | a == 0 | Is-zero comparison | - | - | - |
| 16 | AND | 3 | ab | a & b | Bitwise AND operation | - | - | - |
| 17 | OR | 3 | ab | a | b | Bitwise OR operation | - | - | - |
| 18 | XOR | 3 | ab | a ^ b | Bitwise XOR operation | - | - | - |
| 19 | NOT | 3 | a | ~a | Bitwise NOT operation | - | - | - |
| 1a | BYTE | 3 | ix | y | Retrieve single byte from word | x < 32 | - | - |
| 1b | SHL | 3 | shiftvalue | value << shift | Left shift operation | shift < 256 | - | shift < 256 |
| 1c | SHR | 3 | shiftvalue | value >> shift | Logical right shift operation | shift < 256 | - | shift < 256 |
| 1d | SAR | 3 | shiftvalue | value >> shift | Arithmetic (signed) right shift operation | shift < 256; value.bit(255) | - | ? |
| 20 | KECCAK256 | 30 | offsetsize | hash | Compute Keccak-256 hash | size, mem_increased | size | ? |
| 30 | ADDRESS | 2 |  | address | Get address of currently executing account | - | - | - |
| 31 | BALANCE | 100 | address | balance | Get balance of the given account | address | address.is_balance_cold() | ? |
| 32 | ORIGIN | 2 |  | address | Get execution origination address | - | - | - |
| 33 | CALLER | 2 |  | address | Get caller address | - | - | - |
| 34 | CALLVALUE | 2 |  | value | Get deposited value by the instruction/transaction responsible for this execution | - | - | - |
| 35 | CALLDATALOAD | 3 | i | data[i] | Get input data of current environment | i | - | ? |
| 36 | CALLDATASIZE | 2 |  | size | Get size of input data in current environment | - | - | - |
| 37 | CALLDATACOPY | 3 | destOffsetoffsetsize |  | Copy input data in current environment to memory | destOffset, offset, size | - | ? |
| 38 | CODESIZE | 2 |  | size | Get size of code running in current environment | - | - | - |
| 39 | CODECOPY | 3 | destOffsetoffsetsize |  | Copy code running in current environment to memory | destOffset, offset, size | - | ? |
| 3a | GASPRICE | 2 |  | price | Get price of gas in current environment | - | - | - |
| 3b | EXTCODESIZE | 100 | address | size | Get size of an account’s code | address | address.is_code_cold() | ? |
| 3c | EXTCODECOPY | 100 | addressdestOffsetoffsetsize |  | Copy an account’s code to memory | address, destOffset, offset, size, mem_increased | size, address.is_code_cold() | ? |
| 3d | RETURNDATASIZE | 2 |  | size | Get size of output data from the previous call from the current environment | - | - | - |
| 3e | RETURNDATACOPY | 3 | destOffsetoffsetsize |  | Copy output data from the previous call to memory | ret.size(), size, destOffset, offset | - | ? |
| 3f | EXTCODEHASH | 100 | address | hash | Get hash of an account’s code | address | address.is_code_hash_cold() | ? |
| 40 | BLOCKHASH | 20 | blockNumber | hash | Get the hash of one of the 256 most recent complete blocks | blockNumber |  | ? |
| 41 | COINBASE | 2 |  | address | Get the block’s beneficiary address | - | - | - |
| 42 | TIMESTAMP | 2 |  | timestamp | Get the block’s timestamp | - | - | - |
| 43 | NUMBER | 2 |  | blockNumber | Get the block’s number | - | - | - |
| 44 | PREVRANDAO | 2 |  | difficulty | Get the block’s difficulty | - | - | - |
| 45 | GASLIMIT | 2 |  | gasLimit | Get the block’s gas limit | - | - | - |
| 46 | CHAINID | 2 |  | chainId | Get the chain ID | - | - | - |
| 47 | SELFBALANCE | 5 |  | balance | Get balance of currently executing account | account | - | ? |
| 48 | BASEFEE | 2 |  | baseFee | Get the base fee | - | - | - |
| 49 | BLOBHASH | 3 | index | blobVersionedHashesAtIndex | Get versioned hashes | index | - |  |
| 4a | BLOBBASEFEE | 2 |  | blobBaseFee | Returns the value of the blob base-fee of the current block | - | - | - |
| 50 | POP | 2 | y |  | Remove item from stack | - | - | - |
| 51 | MLOAD | 3 | offset | value | Load word from memory | offset, mem_increased | - | ? |
| 52 | MSTORE | 3 | offsetvalue |  | Save word to memory | offset, mem_increased | - | ? |
| 53 | MSTORE8 | 3 | offsetvalue |  | Save byte to memory | offset, mem_increased |  | ? |
| 54 | SLOAD | 100 | key | value | Load word from storage | key | key.is_cold() | ? |
| 55 | SSTORE | 100 | keyvalue |  | Save word to storage | key | key.is_cold(), value.is_zero() | ? |
| 56 | JUMP | 8 | counter |  | Alter the program counter | counter | - | counter |
| 57 | JUMPI | 10 | counterb |  | Conditionally alter the program counter | counter, b | - | b, b ⇒ counter |
| 58 | PC | 2 |  | counter | Get the value of the program counter prior to the increment corresponding to this instruction | - | - | - |
| 59 | MSIZE | 2 |  | size | Get the size of active memory in bytes | - | - | - |
| 5a | GAS | 2 |  | gas | Get the amount of available gas, including the corresponding reduction for the cost of this instruction | - | - | - |
| 5b | JUMPDEST | 1 |  |  | Mark a valid destination for jumps | - | - | - |
| 5c | TLOAD | 100 | key | value | Load word from transient storage | key | - | ? |
| 5d | TSTORE | 100 | keyvalue |  | Save word to transient storage | key | - | ? |
| 5e | MCOPY | 3 | destOffsetoffsetsize |  | Copy memory areas | dst, src, len, mem_increased | - | ? |
| 5f | PUSH0 | 2 |  | 0 | Place value 0 on stack | stack_overflow | - | ? |
| 60-7f | PUSH_N | 3 |  | value | Place N byte item on stack | stack_overflow | - | ? |
| 80-8f | DUP1_N | 3 | value | valuevalue | Duplicate n-th stack item | stack_overflow | - | ? |
| 90-9f | SWAP_N | 3 | ab | ba | Exchange 1st and (1+N)-th stack items | - | - | - |
| a0 | LOG0 | 375 | offsetsize |  | Append log record with no topics | offset, size | - | ? |
| a1 | LOG1 | 750 | offsetsizetopic |  | Append log record with one topic | offset, size | - | ? |
| a2 | LOG2 | 1125 | offsetsizetopic1topic2 |  | Append log record with two topics | offset, size | - | ? |
| a3 | LOG3 | 1500 | offsetsizetopic1topic2topic3 |  | Append log record with three topics | offset, size | - | ? |
| a4 | LOG4 | 1875 | offsetsizetopic1topic2topic3topic4 |  | Append log record with four topics | offset, size | - | ? |
| f0 | CREATE | 32000 | valueoffsetsize | address | Create a new account with associated code | offset, size, address | - | ? |
| f1 | CALL | 100 | gasaddressvalueargsOffsetargsSizeretOffsetretSize | success | Message-call into an account | * | * | ? |
| f2 | CALLCODE | 100 | gasaddressvalueargsOffsetargsSizeretOffsetretSize | success | Message-call into this account with alternative account’s code | * | * | ? |
| f3 | RETURN | 0 | offsetsize |  | Halt execution returning output data | offset, size | - | ? |
| f4 | DELEGATECALL | 100 | gasaddressargsOffsetargsSizeretOffsetretSize | success | Message-call into this account with an alternative account’s code, but persisting the current values for sender and value | * | * | ? |
| f5 | CREATE2 | 32000 | valueoffsetsizesalt | address | Create a new account with associated code at a predictable address | offset, size, address | - | ? |
| fa | STATICCALL | 100 | gasaddressargsOffsetargsSizeretOffsetretSize | success | Static message-call into an account | * | * | ? |
| fd | REVERT | 0 | offsetsize |  | Halt execution reverting state changes but returning data and remaining gas | offset, size | - | ? |
| fe | INVALID | NaN |  |  | Designated invalid instruction | - | - | - |
| ff | SELFDESTRUCT | 5000 | address |  | Halt execution and register account for later deletion or send all Ether to address (post-Cancun) | address | - | ? |