package org.openlca.nativelib;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.julia.Julia;

import static org.junit.Assert.assertArrayEquals;

public class BlasTest {

	@BeforeClass
	public static void setup() {
		NativeLib.loadFrom(Tests.getLibDir());
	}

	@Test
	public void testMatrixMatrixMult() {
		double[] a = {1, 4, 2, 5, 3, 6};
		double[] b = {7, 8, 9, 10, 11, 12};
		double[] c = new double[4];
		Julia.mmult(2, 2, 3, a, b, c);
		assertArrayEquals(new double[]{50, 122, 68, 167}, c, 1e-16);
	}

	@Test
	public void testMatrixVectorMult() {
		double[] a = {1, 4, 2, 5, 3, 6};
		double[] x = {2, 1, 0.5};
		double[] y = new double[2];
		Julia.mvmult(2, 3, a, x, y);
		assertArrayEquals(new double[]{5.5, 16}, y, 1e-16);
	}
}
