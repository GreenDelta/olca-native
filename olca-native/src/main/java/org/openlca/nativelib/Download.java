package org.openlca.nativelib;

import java.io.File;
import java.io.IOException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.zip.ZipFile;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

record Download(File targetDir, Module module, Logger log) {

	Download(File target, Module module) {
		this(target, module, LoggerFactory.getLogger(Download.class));
	}

	void run() throws Exception {

		var dir = NativeLib.libFolderOf(targetDir);
		if (!dir.exists()) {
			Files.createDirectories(dir.toPath());
		}

		var id = "olca-native-" + module.toString() + "-"
			+ NativeLib.os() + "-" + NativeLib.arch();
		var path = "https://repo1.maven.org/maven2/org/openlca/" + id + "/"
			+ NativeLib.VERSION + "/" + id + "-" + NativeLib.VERSION + ".jar";

		log.info("fetch jar from {}", path);
		try {
			URL url = new URL(path);

			// download the jar into a temporary file
			var jar = Files.createTempFile(
				"olca-native-" + NativeLib.VERSION + "-", ".jar");
			log.info("download jar to {}", jar);
			try (var in = url.openStream()) {
				Files.copy(in, jar, StandardCopyOption.REPLACE_EXISTING);
			}

			// extract and delete the jar
			extractJar(dir, jar.toFile());
			Files.delete(jar);

		} catch (Exception e) {
			log.error("failed to download native libraries from " + path, e);
			throw e;
		}
	}

	private void extractJar(File dir, File jar) throws IOException {
		try (var zip = new ZipFile(jar)) {
			var entries = zip.entries();
			while (entries.hasMoreElements()) {
				var e = entries.nextElement();
				if (e.isDirectory())
					continue;
				var target = new File(dir, e.getName());
				// we do this to skip the sub-directories
				target = new File(dir, target.getName());
				try (var in = zip.getInputStream(e)) {
					Files.copy(in, target.toPath(),
						StandardCopyOption.REPLACE_EXISTING);
				}
			}
		}
	}
}

