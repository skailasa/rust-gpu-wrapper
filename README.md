## TODO

[1] Extend build script to support HIP kernels too

[2] Set hardware configuration automatically.

[3] Ensure that users don't have to manually edit build.rs file for appropriate library paths and that these are instead set by environment variables, for which we provide reasonable defaults, or queries system appropriate to each compiler.

## Demo Kernel (essentially mimicing FMM operations)

- Single GPU N-body kernel.
- Understand how to call asynchronously using streams.
- Understand how to wrap batched BLAS calls in both HIP and CUDA.
- Figure out how to distribute over multiple GPUs on a given node of a HPC system.
- Figure out how to distribute heterogenous tasks over multiple GPU nodes.
- Profiling code.
