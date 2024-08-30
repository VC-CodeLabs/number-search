# Number Tumbler!

Implementation for the "Number Search" challenge in C#.

### Setup
Requires .NET 8.0: https://dotnet.microsoft.com/en-us/download/dotnet/8.0
Tested on Visual Studio 2022

Input/output can be set in [NumberTumbler/Program.cs](NumberTumbler/Program.cs) via provided variables in the `Main` function, or specified at runtime. ([see below](#run))

### Build

Build the [NumberTumbler project](NumberTumbler/) via Visual Studio. The "Release" configuration will [perform slightly faster](https://learn.microsoft.com/en-us/visualstudio/debugger/how-to-set-debug-and-release-configurations?view=vs-2022).

Or, from this directory:
```pwsh
dotnet build --configuration Release
```

### Run

After building, simply run:
```pwsh
.\NumberTumbler\bin\Release\net8.0\NumberTumbler.exe [input file] [output file]

# [input file] - input file path (optional, default "./input.txt")
# [output file] - output file path (optional, default "./output.txt")
```

By default, the program will read `input.txt` from the program directory, and output `output.txt` to the same directory, if these arguments are unspecified.

Enjoy!
