android-memeater
================

Make low memory environment for debugging APK.

Usage
-----

```bash
# push executable binary to android.
adb push android-memeater /data/local/tmp

# run.
adb shell /data/local/tmp/android-memeater
```

### Options ###

```
$ ./android-memeater --help
Usage: android-memeater [OPTIONS]

Options:
  -i, --interval <INTERVAL>
          Interval to allocate memory (s|ms) [default: 1s]
  -s, --score <SCORE>
          Score for low memory killer [default: 50]
      --initial-bytes <INITIAL_BYTES>
          Allocate bytes for initial (GiB|GB|MiB|MB|KiB|KB|B) [default: 100MiB]
      --interval-bytes <INTERVAL_BYTES>
          Allocate bytes for every interval (GiB|GB|MiB|MB|KiB|KB|B) [default: 10MiB]
  -h, --help
          Print help
```

Build
-----

```bash
make setup-ndk setup-rust-target

make release
```

LICENSE
-------

```
   Copyright 2024 sukawasatoru

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```
