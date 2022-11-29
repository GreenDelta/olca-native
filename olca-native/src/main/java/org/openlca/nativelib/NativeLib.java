package org.openlca.nativelib;

import java.io.File;
import java.util.EnumSet;
import java.util.concurrent.atomic.AtomicBoolean;

import org.slf4j.LoggerFactory;

public class NativeLib {

	public static final String VERSION = "0.0.1";

	private static final EnumSet<Module> _modules = EnumSet.noneOf(Module.class);
	private static final AtomicBoolean _loaded = new AtomicBoolean(false);

	/**
	 * Returns {@code true} if native libraries are loaded.
	 */
	public static boolean isLoaded() {
		return _loaded.get() && !_modules.isEmpty();
	}

	/**
	 * Returns {@code true} if the given module is available and loaded.
	 */
	public static boolean isLoaded(Module module) {
		return isLoaded() && _modules.contains(module);
	}

	/**
	 * Returns {@code true} if the given folder contains a native library package
	 * for the current platform.
	 */
	public static boolean isLibraryDir(File root) {
		if (root == null || !root.exists())
			return false;
		var index = Index.fromFolder(root);
		return !index.isEmpty();
	}

	public static void reloadFrom(File root) {
		synchronized (_loaded) {
			_loaded.set(false);
			_modules.clear();
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
			var dir = storageLocationIn(root);
			log.info("load native libraries from {}", dir);
			for (var lib : idx.libraries()) {
				var libFile = new File(dir, lib);
				try {
					System.load(libFile.getAbsolutePath());
					log.trace("loaded library {}", libFile);
				} catch (Throwable e) {
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

	/**
	 * Returns the storage location of the native libraries and their meta-data
	 * files within the given directory (the root folder).
	 */
	public static File storageLocationIn(File root) {
		var path = String.join(File.separator, "olca-native", VERSION, arch());
		return new File(root, path);
	}

	static String arch() {
		var arch = System.getProperty("os.arch");
		if (arch == null)
			return "x64";
		var lower = arch.trim().toLowerCase();
		return lower.startsWith("aarch") || lower.startsWith("arm")
			? "arm64"
			: "x64";
	}

	static String os() {
		var os = System.getProperty("os.name");
		if (os == null)
			return "win";
		var lower = os.trim().toLowerCase();
		if (lower.startsWith("linux"))
			return "linux";
		if (lower.startsWith("mac"))
			return "macos";
		return "win";
	}
}
