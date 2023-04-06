package nl.mrfriendly.tracing;

public class TracingSlf4jImpl {
    // Function registered from native code.
    // Constants are also defined in native code.

    public static final int ERROR = 0;
    public static final int WARN = 1;
    public static final int INFO = 2;
    public static final int DEBUG = 3;
    public static final int TRACE = 4;

    public static native void tracingSlf4jImpl(int level, String string);
}
