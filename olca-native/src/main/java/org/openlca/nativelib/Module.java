package org.openlca.nativelib;

public enum Module {

	BLAS("blas"),

	UMFPACK("umfpack");

	private final String name;

	Module(String name) {
		this.name = name;
	}

	static Module fromString(String name) {
		for (var mod : Module.values()) {
			if (mod.name.equalsIgnoreCase(name))
				return mod;
		}
		return null;
	}

	@Override
	public String toString() {
		return name;
	}
}
