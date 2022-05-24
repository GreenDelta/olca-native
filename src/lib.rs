extern crate jni_sys;
extern crate libc;

use std::ffi::c_void;
use std::ptr;

use jni_sys::*;
use libc::{c_char, free, malloc, memcpy};

mod blas;
mod umf;

const NULL: *mut u8 = ptr::null_mut();

/// Get the raw pointer of the given array from the JVM.
unsafe fn get_array_f64(env: *mut JNIEnv, array: jdoubleArray) -> *mut f64 {
    return (**env).GetDoubleArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
unsafe fn release_array_f64(
    env: *mut JNIEnv,
    array: jdoubleArray,
    ptr: *mut f64,
) {
    (**env).ReleaseDoubleArrayElements.unwrap()(env, array, ptr, 0);
}

/// Get the raw pointer of the given array from the JVM.
#[cfg(umfpack)]
unsafe fn get_array_i32(env: *mut JNIEnv, array: jintArray) -> *mut i32 {
    return (**env).GetIntArrayElements.unwrap()(env, array, NULL);
}

/// Give the data behind the raw pointer of the given array back to the JVM.
#[cfg(umfpack)]
unsafe fn release_array_i32(env: *mut JNIEnv, array: jintArray, ptr: *mut i32) {
    (**env).ReleaseIntArrayElements.unwrap()(env, array, ptr, 0);
}

/// Performs a dense matrix-vector multiplication of a `m` by `n` matrix `A`
/// with a vector `x`: `y = A * x`.
///
/// * `m` - The number of rows of the matrix `A`
/// * `n` - The number of columns of the matrix `A`
/// * `A` - The matrix `A` stored in column-major order in an array.
/// * `x` - The vector `x` as an array of size `n`.
/// * `y` - The result vector `y` as an initialized array of size `m`.
///
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_mvmult(
    env: *mut JNIEnv,
    _class: jclass,
    m: jint,
    n: jint,
    A: jdoubleArray,
    x: jdoubleArray,
    y: jdoubleArray,
) {
    unsafe {
        let aPtr = get_array_f64(env, A);
        let xPtr = get_array_f64(env, x);
        let yPtr = get_array_f64(env, y);

        let trans = 'N' as c_char;
        let alpha: f64 = 1.0;
        let beta: f64 = 0.0;
        let inc: i64 = 1;
        let rowsA_64: i64 = m as i64;
        let colsA_64: i64 = n as i64;

        blas::dgemv(
            &trans, &rowsA_64, &colsA_64, &alpha, aPtr, &rowsA_64, xPtr, &inc,
            &beta, yPtr, &inc,
        );

        release_array_f64(env, A, aPtr);
        release_array_f64(env, x, xPtr);
        release_array_f64(env, y, yPtr);
    }
}

/// Performs a dense matrix-matrix multiplication: `C := A * B`
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_mmult(
    env: *mut JNIEnv,
    _class: jclass,
    rowsA: jint,
    colsB: jint,
    k: jint,
    A: jdoubleArray,
    B: jdoubleArray,
    C: jdoubleArray,
) {
    unsafe {
        let ptrA = get_array_f64(env, A);
        let ptrB = get_array_f64(env, B);
        let ptrC = get_array_f64(env, C);

        let mut trans = 'N' as c_char;
        let mut alpha: f64 = 1.0;
        let mut beta: f64 = 0.0;
        let mut rowsA_64 = rowsA as i64;
        let mut colsB_64 = colsB as i64;
        let mut k_64 = k as i64;

        blas::dgemm(
            &mut trans,
            &mut trans,
            &mut rowsA_64,
            &mut colsB_64,
            &mut k_64,
            &mut alpha,
            ptrA,
            &mut rowsA_64,
            ptrB,
            &mut k_64,
            &mut beta,
            ptrC,
            &mut rowsA_64,
        );

        release_array_f64(env, A, ptrA);
        release_array_f64(env, B, ptrB);
        release_array_f64(env, C, ptrC);
    }
}

/// Solves a system of linear equations `A * X = B` based on dense matrices.
///
/// This function calls the `DGESV` routine of the underlying BLAS library.
///
/// * `n`: The number of rows and columns of the square matrix `A`.
/// * `nrhs`: The number of `right hand sides`, i.e. the number of columns of
///   matrix `B`.
/// * `matrix`: On entry, the matrix `A` on the left side of the equation. After
///   the call, it will contain the LU factorization of `A`.
/// * `b`: On entry, the right hand side of the equation and after the call, it
///   will contain the solution `X`.
#[no_mangle]
pub extern "C" fn solve_dense(
    n: i64,
    nrhs: i64,
    matrix: *mut f64,
    b: *mut f64,
) -> i64 {
    let mut info: i64 = 0;
    unsafe {
        let ipiv = malloc((8 * n) as usize) as *mut i64;
        blas::dgesv(&n, &nrhs, matrix, &n, ipiv, b, &n, &mut info);
        free(ipiv as *mut c_void);
    }
    return info;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solve(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    nrhs: jint,
    matrix: jdoubleArray,
    b: jdoubleArray,
) -> jint {
    unsafe {
        let matrix_ptr = get_array_f64(env, matrix);
        let b_ptr = get_array_f64(env, b);
        let info = solve_dense(n as i64, nrhs as i64, matrix_ptr, b_ptr);
        release_array_f64(env, matrix, matrix_ptr);
        release_array_f64(env, b, b_ptr);
        return info as jint;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_invert(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    A: jdoubleArray,
) -> jint {
    unsafe {
        let mut n_64 = n as i64;
        let ptrA = get_array_f64(env, A);
        let ipiv = malloc((8 * n) as usize) as *mut i64;
        let mut info: i64 = 0;

        // calculate the factorization
        blas::dgetrf(&n_64, &n_64, ptrA, &n_64, ipiv, &mut info);

        if info != 0 {
            // factorization error
            free(ipiv as *mut c_void);
            release_array_f64(env, A, ptrA);
            return info as jint;
        }

        let mut lwork = (64 * 2 * n) as i64;
        let work = malloc((8 * lwork) as usize) as *mut f64;

        // invert it
        blas::dgetri(
            &mut n_64, ptrA, &mut n_64, ipiv, work, &mut lwork, &mut info,
        );

        free(ipiv as *mut c_void);
        free(work as *mut c_void);
        release_array_f64(env, A, ptrA);

        return info as jint;
    }
}

struct DenseFactorization {
    n: i64,
    matrix: *mut f64,
    pivot_indices: *mut i64,
}

impl Drop for DenseFactorization {
    fn drop(&mut self) {
        unsafe {
            free(self.matrix as *mut c_void);
            free(self.pivot_indices as *mut c_void);
        }
    }
}

/// Computes the LU factorization of the given dense matrix. The given matrix
/// is not modified. This function returns a pointer to the calculated
/// factorization.
#[no_mangle]
pub extern "C" fn create_dense_factorization(
    n: i64,
    matrix: *const f64,
) -> i64 {
    unsafe {
        let byte_count = (n * n * 8) as usize;
        let factorization = malloc(byte_count) as *mut f64;
        memcpy(
            factorization as *mut c_void,
            matrix as *const c_void,
            byte_count,
        );
        let pivot_indices = malloc((8 * n) as usize) as *mut i64;
        let mut info = 0i64;
        blas::dgetrf(&n, &n, factorization, &n, pivot_indices, &mut info);

        let ptr = Box::into_raw(Box::new(DenseFactorization {
            n,
            matrix: factorization,
            pivot_indices,
        }));
        return ptr as i64;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_createDenseFactorization(
    env: *mut JNIEnv,
    __class: jclass,
    n: jint,
    matrix: jdoubleArray,
) -> jlong {
    unsafe {
        let matrix_ptr = get_array_f64(env, matrix);
        let factorization_ptr =
            create_dense_factorization(n as i64, matrix_ptr);
        release_array_f64(env, matrix, matrix_ptr);
        return factorization_ptr;
    }
}

#[no_mangle]
pub extern "C" fn solve_dense_factorization(
    factorization: i64,
    columns: i64,
    b: *mut f64,
) {
    unsafe {
        let f = factorization as *const DenseFactorization;
        let n = (*f).n;
        let mut info: i64 = 0;
        blas::dgetrs(
            &('N' as c_char),
            &n,
            &columns,
            (*f).matrix,
            &n,
            (*f).pivot_indices,
            b,
            &n,
            &mut info,
        );
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solveDenseFactorization(
    env: *mut JNIEnv,
    __class: jclass,
    factorization: jlong,
    columns: jint,
    b: jdoubleArray,
) {
    unsafe {
        let b_ptr = get_array_f64(env, b);
        solve_dense_factorization(factorization, columns as i64, b_ptr);
        release_array_f64(env, b, b_ptr);
    }
}

#[no_mangle]
pub extern "C" fn destroy_dense_factorization(ptr: i64) {
    unsafe {
        let p = ptr as *mut DenseFactorization;
        let f = Box::from_raw(p);
        drop(f);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_destroyDenseFactorization(
    _env: *mut JNIEnv,
    __class: jclass,
    factorization: jlong,
) {
    destroy_dense_factorization(factorization);
}

#[no_mangle]
#[cfg(umfpack)]
pub extern "C" fn solve_sparse(
    n: i32,
    column_pointers: *const i32,
    row_indices: *const i32,
    values: *const f64,
    b: *const f64,
    x: *mut f64,
) {
    unsafe {
        let mut symbolic = ptr::null_mut();
        let mut numeric = ptr::null_mut();
        let null = ptr::null_mut() as *mut f64;

        umf::umfpack_di_symbolic(
            n,
            n,
            column_pointers,
            row_indices,
            values,
            &mut symbolic,
            null,
            null,
        );

        umf::umfpack_di_numeric(
            column_pointers,
            row_indices,
            values,
            symbolic,
            &mut numeric,
            null,
            null,
        );

        umf::umfpack_di_free_symbolic(&mut symbolic);

        umf::umfpack_di_solve(
            0,
            column_pointers,
            row_indices,
            values,
            x,
            b,
            numeric,
            null,
            null,
        );
        umf::umfpack_di_free_numeric(&mut numeric);
    }
}

#[deprecated = "Use *solveSparse instead"]
#[no_mangle]
#[cfg(umfpack)]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_umfSolve(
    env: *mut JNIEnv,
    class: jclass,
    n: jint,
    column_pointers: jintArray,
    row_indices: jintArray,
    values: jdoubleArray,
    b: jdoubleArray,
    x: jdoubleArray,
) {
    Java_org_openlca_julia_Julia_solveSparse(
        env,
        class,
        n,
        column_pointers,
        row_indices,
        values,
        b,
        x,
    );
}

#[no_mangle]
#[cfg(umfpack)]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solveSparse(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    column_pointers: jintArray,
    row_indices: jintArray,
    values: jdoubleArray,
    b: jdoubleArray,
    x: jdoubleArray,
) {
    unsafe {
        let column_pointers_ptr = get_array_i32(env, column_pointers);
        let row_indices_ptr = get_array_i32(env, row_indices);
        let values_ptr = get_array_f64(env, values);
        let b_ptr = get_array_f64(env, b);
        let x_ptr = get_array_f64(env, x);

        solve_sparse(
            n,
            column_pointers_ptr,
            row_indices_ptr,
            values_ptr,
            b_ptr,
            x_ptr,
        );

        release_array_i32(env, column_pointers, column_pointers_ptr);
        release_array_i32(env, row_indices, row_indices_ptr);
        release_array_f64(env, values, values_ptr);
        release_array_f64(env, b, b_ptr);
        release_array_f64(env, x, x_ptr);
    }
}

#[cfg(umfpack)]
struct SparseFactorization {
    column_pointers: *mut i32,
    row_indices: *mut i32,
    values: *mut f64,
    numeric: *mut c_void,
}

#[cfg(umfpack)]
impl Drop for SparseFactorization {
    fn drop(&mut self) {
        unsafe {
            umf::umfpack_di_free_numeric(&mut self.numeric);
            free(self.column_pointers as *mut c_void);
            free(self.row_indices as *mut c_void);
            free(self.values as *mut c_void);
        }
    }
}

#[no_mangle]
#[cfg(umfpack)]
pub extern "C" fn create_sparse_factorization(
    n: i32,
    column_pointers: *const i32,
    row_indices: *const i32,
    values: *const f64,
) -> i64 {
    unsafe {
        // the column pointers must contain n + 1 values
        // the last value must contain the number of
        // non-zero entries
        let non_zeros = (*(column_pointers.offset(n as isize))) as usize;

        // allocate the factorization and initialize it
        let mut f = SparseFactorization {
            column_pointers: malloc(((n + 1) * 4) as usize) as *mut i32,
            row_indices: malloc((non_zeros * 4) as usize) as *mut i32,
            values: malloc((non_zeros * 8) as usize) as *mut f64,
            numeric: ptr::null_mut(),
        };
        memcpy(
            f.column_pointers as *mut c_void,
            column_pointers as *mut c_void,
            ((n + 1) * 4) as usize,
        );
        memcpy(
            f.row_indices as *mut c_void,
            row_indices as *mut c_void,
            (non_zeros * 4) as usize,
        );
        memcpy(
            f.values as *mut c_void,
            values as *mut c_void,
            (non_zeros * 8) as usize,
        );

        // calculate the sparse factorization
        let mut symbolic = ptr::null_mut();
        let null = ptr::null_mut() as *mut f64;
        umf::umfpack_di_symbolic(
            n,
            n,
            f.column_pointers,
            f.row_indices,
            f.values,
            &mut symbolic,
            null,
            null,
        );
        umf::umfpack_di_numeric(
            f.column_pointers,
            f.row_indices,
            f.values,
            symbolic,
            &mut f.numeric,
            null,
            null,
        );
        umf::umfpack_di_free_symbolic(&mut symbolic);

        // wrap it into a pointer
        let pointer = Box::into_raw(Box::new(f));
        return pointer as i64;
    }
}

#[no_mangle]
#[cfg(umfpack)]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_createSparseFactorization(
    env: *mut JNIEnv,
    _class: jclass,
    n: jint,
    column_pointers: jintArray,
    row_indices: jintArray,
    values: jdoubleArray,
) -> jlong {
    unsafe {
        let column_pointers_ptr = get_array_i32(env, column_pointers);
        let row_indices_ptr = get_array_i32(env, row_indices);
        let values_ptr = get_array_f64(env, values);
        let pointer = create_sparse_factorization(
            n,
            column_pointers_ptr,
            row_indices_ptr,
            values_ptr,
        );
        release_array_i32(env, column_pointers, column_pointers_ptr);
        release_array_i32(env, row_indices, row_indices_ptr);
        release_array_f64(env, values, values_ptr);
        return pointer;
    }
}

#[no_mangle]
#[cfg(umfpack)]
pub extern "C" fn solve_sparse_factorization(
    factorization: i64,
    b: *const f64,
    x: *mut f64,
) {
    unsafe {
        let f = factorization as *mut SparseFactorization;
        let null = ptr::null_mut() as *mut f64;
        umf::umfpack_di_solve(
            0,
            (*f).column_pointers,
            (*f).row_indices,
            (*f).values,
            x,
            b,
            (*f).numeric,
            null,
            null,
        );
    }
}

#[no_mangle]
#[cfg(umfpack)]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_solveSparseFactorization(
    env: *mut JNIEnv,
    _class: jclass,
    factorization: jlong,
    b: jdoubleArray,
    x: jdoubleArray,
) {
    unsafe {
        let b_ptr = get_array_f64(env, b);
        let x_ptr = get_array_f64(env, x);
        solve_sparse_factorization(factorization, b_ptr, x_ptr);
        release_array_f64(env, b, b_ptr);
        release_array_f64(env, x, x_ptr);
    }
}

#[no_mangle]
#[cfg(umfpack)]
pub extern "C" fn destroy_sparse_factorization(ptr: i64) {
    unsafe {
        let p = ptr as *mut SparseFactorization;
        let f = Box::from_raw(p);
        drop(f);
    }
}

#[no_mangle]
#[cfg(umfpack)]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_openlca_julia_Julia_destroySparseFactorization(
    _env: *mut JNIEnv,
    _class: jclass,
    factorization: jlong,
) {
    destroy_sparse_factorization(factorization);
}
