package org.openlca.nativelib;

import static org.junit.Assert.*;
import static org.junit.Assume.*;

import org.junit.BeforeClass;
import org.junit.Test;

public class NativeLibTest {

	@BeforeClass
	public static void setup() {
		NativeLib.loadFrom(Tests.getLibDir());
	}

	@Test
	public void testIsLibraryDir() {
		assumeTrue(NativeLib.isLoaded());
		assertTrue(NativeLib.isLibraryDir(Tests.getLibDir()));
	}

	@Test
	public void testIsBlasLoaded() {
		assumeTrue(NativeLib.isLoaded());
		assumeTrue(NativeLib.isLoaded(Module.BLAS));
	}

}
