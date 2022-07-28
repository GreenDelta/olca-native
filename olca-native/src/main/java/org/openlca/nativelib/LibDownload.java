package org.openlca.nativelib;

import java.io.File;
import java.io.IOException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.util.zip.ZipFile;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * A utility class for downloading the native libraries of a module to a given
 * folder.
 */
public class LibDownload {

	public enum Repo {
		GITHUB, MAVEN
	}

	private final Logger log = LoggerFactory.getLogger(getClass());

	private final File targetDir;
	private final Module module;
	private final Repo repo;

	/**
	 * Creates a new download.
	 *
	 * @param repo      the repository from which the libraries should be
	 *                  downloaded
	 * @param module    the module for which the libraries should be downloaded
	 * @param targetDir the target directory where the libraries should be
	 *                  extracted to
	 */
	public LibDownload(Repo repo, Module module, File targetDir) {
		this.targetDir = targetDir;
		this.module = module;
		this.repo = repo;
	}

	public static void fetch(Repo repo, Module module, File targetDir) {
		try {
			new LibDownload(repo, module, targetDir).run();
		} catch (Exception e) {
			throw new RuntimeException("Download failed", e);
		}
	}

	public void run() throws Exception {

		try {
			var dir = NativeLib.libFolderOf(targetDir);
			if (!dir.exists()) {
				Files.createDirectories(dir.toPath());
			}

			var path = getUrl();
			log.info("fetch jar from {}", path);
			URL url = new URL(path);

			// download the jar/zip into a temporary file
			var zip = Files.createTempFile(
				"olca-native-" + NativeLib.VERSION + "-", ".zip");
			log.info("download jar to {}", zip);
			try (var in = url.openStream()) {
				Files.copy(in, zip, StandardCopyOption.REPLACE_EXISTING);
			}

			// extract and delete the jar/zip
			extractJar(dir, zip.toFile());
			Files.delete(zip);

		} catch (Exception e) {
			log.error("failed to download and extract native libraries", e);
			throw e;
		}
	}

	private String getUrl() {
		var baseName = "olca-native-" + module.toString() + "-"
			+ NativeLib.os() + "-" + NativeLib.arch();
		if (repo == Repo.MAVEN) {
			return "https://repo1.maven.org/maven2/org/openlca/"
				+ baseName + "/" + NativeLib.VERSION + "/" + baseName
				+ "-" + NativeLib.VERSION + ".jar";
		} else {
			return "https://github.com/GreenDelta/olca-native/releases/download/v"
				+ NativeLib.VERSION + "/" + baseName + ".zip";
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

