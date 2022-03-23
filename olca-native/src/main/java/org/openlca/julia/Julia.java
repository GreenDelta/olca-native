package org.openlca.julia;

public class Julia {

	// BLAS

	/**
	 * Matrix-matrix multiplication: C := A * B
	 *
	 * @param rowsA [in] number of rows of matrix A
	 * @param colsB [in] number of columns of matrix B
	 * @param k     [in] number of columns of matrix A and number of rows of matrix
	 *              B
	 * @param a     [in] matrix A (size = rowsA*k)
	 * @param b     [in] matrix B (size = k * colsB)
	 * @param c     [out] matrix C (size = rowsA * colsB)
	 */
	public static native void mmult(int rowsA, int colsB, int k,
		double[] a, double[] b, double[] c);

	/**
	 * Matrix-vector multiplication: y:= A * x
	 *
	 * @param rowsA [in] rows of matrix A
	 * @param colsA [in] columns of matrix A
	 * @param a     [in] the matrix A
	 * @param x     [in] the vector x
	 * @param y     [out] the resulting vector y
	 */
	public static native void mvmult(int rowsA, int colsA,
		double[] a, double[] x, double[] y);

	// LAPACK

	/**
	 * Solves a system of linear equations A * X = B for general matrices. It calls
	 * the LAPACK DGESV routine.
	 *
	 * @param n    [in] the dimension of the matrix A (n = rows = columns of A)
	 * @param nrhs [in] the number of columns of the matrix B
	 * @param a    [io] on entry the matrix A, on exit the LU factorization of A
	 *             (size = n * n)
	 * @param b    [io] on entry the matrix B, on exit the solution of the equation
	 *             (size = n * bColums)
	 * @return the LAPACK return code
	 */
	public static native int solve(int n, int nrhs, double[] a, double[] b);

	/**
	 * Inverts the given matrix.
	 *
	 * @param n [in] the dimension of the matrix (n = rows = columns)
	 * @param a [io] on entry: the matrix to be inverted, on exit: the inverse (size
	 *          = n * n)
	 * @return the LAPACK return code
	 */
	public static native int invert(int n, double[] a);

	// UMFPACK
	public static native void umfSolve(
		int n,
		int[] columnPointers,
		int[] rowIndices,
		double[] values,
		double[] demand,
		double[] result);

	public static native long umfFactorize(
		int n,
		int[] columnPointers,
		int[] rowIndices,
		double[] values);

	public static native void umfDispose(long pointer);

	public static native long umfSolveFactorized(
		long pointer,
		double[] demand,
		double[] result);


	public static native long createDenseFactorization(
		int n,
		double[] matrix);

	public static native void solveDenseFactorization(
		long factorization,
		int columns,
		double[] b);

	public static native void destroyDenseFactorization(
		long factorization);

	public static native long createSparseFactorization(
		int n,
		int[] columnPointers,
		int[] rowIndices,
		double[] values);

	public static native void solveSparseFactorization(
		long factorization,
		double[] b,
		double[] x);

	public static native void destroySparseFactorization(
		long factorization);
}
