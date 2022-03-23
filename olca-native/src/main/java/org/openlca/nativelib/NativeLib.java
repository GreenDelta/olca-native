package org.openlca.nativelib;

import java.io.File;
import java.util.EnumSet;
import java.util.concurrent.atomic.AtomicBoolean;

import org.slf4j.LoggerFactory;

public class NativeLib {

	public static final String VERSION = "1.1.0";

	private static final EnumSet<Module> _modules = EnumSet.noneOf(Module.class);
	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	/**
	 * Returns true if the native libraries are loaded.
	 */
	public static boolean isLoaded() {
		return _loaded.get();
	}

	/**
	 * Returns true if the given module is available and loaded.
	 */
	public static boolean isLoaded(Module module) {
		return isLoaded() && _modules.contains(module);
	}

	public static void reloadFrom(File root) {
		synchronized (_loaded) {
			_loaded.set(false);
			loadFrom(root);
		}
	}

	public static void loadFrom(File root) {
		if (_loaded.get())
			return;
		synchronized (_loaded) {
			if (_loaded.get())
				return;
			var log = LoggerFactory.getLogger(NativeLib.class);
			var idx = indexDir(root);
			if (idx.isEmpty()) {
					log.warn("could not find native libraries in classpath and {}", root);
					return;
				}

			// load the libraries
			var dir = libFolderOf(root);
			log.info("load native libraries from {}", dir);
			for (var lib : idx.libraries()) {
				var libFile = new File(dir, lib);
				try {
					System.load(libFile.getAbsolutePath());
					log.trace("loaded library {}", libFile);
				} catch (Exception e) {
					log.error("failed to load library " + lib, e);
					return;
				}
			}
			_modules.addAll(idx.modules());
			_loaded.set(true);
		}
	}

	private static Index indexDir(File root) {
		var dirIdx = Index.fromFolder(root);
		if (!dirIdx.isEmpty())
			return dirIdx;
		var jarIdx = Index.fromClassPath();
		if (jarIdx.isEmpty())
			return jarIdx;
		try {
			jarIdx.extractFromClassPathTo(root);
			return jarIdx;
		} catch (Exception e) {
			var log = LoggerFactory.getLogger(NativeLib.class);
			log.error("failed to extract libraries to " + root, e);
			return Index.empty();
		}
	}

	static File libFolderOf(File rootDir) {
		var arch = System.getProperty("os.arch");
		var path = String.join(File.separator, "olca-native", VERSION, arch);
		return new File(rootDir, path);
	}
}
