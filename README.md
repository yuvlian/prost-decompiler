# Prost Decompiler
Turn back prost output files to .proto

Usage: `python decompiler.py`

NOTE: You may need to modify the code a little, if the prost output has custom derive macros. This code assumes the prost output was built with default configuration.

Features (Anything that isn't here is probably not supported, like tonic gRPC, etc.):
- Proto3
- Capturing oneofs
- Capturing enums
- Capturing messages

Not tested yet:
- Nested messages, e.g.
  ```
    message Hello {
        message Hi {
            string world = 1;
        }
    }
  ```
