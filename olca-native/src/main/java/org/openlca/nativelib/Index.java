package org.openlca.nativelib;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.function.Consumer;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import org.slf4j.LoggerFactory;

record Index(List<Module> modules, List<String> libraries) {

	static final String NAME = "olca-native.json";

	static Index empty() {
		return new Index(
			Collections.emptyList(),
			Collections.emptyList());
	}

	boolean isEmpty() {
		return modules.isEmpty();
	}

	static Index fromClassPath() {
		var stream = Index.class.getResourceAsStream(NAME);
		if (stream == null)
			return empty();
		try (var in = stream) {
			return parse(in);
		} catch (Exception e) {
			var log = LoggerFactory.getLogger(Index.class);
			log.error("failed to read index from classpath: " + NAME, e);
			return empty();
		}
	}

	static Index fromFolder(File root) {
		var file = new File(NativeLib.libFolderOf(root), NAME);
		if (!file.exists())
			return empty();
		try (var stream = new FileInputStream(file)) {
			return parse(stream);
		} catch (Exception e) {
			var log = LoggerFactory.getLogger(Index.class);
			log.error("failed to read index file: " + file, e);
			return empty();
		}
	}

	private static Index parse(InputStream stream) {
		try (var reader = new InputStreamReader(stream, StandardCharsets.UTF_8)) {
			var obj = new Gson().fromJson(reader, JsonObject.class);

			var mods = new ArrayList<Module>();
			each(obj, "modules", name -> {
				var mod = Module.fromString(name);
				if (mod != null) {
					mods.add(mod);
				}
			});

			var libs = new ArrayList<String>();
			each(obj, "libraries", libs::add);
			return new Index(mods, libs);

		} catch (Exception e) {
			var log = LoggerFactory.getLogger(Index.class);
			log.error("failed to read index file", e);
			return empty();
		}
	}

	private static void each(JsonObject obj, String prop, Consumer<String> fn) {
		if (obj == null)
			return;
		var elem = obj.get(prop);
		if (elem == null || !elem.isJsonArray())
			return;
		var array = elem.getAsJsonArray();
		for (var e : array) {
			if (e == null || !e.isJsonPrimitive())
				continue;
			fn.accept(e.getAsString());
		}
	}

	void extractFromClassPathTo(File root) throws IOException {
		var dir = NativeLib.libFolderOf(root);
		if (!dir.exists()) {
			Files.createDirectories(dir.toPath());
		}
		extractResource(NAME, dir);
		for (var lib : libraries) {
			extractResource(lib, dir);
		}
	}

	private void extractResource(String name, File dir) throws IOException {
		var in = Objects.requireNonNull(getClass().getResourceAsStream(name));
		var file = new File(dir, name);
		if (file.exists())
			return;
		var log = LoggerFactory.getLogger(getClass());
		log.info("extract file {}", file);
		Files.copy(in, file.toPath());
	}

}
