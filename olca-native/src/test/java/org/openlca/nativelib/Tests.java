package org.openlca.nativelib;

import java.io.File;

class Tests {

	private Tests() {
	}

	static void delete(File f) {
		if (f == null || !f.exists())
			return;
		try {
			if (f.isDirectory()) {
				var files = f.listFiles();
				if (files != null) {
					for (var fi : files) {
						delete(fi);
					}
				}
			}
			if (!f.delete()) {
				f.deleteOnExit();
			}
		} catch (Exception e) {
			throw new RuntimeException(e);
		}
	}
}
