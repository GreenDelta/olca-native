package org.openlca.nativelib;

import java.nio.file.Files;

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

	@Test
	public void testLoadFromDir() throws Exception {
		var tempDir = Files.createTempDirectory("_olca_").toFile();
		NativeLib.reloadFrom(tempDir);
		var index = Index.fromFolder(tempDir);
		assertFalse(index.isEmpty());
		assertTrue(index.modules().contains(Module.BLAS));
		assertFalse(index.libraries().isEmpty());
		Tests.delete(tempDir);
	}

}
