extern crate libc;

use libc::c_char;

#[allow(non_snake_case)]
#[cfg_attr(target_os = "windows", link(name = "libopenblas64_"))]
#[cfg_attr(target_os = "linux", link(name = "openblas64_"))]
#[cfg_attr(target_os = "macos", link(name = "openblas64_"))]
extern "C" {
    /// [DGEMV](http://www.netlib.org/lapack/explore-html/dc/da8/dgemv_8f.html)
    ///  performs one of the matrix-vector operations
    /// `y := alpha*A*x + beta*y`   or   `y := alpha*A**T*x + beta*y`
    /// where `alpha` and `beta` are scalars, `x` and `y` are vectors and `A`
    /// is an `m` by `n` matrix.
    #[cfg_attr(target_os = "windows", link_name = "dgemv64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgemv_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgemv_64_")]
    pub fn dgemv(
        TRANS: *const c_char,
        M: *const i64,
        N: *const i64,
        ALPHA: *const f64,
        A: *const f64,
        LDA: *const i64,
        X: *const f64,
        INCX: *const i64,
        BETA: *const f64,
        Y: *mut f64,
        INCY: *const i64,
    );

    /// [DGEMM](http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html)
    /// performs one of the matrix-matrix operations
    ///
    /// `C := alpha*op( A )*op( B ) + beta*C`,
    ///
    /// where  `op( X )` is one of
    ///
    /// `op( X ) = X`   or   `op( X ) = X**T`,
    ///
    /// `alpha` and `beta` are scalars, and `A`, `B` and `C` are matrices,
    /// with `op( A )` an `m` by `k` matrix,  `op( B )`  a  `k` by `n`
    /// matrix and `C` an `m` by `n` matrix.
    #[cfg_attr(target_os = "windows", link_name = "dgemm64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgemm_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgemm_64_")]
    pub fn dgemm(
        TRANSA: *mut c_char,
        TRANSB: *mut c_char,
        M: *mut i64,
        N: *mut i64,
        K: *mut i64,
        ALPHA: *mut f64,
        A: *mut f64,
        LDA: *mut i64,
        B: *mut f64,
        LDB: *mut i64,
        BETA: *mut f64,
        C: *mut f64,
        LDC: *mut i64,
    );

    /// [DGESV](http://www.netlib.org/lapack/explore-html/d8/d72/dgesv_8f.html)
    /// computes the solution to system of linear equations `A * X = B` for
    /// GE matrices
    #[cfg_attr(target_os = "windows", link_name = "dgesv64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgesv_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgesv_64_")]
    pub fn dgesv(
        N: *const i64,
        NRHS: *const i64,
        A: *mut f64,
        LDA: *const i64,
        IPIV: *mut i64,
        B: *mut f64,
        LDB: *const i64,
        INFO: *mut i64,
    );

    /// [DGETRF](http://www.netlib.org/lapack/explore-html/d3/d6a/dgetrf_8f.html)
    /// computes an LU factorization of a general M-by-N matrix A using partial
    /// pivoting with row interchanges.
    #[cfg_attr(target_os = "windows", link_name = "dgetrf64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgetrf_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgetrf_64_")]
    pub fn dgetrf(
        M: *const i64,
        N: *const i64,
        A: *mut f64,
        LDA: *const i64,
        IPIV: *mut i64,
        INFO: *mut i64,
    );

    /// [DGETRI](http://www.netlib.org/lapack/explore-html/df/da4/dgetri_8f.html)
    /// DGETRI computes the inverse of a matrix using the LU factorization
    /// computed by DGETRF.
    #[cfg_attr(target_os = "windows", link_name = "dgetri64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgetri_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgetri_64_")]
    pub fn dgetri(
        N: *mut i64,
        A: *mut f64,
        LDA: *mut i64,
        IPIV: *mut i64,
        WORK: *mut f64,
        LWORK: *mut i64,
        INFO: *mut i64,
    );

    /// [DGETRS](http://www.netlib.org/lapack/explore-html/d6/d49/dgetrs_8f.html)
    /// DGETRS solves a system of linear equations `A * X = B`  or  `A**T * X = B`
    /// with a general N-by-N matrix A using the LU factorization computed
    /// by DGETRF.
    #[cfg_attr(target_os = "windows", link_name = "dgetrs64_")]
    #[cfg_attr(target_os = "linux", link_name = "dgetrs_64_")]
    #[cfg_attr(target_os = "macos", link_name = "dgetrs_64_")]
    pub fn dgetrs(
        TRANS: *const c_char,
        N: *const i64,
        NRHS: *const i64,
        A: *const f64,
        LDA: *const i64,
        IPIV: *const i64,
        B: *mut f64,
        LDB: *const i64,
        INFO: *mut i64,
    );
}
