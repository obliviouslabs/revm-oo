### Revm-oo

This project is a fork of [revm](https://github.com/bluealloy/revm) with opcodes rewriten such that the cpu and memory acccess trace during each opcode execution won't leak any information about the operands of that opcode. This project is a first step towards a fully oblivious evm, that can run in Trusted Execution Environments, while still being resistant to memory and instruction trace side channels.

For information about which data is leaked during each opcode execution, please take a look at [OPCODES.md](./OPCODES.md)

> [!WARNING]
> This project is still at early development stages and partially incomplete and it's not yet intended to be used in production.
> Currently we have modified most of the arithmetic and bitwise opcodes. The memory access opcodes need to use an oblivious database backend such as [obliviouslabs/oram](https://github.com/obliviouslabs/oram) in order to have privacy guarentees. Take a look at [OPCODES.md](./OPCODES.md) to see the state for each opcode.
> Feel free to submit a pull request making progress towards rewriting the remaining opcodes in [OPCODES.md](./OPCODES.md) that are not oblivious yet.
