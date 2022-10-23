# lazy-sfinder
Some of us don't want to memorise all 24 minimal solutions to whatever opener is the current craze. Something that people tend to skim over is that most of the solutions contribute very little to the overall solve %. You might think that you'd be able to just drop the few lowest % solutions. However, due to the high amount of overlap between solutions, this might not actually get you the best coverage.

This is in fact a variant of the 'set cover problem' which is, alas, NP-hard. I have implemented a greedy algorithm so that we can get answers without our CPUs combusting. 

The program will give you a list of fumens for each number of solutions up to and including the number you give it.

## Installation

You can grab the binaries from the release, in which case you need only put the unzipped folder into your sfinder folder. You will need to have a vaguely recent (2015+) [C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) installed. 

This will only work on Windows. But sfinder also only works for Windows, so it's unclear what you're doing here if you're not on windows. 

If you're keen, you can build the source code yourself with Cargo.

## Usage

Run [sfinder](https://github.com/knewjade/sfinder-cpp) with the 'solution' key and 'csv' format, i.e

``java -jar sfinder.jar path --tetfu <fumen> --patterns <queue> --clear-line <L> -k solution -f csv``

Then enter the lazy-sfinder folder, which you put in your sfinder folder, and run it.

```
cd lazy-sfinder
lazy-sfinder.exe
```

The default number of solutions is set to 10. Use -n to change that.

``lazy-sfinder.exe -n 20``





