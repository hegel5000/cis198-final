CIS198 Final Project Proposal: Scientific Rust

Summary:
 * Python + the library SciPy see widespread use in the scientific community.  While Python’s REPL is a nice environment for interactive use of a library, it’s not necessarily the most performant or easiest to use option for building a larger application.  SciPy’s C backend, while performant, is even harder to use in production code than SciPy’s Python API.  Our alternative is to write (the beginnings of) a Rust backend to SciPy, which should be easier to extend or work directly with than the existing C version.

Approximate Time Spent: 
 * Parker: 13 hours
 * Ian: 13 hours

Accomplishments:
 * We learned a significant amount about writing code to be used through an FFI.  More importantly, we have a small handful of standard vector operations implemented, as well as the essential feature of an ODE solver.
 * Components, structure, design decisions
 * Our operations and ODE solver work on Rust Vecs.
 * Unlike the Numeric Rust library’s Tensor type which takes ownership of its first Tensor argument and performs the operation in-place, our implementation takes references to both arguments and allocates a new vector.  We felt that a little array copying wouldn’t be a big deal performance-wise, and that without Rust’s ownership system, a Python user might accidentally re-use a value which has been modified in place. 
 * We have FFIable wrappers which are monomorphized to work on f64 values, and which return const* pointers to the output vectors.
 * We do not expose the internals of those Vecs to the Python side.
 * Instead, provide a printing and parsing functions which go between pointers to character arrays and pointers to Rust vectors.
 * We made the package into a Python package which can be imported into other Python projects, and installed like any other Python module.
 * We provide a little Python script which loads up libscirust.so as a Python object.

Testing approach and results
 * Rust unit tests were written for RK4 and the vector utility functions (can be run with Cargo test)
 * Rust unit tests pass.
 * Rust FFT function was run, and an assert!(false) test was used to view the result.
 * FFI testing was done with IPython (see below for results).

Benchmarks
 * Unit testing ensures that the RK4 implementation solves simple ODEs (run “cargo test”).

Limitations
 * Rust code passed unit tests, so the Rust backend works
 * FFI can take integer and string-based arguments and return integer-based arguments with no problem
 * We can have Python send in list-like arguments, but we can’t return array-like values from Rust into Python, due to pointer problems.
 * The only scientific algorithms that got implemented were RK4 and Discrete FFT; FFI with array-like data was hard.

Postmortem:

What went well:
 * We implemented RK4 and Discrete FFT.
 * We were able to successfully call Rust functions in Python.
 * We were also able to send Python arguments (integer-like, string-like, and list-like) into Rust functions.  Much of our FFI followed this guide: http://jakegoulding.com/rust-ffi-omnibus/
 * Rust unit testing worked, so all of our Rust code was good.
 * We successfully got some bindings to work for Numeric types into Python.
 * We made the directory into a Python module that can be installed and imported like any standard Python package.

What we would do differently:
 * We would also need to find a way to send Rust array-like things into Python (attempts at this were unsuccessful).
 * Have a module written such that the Python user never has to worry about Rust internals.
 * I (Parker) would not have used Ubuntu as my only Linux distro to test Python with, as there is a libhdf5 dependency that stable Ubuntu does not satisfy properly (hdf5 is a numeric dependency).  This made the Python end difficult to debug.  EDIT: when numeric was removed as a dependency, libhdf5 was removed as a dependency, so the code could compile on my machine.  This is still worth dealing with in the future.
 * Have some performance tests to compare to C-code (ran out of time to do this).
 * We would note the fact that many Scipy-routines are Fortran based, so we shouldn’t expect C-like code to beat them in terms of performance (the Rust-based routines should far outclass them in readability and debugability though :) )

