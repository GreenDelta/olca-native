# olca-native

This project packages the native calculation libraries as Maven modules for
[openLCA](https://github.com/GreenDelta/olca-app). It also provides some
utility functions for loading these libraries on different platforms. Currently,
these libraries come in two facets: BLAS/LAPACK only and UMFPACK (which contains
BLAS/LAPACK). With UMFPACK you can solve large sparse systems very efficiently
in openLCA. However, UMFPACK is distributed under the GPL v2/3 which is not
compatible with the openLCA application. This is why the UMFPACK libraries
are not included by default in openLCA but can be added by the users.

# Usage

In order to use these calculation libraries you need to add the respective
version of your platform to the classpath, e.g. on Windows:

```xml
<dependency>
    <groupId>org.openlca</groupId>
    <artifactId>olca-native</artifactId>
    <version>{version}</version>
</dependency>
<dependency>
  <groupId>org.openlca</groupId>
  <artifactId>olca-native-umfpack-win-x64</artifactId>
  <version>{version}</version>
</dependency>
```

You can then load the libraries from you file system in the following way:

```java
File dir = ...;
NativeLib.loadFrom(dir);
```

This will try to load the libraries for your platform from the given folder.
If there are no libraries at this location yet, it will try to extract them
from the classpath.
