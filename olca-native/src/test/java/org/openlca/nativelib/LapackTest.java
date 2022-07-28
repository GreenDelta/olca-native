package org.openlca.nativelib;

import org.junit.BeforeClass;
import org.junit.Test;
import org.openlca.julia.Julia;

import static org.junit.Assert.*;

public class LapackTest {

	@BeforeClass
	public static void setup() {
		NativeLib.loadFrom(Tests.getLibDir());
	}

	@Test
	public void testSolve() {
		double[] a = {1, -4, 0, 2};
		double[] b = {1, 0, 0, 1};
		int info = Julia.solve(2, 2, a, b);
		assertEquals(0, info);
		assertArrayEquals(new double[]{1, 2, 0, 0.5}, b, 1e-16);
	}

	@Test
	public void testSolveSingularMatrix() {
		double[] a = {1, -1, 0, 0};
		double[] b = {1, 0};
		int info = Julia.solve(2, 1, a, b);
		assertTrue(info > 0); // info > 0 indicates that A was singular
	}

	@Test
	public void testInvert() {
		double[] a = {1, -4, 0, 2};
		Julia.invert(2, a);
		assertArrayEquals(new double[]{1, 2, 0, 0.5}, a, 1e-16);
	}

	@Test
	public void testInvertSingularMatrix() {
		double[] a = {1, -1, 0, 0};
		int info = Julia.invert(2, a);
		assertTrue(info > 0); // info > 0 indicates that A was singular
	}

}
