I need to start keeping track of these 

TODO get exact minimum versions required

- RustRover
- Slang
- Up-to-date graphics drivers
- Up-to-date Vulkan SDK (or just SPIR-V if you can figure it out)
- rust toolchain


Huge pages must be supported and enabled on your OS.
- On Linux, you might need to configure huge pages via /proc/sys/vm/nr_hugepages (e.g., sudo sysctl -w vm.nr_hugepages=10).
- On Windows:
  - Win + R
  - secpol.msc
  - Left Pane -> Security Settings -> Local Policy -> User Rights Assignment
  - Edit "Lock pages in memory"
  - Add User or Group
  - Put your username (e.g. the last part of your home folder at C:\Users\username)
  - Click OK, Apply, and Ok