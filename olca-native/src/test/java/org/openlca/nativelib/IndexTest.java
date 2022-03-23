package org.openlca.nativelib;

import static org.junit.Assert.*;
import org.junit.Test;

public class IndexTest {

	@Test
	public void testLoadFromClassPath() {
		var index = Index.fromClassPath();
		assertFalse(index.isEmpty());
		assertTrue(index.modules().contains(Module.BLAS));
		assertFalse(index.libraries().isEmpty());
	}

}
