package org.openlca.nativelib;

import java.io.File;
import java.util.EnumSet;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

public class NativeLib {

	public static final String VERSION = "1.1.0";

	private static final EnumSet<Module> _modules = EnumSet.noneOf(Module.class);
	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	public static boolean isLoaded() {
		return _loaded.get();
	}

	public static boolean isLoaded(Module module) {
		return _modules.contains(module);
	}

	static File libFolderOf(File rootDir) {
		var arch = System.getProperty("os.arch");
		var path = String.join(File.separator, "olca-native", VERSION, arch);
		return new File(rootDir, path);
	}

}
