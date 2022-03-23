package org.openlca.nativelib;

import static org.junit.Assert.*;
import org.junit.Test;

public class IndexTest {

	@Test
	public void testLoadFromClassPath() {
		var index = Index.fromClassPath();
		assertFalse(index.isEmpty());
		assertTrue(index.modules().contains(Module.BLAS));
		assertTrue(index.modules().contains(Module.UMFPACK));
		assertFalse(index.libraries().isEmpty());
	}

	@Test
	public void testLoadFromDir() {
		var libDir = Tests.getLibDir();
		NativeLib.reloadFrom(libDir);
		var index = Index.fromFolder(libDir);
		assertFalse(index.isEmpty());
		assertTrue(index.modules().contains(Module.BLAS));
		assertFalse(index.libraries().isEmpty());
	}

}
