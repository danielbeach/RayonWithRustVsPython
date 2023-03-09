### Rayon in Rust vs. Python Pools (Threads and Processes)

The idea behind this codebase and blog, found here,
was to understand how easy it is to use Rayon in Rust to parallelize data processing
in `Rust` vs `Python`.

Performance is a thought, but it's more about how easy it is to 
accomplish in both languages.

### The data problem.
We used a sample file of divy bike trips data set, a file with 5+ million records
that we converted from CSV to TAB delimited. This is what we test and write
code against.

#### Python
In the folder `PythonPools` you will see a single file that 
can be swapped back and forth between `ProcessPoolExecutor` and 
`ThreadPoolExecutor`.

#### Rust
The `Rust` code uses a single file found in `src/main.rs` that can be swapped
back and forth between Rayon and non-Rayon simply by changing
the `iter()` and `par_iter()` calls.

