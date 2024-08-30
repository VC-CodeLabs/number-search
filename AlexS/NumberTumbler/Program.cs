/*
 * Number Tumbler!
 * This solution marches input files via an allocated buffer, using some potentially unsafe char conversions.
 * It should theoretically be able to handle very large files, with lines of arbitrary length.
 *
 * Program can be passed in optional file parameters:
 * > NumberTumbler.exe [input path] [output path]
 *
 * e.g. NumberTumbler.exe ./sample.txt ./output/123.txt
 */
using System.Text;

namespace NumberTumbler
{
    /// <summary>
    /// Main program.
    /// </summary>
    internal class Program
    {
        /// <summary>
        /// Runs the program.
        /// </summary>
        /// <param name="args">File arguments: [input] [output]</param>
        static void Main(string[] args)
        {
            //Standard file paths, if unspecified below
            string inputFilePath = "./input.txt";
            string outputFilePath = "./output.txt";

            //Read in file arguments, if passed
            if (args.Length > 0)
                inputFilePath = args[0];
            if (args.Length > 1)
                outputFilePath = args[1];

            //Pass any file etc. exceptions to standard output
            try
            {
                //Open file streams, disposing at end of block w/ using statement
                using FileStream inputFile = new(inputFilePath, FileMode.Open);
                using FileStream outputFile = new(outputFilePath, FileMode.Create);

                //Output number trackers (two per line - first and last number)
                int? num1 = null;
                int? num2 = null;

                //Allocate input buffer and begin reading input file
                byte[] buffer = new byte[32 * 1024]; //32 Kb buffer
                int ReadNext() => inputFile.Read(buffer, 0, buffer.Length);
                for (int bytesRead = ReadNext(); bytesRead > 0; bytesRead = ReadNext())
                {
                    //For each byte we've read...
                    for (int i = 0; i < bytesRead; i++)
                    {
                        //Convert to character from unicode (or smaller)
                        string c = char.ConvertFromUtf32(buffer[i]);

                        //Is it a number?
                        if (int.TryParse(c, out int n))
                        {
                            if (num1 == null) //First number?
                                num1 = num2 = n; //Cool, set both ("a3e" should print "33")
                            else
                                num2 = n; //Just set 2nd (end) number
                        }

                        //Is it a line ending?
                        if (c == "\n")
                        {
                            outputFile.Write(Encoding.UTF8.GetBytes($"{num1}{num2}\n")); //Write it!
                            num1 = num2 = null; //Reset
                        }
                    }
                }

                //Write the output file
                outputFile.Flush();
                Console.WriteLine($"Wrote {outputFile.Name} ({outputFile.Length} bytes)");
            }
            catch (Exception ex)
            {
                //Oh no! Just log error message
                Console.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
