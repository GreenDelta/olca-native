package org.openlca.nativelib;

import java.io.File;

class Tests {

	private Tests() {
	}

	public static File getLibDir() {
		var home = new File(System.getProperty("user.home"));
		return new File(home, "openLCA-data-1.4");
	}
}
