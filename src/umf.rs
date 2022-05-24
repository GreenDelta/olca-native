#[cfg(umfpack)]
use std::ffi::c_void;

#[cfg(umfpack)]
#[allow(non_snake_case)]
#[cfg_attr(target_os = "windows", link(name = "libumfpack"))]
#[cfg_attr(target_os = "linux", link(name = "umfpack"))]
#[cfg_attr(target_os = "macos", link(name = "umfpack"))]
extern "C" {
    pub fn umfpack_di_symbolic(
        n_row: i32,
        n_col: i32,
        Ap: *const i32,
        Ai: *const i32,
        Ax: *const f64,
        Symbolic: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_numeric(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *const f64,
        Symbolic: *mut c_void,
        Numeric: *mut *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_solve(
        sys: i32,
        Ap: *const i32,
        Ai: *const i32,
        Ax: *const f64,
        X: *mut f64,
        B: *const f64,
        Numeric: *mut c_void,
        Control: *mut f64,
        Info: *mut f64,
    ) -> i32;

    pub fn umfpack_di_free_symbolic(Symbolic: *mut *mut c_void);

    pub fn umfpack_di_free_numeric(Numeric: *mut *mut c_void);
}
